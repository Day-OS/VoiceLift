use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{Res, ResMut},
    },
    time::Time,
};
use bevy_tokio_tasks::TokioTasksRuntime;
use vl_global::audio_devices::AudioDeviceType;

use crate::{
    modules::module_manager::ModuleManager,
    ui::screens::{
        Screen, ScreenManager, config_screen::ConfigScreen,
    },
};

#[derive(Event, Debug)]
pub enum ModuleEvent {
    LoadModule(String),
    UpdateDeviceSelection {
        selected: bool,
        device_type: AudioDeviceType,
        name: String,
    },
}

pub fn initialize_module_manager(
    mut module_manager: ResMut<ModuleManager>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let runtime = runtime.runtime();
    let result = runtime.block_on(module_manager.initialize());
    let linker = result.selected_device_module.clone().unwrap();
    let mut linker = runtime.block_on(linker.write());
    let linker_start_result = runtime.block_on(linker.start());
    if let Err(e) = linker_start_result {
        log::error!("{e}");
        let error = "Linker did not start".to_owned();
        module_manager.error(error);
    }
}

pub fn module_manager_event_handler(
    mut module_manager: ResMut<ModuleManager>,
    mut event_r: EventReader<ModuleEvent>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let runtime = runtime.runtime();
    for event in event_r.read() {
        log::info!("{event:?}")
    }
}

//; System for activating repeating tasks in the background
pub fn module_manager_ticker(
    mut module_manager: ResMut<ModuleManager>,
    time: Res<Time>,
    tokio: ResMut<TokioTasksRuntime>,
    mut screen: ResMut<ScreenManager>,
) {
    let mut timer = &mut module_manager._timer;
    timer.tick(time.delta());

    if timer.finished() {
        let runtime = tokio.runtime();
        runtime.block_on(async {
            get_available_devices(module_manager, screen).await;
        });
    }
}

async fn get_available_devices(
    mut module_manager: ResMut<'_, ModuleManager>,
    screen: ResMut<'_, ScreenManager>,
) {
    if screen.current_screen_name() != ConfigScreen::get_name() {
        return;
    }
    let selected_device_module =
        module_manager.selected_device_module.clone();
    if selected_device_module.is_none() {
        return;
    }
    let module = selected_device_module.unwrap();
    let module = module.write().await;

    if !module.is_started() {
        return;
    }

    let devices = module.get_devices().await;
    if let Err(e) = devices {
        log::error!("{e}");
        return;
    }
    let devices = devices.unwrap();

    module_manager.available_devices = devices;
}
