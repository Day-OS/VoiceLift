use bevy_egui::egui;
use bevy_egui::egui::TextEdit;
use bevy_egui::egui::Vec2;
use egui_taffy::taffy::prelude::auto;
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::prelude::percent;
use egui_taffy::{TuiBuilderLogic, taffy, tui};

use crate::ui::virtual_keyboard::Keyboard;

use super::Screen;

#[derive(Default)]
pub struct MainScreen {
    text: String,
}

impl Screen for MainScreen {
    fn get_size(&self) -> Vec2 {
        Vec2::new(500., 500.)
    }
    fn uses_keyboard(&self) -> bool {
        true
    }
    fn draw_with_keyboard(
        &mut self,
        ui: &mut egui::Ui,
        _ctx: &mut egui::Context,
        keyboard: &mut Keyboard,
        work_area: Vec2,
    ) {
        let style = ui.style_mut();
        if let Some(style) =
            style.text_styles.get_mut(&egui::TextStyle::Body)
        {
            style.size = 18.0;
        }
        let mut work_area = work_area;
        work_area.y = 0.;

        tui(ui, ui.id().with("demo"))
            .reserve_space(work_area)
            .style(taffy::Style {
                flex_direction: taffy::FlexDirection::Column,
                align_items: Some(taffy::AlignItems::Stretch),
                size: taffy::Size {
                    width: percent(1.),
                    height: auto(),
                },
                padding: length(0.),
                gap: length(0.),
                flex_grow: 1.,
                justify_content: Some(taffy::AlignContent::Center),
                ..Default::default()
            })
            .show(|tui| {
                tui.ui(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add(TextEdit::multiline(&mut self.text));
                    });
                    keyboard.base.show(ui)
                })
            });
    }
}
