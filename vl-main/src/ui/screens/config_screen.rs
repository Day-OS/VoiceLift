use core::f32;
use std::default;
use std::time::Duration;

use bevy::app::Main;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Button;
use bevy_egui::egui::Color32;
use bevy_egui::egui::FontId;
use bevy_egui::egui::Vec2;
use egui_taffy::taffy::prelude::auto;
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::prelude::percent;
use egui_taffy::{TuiBuilderLogic, taffy, tui};

use crate::base_managers::ModuleManager;
use crate::ui::virtual_keyboard::Keyboard;

use super::Screen;
use super::ScreenEvent;

#[derive(Default)]
pub struct ConfigScreen {}

impl Screen for ConfigScreen {
    fn get_size(&self) -> Vec2 {
        Vec2::new(800., 500.)
    }
    fn uses_keyboard(&self) -> bool {
        false
    }
    fn draw(
        &mut self,
        module_manager: &mut ResMut<'_, ModuleManager>,
        screen_event_w: &mut EventWriter<ScreenEvent>,
        ui: &mut egui::Ui,
        _ctx: &mut egui::Context,
        work_area: Vec2,
    ) {
    }
}
