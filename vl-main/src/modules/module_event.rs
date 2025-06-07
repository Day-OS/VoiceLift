use anyhow::Ok;
use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{Res, ResMut},
    },
    log::tracing::event,
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
    UpdateDeviceSelection(UpdateDeviceSelectionEvent),
}

#[derive(Debug)]
pub struct UpdateDeviceSelectionEvent {
    pub selected: bool,
    pub device_type: AudioDeviceType,
    pub name: String,
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
    if event_r.is_empty() {
        return;
    }
    let runtime = runtime.runtime();
    runtime.block_on(async {
        for event in event_r.read() {
            match event {
                ModuleEvent::LoadModule(_) => {}
                ModuleEvent::UpdateDeviceSelection(e) => {
                    handler_update_device_selection_event(
                        &mut module_manager,
                        e,
                    )
                    .await;
                }
            }
            log::info!("{event:?}")
        }
    });
}

//; System for activating repeating tasks in the background
pub fn module_manager_ticker(
    mut module_manager: ResMut<ModuleManager>,
    time: Res<Time>,
    tokio: ResMut<TokioTasksRuntime>,
    screen: ResMut<ScreenManager>,
) {
    let timer = &mut module_manager._timer;
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

pub async fn handler_update_device_selection_event(
    module_manager: &mut ResMut<'_, ModuleManager>,
    event: &UpdateDeviceSelectionEvent,
) {
    let config_arc = module_manager.config.clone();
    let mut config = config_arc.write().await;
    config.modify_and_save(|config| {
        let devices = &mut config.devices;
        let device_list = match event.device_type {
            AudioDeviceType::INPUT => &mut devices.input_devices,
            AudioDeviceType::OUTPUT => &mut devices.output_devices,
        };
        if event.selected {
            device_list.push(event.name.clone());
        } else {
            if let Some(index) = device_list
                .iter()
                .position(|name| name.clone() == event.name)
            {
                device_list.remove(index);
            }
        }
        Ok(())
    });
    module_manager.reload_config();
}
