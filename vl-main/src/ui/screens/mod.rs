use std::{intrinsics::type_name, sync::Arc};
pub mod config_screen;
pub mod main_screen;
use async_lock::RwLock;
use bevy::{
    app::Update,
    ecs::{
        event::{Event, EventReader, EventWriter},
        resource::Resource,
        system::ResMut,
    },
    platform::collections::HashMap,
};
use bevy_egui::{egui, input::EguiInputEvent};
use bevy_tokio_tasks::TokioTasksRuntime;
use futures::executor;

use bevy_egui::egui::Vec2;

use crate::base_modules::module_manager::{
    ModuleManager, ModuleManagerEvent,
};

use super::virtual_keyboard::Keyboard;

pub struct ScreenParameters<'w> {
    pub module_manager: ResMut<'w, ModuleManager>,
    pub screen_event_w: EventWriter<'w, ScreenEvent>,
    pub module_event_w: EventWriter<'w, ModuleManagerEvent>,
    pub ui: &'w mut egui::Ui,
    pub ctx: &'w mut egui::Context,
    pub work_area: Vec2,
    pub keyboard: Arc<RwLock<Keyboard>>,
    pub runtime: ResMut<'w, TokioTasksRuntime>,
}

#[derive(thiserror::Error, Debug)]
pub enum ScreenError {
    #[error("Screen {0} was not found.")]
    ScreenNotFound(String),
}

#[derive(Event)]
pub enum ScreenEvent {
    ScreenChangeEvent {
        /// The screen that was ordered to be changed to
        screen_name: String,
    },
}

pub trait Screen: Sync + Send {
    fn uses_keyboard(&self) -> bool {
        false
    }
    fn is_collapsable(&self) -> bool {
        true
    }
    fn draw(&mut self, params: ScreenParameters);
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

#[derive(Resource)]
pub struct ScreenManager {
    pub screen_size: Vec2,
    screens: HashMap<String, Arc<RwLock<dyn Screen>>>,
    pub selected_screen: Arc<RwLock<dyn Screen>>,
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

    fn get(&self) -> Arc<RwLock<dyn Screen>> {
        self.selected_screen.clone()
    }

    pub fn register_systems(&mut self, app: &mut bevy::app::App) {
        app.add_systems(Update, keyboard_input_event);
        app.add_systems(Update, keyboard_output_event);
        app.add_systems(Update, screen_event_handler);
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

        selected_screen.draw(params);
    }

    pub fn get_size(&self) -> Vec2 {
        let selected_screen =
            executor::block_on(self.selected_screen.read());
        selected_screen.get_size()
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

pub fn screen_event_handler(
    mut manager: ResMut<ScreenManager>,
    mut event_r: EventReader<ScreenEvent>,
) {
    for event in event_r.read() {
        match event {
            ScreenEvent::ScreenChangeEvent { screen_name } => {
                if let Err(e) = manager.apply_screen(screen_name) {
                    log::error!(
                        "Failed to change screen: {e}. Available screens are: {:?}",
                        manager.screens.keys()
                    );
                    continue;
                }
                log::debug!("Changed screen to {screen_name}");
            }
        }
    }
}
