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
use bevy_egui::input::EguiInputEvent;
use futures::executor::{self, block_on};
use std::sync::Arc;

use bevy_egui::egui::Vec2;

use crate::{
    events::screen_event::{ScreenEvent, screen_event_handler},
    manager::Manager,
    ui::screens::{Screen, ScreenError, ScreenParameters},
};

use super::virtual_keyboard::Keyboard;

#[derive(Resource)]
pub struct ScreenManager {
    pub screen_size: Vec2,
    pub(crate) screens: HashMap<String, Arc<RwLock<dyn Screen>>>,
    selected_screen: Arc<RwLock<dyn Screen>>,
    pub keyboard: Arc<RwLock<Keyboard>>,
}
impl ScreenManager {
    pub fn new(first_screen: Arc<RwLock<dyn Screen>>) -> Self {
        let mut hashmap = HashMap::new();
        Self::_add_screen(&mut hashmap, first_screen.clone());

        Self {
            screen_size: Vec2::default(),
            screens: hashmap,
            selected_screen: first_screen,
            keyboard: Arc::new(RwLock::new(Keyboard::default())),
        }
    }
    pub fn current_screen_name(&self) -> &str {
        let screen = block_on(self.selected_screen.read());
        screen.get_screen_name()
    }

    pub fn apply_screen(
        &mut self,
        screen_name: &String,
    ) -> Result<(), ScreenError> {
        let screen = self.screens.get(screen_name).ok_or(
            ScreenError::ScreenNotFound(screen_name.clone()),
        )?;
        self.selected_screen = screen.clone();
        Ok(())
    }
    pub fn add_screen(&mut self, screen: Arc<RwLock<dyn Screen>>) {
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

    pub fn is_collapsable(&self) -> bool {
        let selected_screen =
            executor::block_on(self.selected_screen.read());
        selected_screen.is_collapsable()
    }

    /// Draw the current selected screen into the EGUI Window
    pub fn draw(&mut self, mut params: ScreenParameters<'_>) {
        let mut selected_screen =
            executor::block_on(self.selected_screen.write());
        params.module_manager._throw_error_message(params.ctx);

        if let Err(e) = selected_screen.draw(params) {
            log::error!("{e}")
        }
    }

    pub fn get_size(&self) -> Vec2 {
        let selected_screen =
            executor::block_on(self.selected_screen.read());
        selected_screen.get_size()
    }
}

impl Manager for ScreenManager {
    fn modify_app(&mut self, app: &mut bevy::app::App) {
        app.add_systems(Update, keyboard_input_event);
        app.add_systems(Update, keyboard_output_event);
        app.add_systems(Update, screen_event_handler);
        app.add_event::<ScreenEvent>();
    }
}

// Keyboard
pub fn keyboard_output_event(
    screen: ResMut<ScreenManager>,
    event_w: EventWriter<EguiInputEvent>,
) {
    let mut keyboard = executor::block_on(screen.keyboard.write());
    keyboard.write_events(event_w);
}

pub fn keyboard_input_event(
    screen: ResMut<ScreenManager>,
    event_r: EventReader<EguiInputEvent>,
) {
    let mut keyboard = executor::block_on(screen.keyboard.write());
    keyboard.read_events(event_r)
}
