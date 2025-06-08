use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{Res, ResMut},
    },
    tasks::block_on,
    time::Time,
};
use bevy_tokio_tasks::TokioTasksRuntime;
use vl_global::audio_devices::AudioDeviceType;

use crate::{
    modules::{linux::tts, module_manager::ModuleManager},
    ui::{
        screen_manager::ScreenManager,
        screens::{Screen, config_screen::ConfigScreen},
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

#[derive(Debug)]
pub struct LinkAllEvent;

pub fn initialize_module_manager(
    mut module_manager: ResMut<ModuleManager>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let runtime = runtime.runtime();
    runtime.block_on(async {
        module_manager.initialize().await;

        // Enable device Module
        if let Some(device_module) =
            module_manager.selected_device_module.clone()
        {
            let mut device_module = device_module.write().await;
            if !device_module.is_started() {
                let result = device_module.start().await;

                if let Err(e) = result {
                    log::error!("{e}");
                    let error = "Device Module error".to_owned();
                    module_manager.error(error);
                }
            }
        }
        module_manager.relink_all_devices().await;

        // Enable tts Module
        if let Some(tts_module) =
            module_manager.selected_tts_module.clone()
        {
            let mut tts_module = tts_module.write().await;

            if !tts_module.is_started() {
                let result = tts_module.start().await;

                if let Err(e) = result {
                    log::error!("{e}");
                    let error = "TTS Module error".to_owned();
                    module_manager.error(error);
                }
            }
        }
    });
}

pub fn module_event_handler(
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
                    .await
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
    let result = config.modify_and_save(|config| {
        let devices = &mut config.devices;
        let device_list = match event.device_type {
            AudioDeviceType::INPUT => &mut devices.input_devices,
            AudioDeviceType::OUTPUT => &mut devices.output_devices,
        };
        if event.selected {
            device_list.push(event.name.clone());
        } else if let Some(index) = device_list
            .iter()
            .position(|name| name.clone() == event.name)
        {
            device_list.remove(index);
            block_on(
                module_manager.unlink_device(event.name.clone()),
            );
        }
        Ok(())
    });

    if let Err(e) = result {
        log::error!("{e}");
        return;
    }

    module_manager.reload_config();
    module_manager.relink_all_devices().await;
}
