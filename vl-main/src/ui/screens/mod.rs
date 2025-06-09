use std::{intrinsics::type_name, sync::Arc};
pub mod config_screen;
pub mod main_screen;
use async_lock::RwLock;
use bevy::{
    app::AppExit,
    ecs::{
        event::EventWriter,
        system::{Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
};
use bevy_egui::egui;
use bevy_tokio_tasks::TokioTasksRuntime;

use bevy_egui::egui::Vec2;

use crate::{
    events::{module_event::ModuleEvent, screen_event::ScreenEvent},
    modules::module_manager::ModuleManager,
};

use super::virtual_keyboard::Keyboard;

pub struct ScreenParameters<'w> {
    pub module_manager: ResMut<'w, ModuleManager>,
    pub screen_event_w: EventWriter<'w, ScreenEvent>,
    pub module_event_w: EventWriter<'w, ModuleEvent>,
    pub ui: &'w mut egui::Ui,
    pub ctx: &'w mut egui::Context,
    pub work_area: Vec2,
    pub keyboard: Arc<RwLock<Keyboard>>,
    pub runtime: ResMut<'w, TokioTasksRuntime>,
    pub keys: Res<'w, ButtonInput<KeyCode>>,
    pub app_exit_w: EventWriter<'w, AppExit>,
}

#[derive(thiserror::Error, Debug)]
pub enum ScreenError {
    #[error("Screen {0} was not found.")]
    ScreenNotFound(String),
}

pub trait Screen: Sync + Send {
    fn uses_keyboard(&self) -> bool {
        false
    }
    fn is_collapsable(&self) -> bool {
        true
    }
    fn draw(
        &mut self,
        params: ScreenParameters,
    ) -> anyhow::Result<()>;

    fn get_screen_name(&self) -> &'static str {
        type_name::<Self>()
    }
    fn get_name() -> &'static str
    where
        Self: Sized,
    {
        type_name::<Self>()
    }
    fn get_size(&self) -> Vec2;
}
