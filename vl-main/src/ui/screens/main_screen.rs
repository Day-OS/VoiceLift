use core::f32;
use std::sync::Arc;

use crate::modules::base::tts_module::TtsModule;
use crate::modules::module_manager::ModuleManager;
use crate::ui::screens::ScreenParameters;
use crate::ui::virtual_keyboard::Keyboard;
use async_lock::RwLock;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Button;
use bevy_egui::egui::Color32;
use bevy_egui::egui::FontId;
use bevy_egui::egui::Vec2;
use bevy_tokio_tasks::TokioTasksRuntime;
use egui_taffy::taffy::prelude::auto;
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::prelude::percent;
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use futures::executor;
use vl_global::vl_config::ConfigManager;

use super::Screen;
use super::ScreenEvent;
use super::config_screen::ConfigScreen;

#[derive(Default)]
pub struct MainScreen {
    text: String,
    suggestion_text: Option<String>,
    keyboard_enabled: bool,
}

impl MainScreen {
    fn show_suggestion_text(
        &self,
        ui: &mut egui::Ui,
        text_output: egui::text_edit::TextEditOutput,
        font_size: f32,
    ) {
        if let Some(text) = &self.suggestion_text {
            let painter = ui.painter_at(text_output.response.rect);
            let text_color =
                Color32::from_rgba_premultiplied(100, 100, 100, 100);
            let font = FontId {
                size: font_size,
                ..Default::default()
            };
            let galley = painter.layout(
                String::from(text),
                font,
                text_color,
                f32::INFINITY,
            );
            painter.galley(
                text_output.galley_pos,
                galley,
                text_color,
            );
        }
    }

    fn show_keyboard(
        &self,
        tui: &mut egui_taffy::Tui,
        keyboard: &mut Keyboard,
    ) {
        if self.keyboard_enabled {
            tui.ui(|ui| keyboard.base.show(ui));
        }
    }

    fn show_run_button(
        &self,
        tui: &mut egui_taffy::Tui,
        button_width: f32,
        tokio: &ResMut<TokioTasksRuntime>,
        module_manager: &ResMut<ModuleManager>,
    ) {
        tui.style(taffy::Style {
            flex_direction: taffy::FlexDirection::Row,
            padding: length(0.),
            gap: length(0.),
            flex_grow: 1.,
            ..Default::default()
        })
        .add(|tui| {
            tui.ui(|ui| {
                let button = Button::new(
                    egui_material_icons::icons::ICON_VOLUME_UP,
                )
                .corner_radius(0);
                if ui
                    .add_sized(
                        [button_width, ui.available_height()],
                        button,
                    )
                    .clicked()
                {
                    async fn speak(
                        module: Arc<RwLock<dyn TtsModule>>,
                        text: String,
                        config: Arc<RwLock<ConfigManager>>,
                    ) {
                        let module = module.read().await;
                        if let Err(e) = module.stop_speaking().await{
                            log::error!("Error while trying to stop the current audio {e}");
                        };
                        if let Err(e) = module.speak(text, config).await {
                            log::error!("Error while trying to reproduce TTS {e}");
                        }
                    }
                    let runtime = tokio.runtime();
                    if let Some(tts_module) =
                        &module_manager.selected_tts_module
                    {
                        runtime.spawn(speak(
                            tts_module.clone(),
                            self.text.clone(),
                            module_manager.config.clone(),
                        ));
                    }
                }
            })
        });
    }

    fn show_user_input(
        &mut self,
        tui: &mut egui_taffy::Tui,
        font_size: f32,
    ) {
        tui.style(taffy::Style {
            flex_direction: taffy::FlexDirection::Column,
            align_self: Some(taffy::AlignItems::Stretch),
            justify_self: Some(taffy::AlignItems::Stretch),
            padding: length(0.),
            gap: length(0.),
            flex_grow: 6.,
            margin: taffy::Rect {
                left: auto(),
                right: length(10.),
                top: auto(),
                bottom: auto(),
            },
            ..Default::default()
        })
        .add(|tui| {
            tui.ui(|ui| {
                let text_edit_id =
                    ui.make_persistent_id("mai_text_edit");
                ui.memory_mut(|mem| mem.request_focus(text_edit_id));

                let text_edit =
                    egui::TextEdit::multiline(&mut self.text)
                        .id(text_edit_id)
                        .desired_rows(1)
                        .desired_width(f32::INFINITY)
                        .lock_focus(true)
                        .return_key(None);
                let output: egui::text_edit::TextEditOutput =
                    text_edit.show(ui);
                self.show_suggestion_text(ui, output, font_size);
            })
        });
    }

    fn show_menu_buttons(
        &mut self,
        ui: &mut egui::Ui,
        screen_event_w: &mut EventWriter<ScreenEvent>,
    ) {
        ui.menu_button(
            egui_material_icons::icons::ICON_SETTINGS,
            |ui| {
                let keyboard = ui.button(format!(
                    "{} Ativar teclado Virtual",
                    egui_material_icons::icons::ICON_KEYBOARD
                ));
                if keyboard.clicked() {
                    self.keyboard_enabled = !self.keyboard_enabled;
                }
                let preferences = ui.button(format!(
                    "{} Preferências...",
                    egui_material_icons::icons::ICON_SETTINGS
                ));
                if preferences.clicked() {
                    screen_event_w.write(
                        ScreenEvent::ScreenChangeEvent {
                            screen_name: ConfigScreen::get_name()
                                .to_owned(),
                        },
                    );
                }
            },
        );
    }
}

impl Screen for MainScreen {
    fn get_size(&self) -> Vec2 {
        Vec2::new(500., 500.)
    }
    fn uses_keyboard(&self) -> bool {
        true
    }
    fn draw(
        &mut self,
        mut params: ScreenParameters,
    ) -> std::result::Result<(), anyhow::Error> {
        let ui = params.ui;
        let style = ui.style_mut();
        let font_size = 18.0;
        let button_width = 50.;
        if let Some(style) =
            style.text_styles.get_mut(&egui::TextStyle::Body)
        {
            style.size = font_size;
        }
        let mut work_area = params.work_area;
        work_area.y = 0.;

        self.show_menu_buttons(ui, &mut params.screen_event_w);
        let keyboard = params.keyboard.clone();

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
                tui.style(taffy::Style {
                    flex_direction: taffy::FlexDirection::Row,
                    align_self: Some(taffy::AlignItems::Stretch),
                    justify_self: Some(taffy::AlignItems::Stretch),
                    padding: length(0.),
                    gap: length(0.),
                    flex_grow: 1.,
                    size: taffy::Size {
                        width: percent(1.),
                        height: percent(1.),
                    },
                    margin: taffy::Rect {
                        left: auto(),
                        right: auto(),
                        top: auto(),
                        bottom: length(10.),
                    },
                    ..Default::default()
                })
                .add(|tui| {
                    self.show_user_input(tui, font_size);
                    self.show_run_button(
                        tui,
                        button_width,
                        &params.runtime,
                        &params.module_manager,
                    );
                });
                let mut keyboard =
                    executor::block_on(keyboard.write());
                self.show_keyboard(tui, &mut keyboard);
            });
        Ok(())
    }
}
