use std::{intrinsics::type_name, sync::Arc};
pub mod main_screen;
use async_lock::RwLock;
use bevy::{
    app::Update,
    ecs::{
        event::{EventReader, EventWriter},
        resource::Resource,
        system::ResMut,
    },
    platform::collections::HashMap,
};
use bevy_egui::{egui, input::EguiInputEvent};
use futures::executor;

use bevy_egui::egui::Vec2;

use crate::base_managers::ModuleManager;

use super::virtual_keyboard::Keyboard;

#[derive(thiserror::Error, Debug)]
pub enum ScreenError {
    #[error("Screen {0} was not found.")]
    ScreenNotFound(String),
}

pub trait Screen: Sync + Send {
    fn uses_keyboard(&self) -> bool;
    fn draw(
        &mut self,
        module_manager: &mut ResMut<'_, ModuleManager>,
        ui: &mut egui::Ui,
        ctx: &mut egui::Context,
        work_area: Vec2,
    ) {
    }
    fn draw_with_keyboard(
        &mut self,
        module_manager: &mut ResMut<'_, ModuleManager>,
        ui: &mut egui::Ui,
        ctx: &mut egui::Context,
        keyboard: &mut Keyboard,
        work_area: Vec2,
    ) {
    }
    fn get_screen_name(&self) -> &'static str {
        type_name::<Self>()
    }
    fn get_size(&self) -> Vec2;
}

#[derive(Resource)]
pub struct ScreenManager {
    pub screen_size: Vec2,
    screens: HashMap<String, Arc<RwLock<dyn Screen>>>,
    pub selected_screen: Arc<RwLock<dyn Screen>>,
    pub keyboard: Keyboard,
}
impl ScreenManager {
    pub fn new(first_screen: Arc<RwLock<dyn Screen>>) -> Self {
        let mut hashmap = HashMap::new();
        Self::_add_screen(&mut hashmap, first_screen.clone());

        Self {
            screen_size: Vec2::default(),
            screens: hashmap,
            selected_screen: first_screen,
            keyboard: Keyboard::default(),
        }
    }
    fn apply_screen(
        &mut self,
        screen_name: &String,
    ) -> Result<(), ScreenError> {
        let screen = self.screens.get(screen_name).ok_or(
            ScreenError::ScreenNotFound(screen_name.clone()),
        )?;
        self.selected_screen = screen.clone();
        Ok(())
    }
    fn add_screen(&mut self, screen: Arc<RwLock<dyn Screen>>) {
        Self::_add_screen(&mut self.screens, screen)
    }
    fn _add_screen(
        screens: &mut HashMap<String, Arc<RwLock<dyn Screen>>>,
        target: Arc<RwLock<dyn Screen>>,
    ) {
        let screen = executor::block_on(target.read());
        let screen_name = screen.get_screen_name();
        screens.insert(screen_name.to_owned(), target.clone());
    }

    fn get(&self) -> Arc<RwLock<dyn Screen>> {
        self.selected_screen.clone()
    }

    pub fn register_systems(&mut self, app: &mut bevy::app::App) {
        app.add_systems(Update, keyboard_input_event);
        app.add_systems(Update, keyboard_output_event);
    }

    /// Draw the current selected screen into the EGUI Window
    pub fn draw(
        &mut self,
        module_manager: &mut ResMut<'_, ModuleManager>,
        ui: &mut egui::Ui,
        ctx: &mut egui::Context,
        work_area: Vec2,
    ) {
        let mut selected_screen =
            executor::block_on(self.selected_screen.write());
        if selected_screen.uses_keyboard() {
            selected_screen.draw_with_keyboard(
                module_manager,
                ui,
                ctx,
                &mut self.keyboard,
                work_area,
            );
        } else {
            selected_screen.draw(module_manager, ui, ctx, work_area);
        }
    }

    pub fn get_size(&self) -> Vec2 {
        let selected_screen =
            executor::block_on(self.selected_screen.read());
        selected_screen.get_size()
    }
}

// Keybpard
pub fn keyboard_output_event(
    mut screen: ResMut<ScreenManager>,
    event_w: EventWriter<EguiInputEvent>,
) {
    screen.keyboard.write_events(event_w);
}

pub fn keyboard_input_event(
    mut screen: ResMut<ScreenManager>,
    event_r: EventReader<EguiInputEvent>,
) {
    screen.keyboard.read_events(event_r)
}
