use bevy::ecs::{
    event::{Event, EventReader},
    system::ResMut,
};
use bevy_tokio_tasks::TokioTasksRuntime;

use crate::ui::screen_manager::ScreenManager;


#[derive(Event)]
pub enum ScreenEvent {
    ScreenChangeEvent {
        /// The screen that was ordered to be changed to
        screen_name: String,
    },
}

pub fn screen_event_handler(
    mut manager: ResMut<ScreenManager>,
    tokio: ResMut<TokioTasksRuntime>,
    mut event_r: EventReader<ScreenEvent>,
) {
    if event_r.is_empty() {
        return;
    }
    let runtime = tokio.runtime();
    runtime.block_on(async {
        
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
    });

}
