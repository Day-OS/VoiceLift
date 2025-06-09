
use std::sync::Arc;
use anyhow::Ok;
use async_lock::RwLock;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Color32;
use bevy_egui::egui::RichText;
use bevy_egui::egui::Vec2;
use egui_file_dialog::FileDialog;
use egui_taffy::taffy::prelude::auto;
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::prelude::percent;
use egui_taffy::taffy::AlignItems;
use egui_taffy::taffy::Style;
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use futures::executor;
use vl_global::audio_devices::AudioDeviceStatus;
use egui_extras::{Column, TableBuilder};
use vl_global::vl_config::VlConfig;
use crate::events::module_event::ModuleEvent;
use crate::events::module_event::UpdateDeviceSelectionEvent;
use crate::modules::base::i_module::IModule;
use crate::modules::base::module::Module;
use crate::modules::module_manager::ModuleManager;
use crate::ui::screens::ScreenParameters;

use super::Screen;
use super::ScreenEvent;
use super::main_screen::MainScreen;

#[derive(Default)]
pub struct ConfigScreen {
}


impl Screen for ConfigScreen {
    fn get_size(&self) -> Vec2 {
        Vec2::new(800., 200.)
    }
    fn uses_keyboard(&self) -> bool {
        false
    }
    fn draw(
        &mut self,
        params: ScreenParameters,
    ) -> std::result::Result<(), anyhow::Error> {
        let mut module_manager = params.module_manager;
        let mut screen_event_w = params.screen_event_w;
        let mut module_event_w = params.module_event_w;
        let ui = params.ui;
        let mut _ctx = params.ctx;
        let work_area = params.work_area;
        let mut tokio = params.runtime;


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
                tui.ui(|ui|{
                    
                let go_back = ui.button(format!(
                    "{} Voltar",
                    egui_material_icons::icons::ICON_ARROW_BACK
                ));
                if go_back.clicked() {
                    screen_event_w.write(ScreenEvent::ScreenChangeEvent {
                        screen_name: MainScreen::get_name().to_owned(),
                    });
                }
                let file_dialog = &mut module_manager.file_dialog.clone();
                let config_clone = &mut module_manager.config.clone();
                let mut config = executor::block_on(config_clone.write());
                config
                    .modify_and_save(
                        |config: &mut vl_global::vl_config::VlConfig| {
                            ui.heading("Configurações");

                            self.show_modules_widget(&mut module_manager, ui, config, &mut tokio);
                            
                            self.show_linux_tts_widget(ui, file_dialog.clone(), config);
                            
                            self.show_devices_widget(ui, &mut module_manager, &mut module_event_w);
                            Ok(())
                        },
                    ).unwrap();
                let mut file_dialog_guard = executor::block_on(file_dialog.write());
                file_dialog_guard.update(_ctx);
            });
        });
        Ok(())
    }
}

impl ConfigScreen{
    pub fn show_devices_widget(
        &self,
        ui: &mut egui::Ui,
        module_manager: &mut ResMut<'_, ModuleManager>,
        module_event_w: &mut EventWriter<'_, ModuleEvent>,
    ){
        if module_manager.available_devices.is_none(){
            return;
        }
        let comparison = module_manager.available_devices.clone().unwrap();
        tui(ui, ui.id().with("devices_panel")).reserve_available_space().style(Style{
            flex_direction: taffy::FlexDirection::Row,
            min_size: taffy::Size {
                width: auto(),
                height: auto(),
            },
            flex_grow: 1.,
            justify_items: Some(taffy::AlignItems::Stretch),
            align_items: Some(taffy::AlignItems::Stretch),
            // gap: length(8.),
            ..Default::default()
        }).show(|tui|{
            for (device_type, status) in comparison.0 {
                let str_device_type = match device_type {
                    vl_global::audio_devices::AudioDeviceType::INPUT => "Dispositivos de Entrada",
                    vl_global::audio_devices::AudioDeviceType::OUTPUT => "Dispositivos de Saída",
                };

                tui.style(
                    Style { 
                        min_size: taffy::Size {
                            width: auto(),
                            height: length(200.),
                        },
                        flex_grow: 1.,
                        align_self: Some(AlignItems::Stretch),
                        justify_self: Some(AlignItems::Stretch),
                        ..Default::default()
                    }
                ).add_with_border(|tui|{
                    tui.ui(|ui|{
                        ui.heading(str_device_type.to_string());
                        let text_height = egui::TextStyle::Body
                            .resolve(ui.style())
                            .size
                            .max(ui.spacing().interact_size.y*1.2);
                        let available_height = ui.available_height();
                        let table = TableBuilder::new(ui)
                        // .auto_shrink([false, false])
                        .max_scroll_height(f32::INFINITY)
                        .min_scrolled_height(available_height)
                        .vscroll(true)
                        .striped(true)
                        .resizable(false)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::auto())
                        .column(Column::remainder().clip(false))
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center));


                        table.body(|mut body| {
                            for (status, devices) in status {
                                let mut devices = devices.clone();
                                devices.sort();
                                for device in devices {
                                    body.row(text_height, |mut row| {
                                        let mut selected: bool = status.is_selected();
                                        
                                        row.col(|ui| {
                                            let checkbox = ui.checkbox(&mut selected, "");
                                            if checkbox.changed(){
                                                module_event_w.write(ModuleEvent::UpdateDeviceSelection(UpdateDeviceSelectionEvent{
                                                     selected, device_type: device_type.clone(), name: device.clone() 
                                                }));
                                            }
                                        });
                                        row.col(|ui| {
                                        let mut text = RichText::new(device);
                                            if status == AudioDeviceStatus::SelectedButNotAvailable {
                                                text = text.color(Color32::from_rgb(64, 64, 64));
                                            }
                                            ui.label(text);
                                        });
                                    });
                                }
                            }
                        });
                    })
                });
            }
        });
    }

    pub fn show_linux_tts_widget(
        &self,
        ui: &mut egui::Ui,
        file_dialog: Arc<RwLock<FileDialog>>,
        config: &mut vl_global::vl_config::VlConfig,
    ){
        let mut file_dialog_guard = executor::block_on(file_dialog.write());

        if let Some(linux) = &mut config.linux {
            ui.heading("Configurações do Linux Module");
            ui.label(format!("Piper TTS Model Path: {}", linux.piper_tts_model));
            if ui.button("Pick file").clicked() {
                // For some reason it is private?
                // if !linux.piper_tts_model.is_empty(){
                //     let path = Path::new(&linux.piper_tts_model);
                //     if path.exists(){
                //         file_dialog.initial_directory(path.to_path_buf());
                //     }
                // }
                file_dialog_guard.pick_file();
            }
            if let Some(path) = file_dialog_guard.take_picked() {
                let path = path.to_path_buf();
                if linux.validate_piper_tts_model(&path){
                    linux.piper_tts_model = path.display().to_string();
                }
            }
        }
    }


    /// Draw module configs, with module selection and initialization options  
    pub fn show_modules_widget(
        &mut self,
        module_manager: &mut ResMut<ModuleManager>,
        ui: &mut egui::Ui,
        config: &mut VlConfig,
        tokio: &mut ResMut<bevy_tokio_tasks::TokioTasksRuntime>,
    ) {
        let runtime = tokio.runtime();
        
        runtime.block_on(async {
            let mut tts: Vec<Module> = vec![];
            let mut audio_device: Vec<Module> = vec![];

            for module in &mut module_manager.modules{
                match module{
                    Module::TtsModule(_) => tts.push(module.clone()),
                    Module::DeviceModule(_) => audio_device.push(module.clone()),
                }
            }

            // Get TTS Module
            if let Err(e) = show_module_options_widget(
                module_manager,
                "TTS Module".to_string(),
                module_manager.selected_tts_module.clone().map(Module::from),
                tts,
                ui,
                config,
            ).await{
                log::error!("{e}");
            }


            // Get Audio Device Module
            if let Err(e) = show_module_options_widget(
                module_manager,
                "Audio Device Module".to_string(),
                module_manager.selected_device_module.clone().map(Module::from),
                audio_device,
                ui,
                config,
            ).await{
                log::error!("{e}");
            }

        });
    }
}

pub async fn show_module_options_widget(
    module_manager: &mut ResMut<'_, ModuleManager>,
    mut module_title: String,
    selected_module: Option<Module>,
    modules: Vec<Module>,
    ui: &mut egui::Ui,
    config: &mut VlConfig,
) -> anyhow::Result<()>
{   
    if selected_module.is_none(){
        return Ok(());
    }
    let mut selected_module = selected_module.unwrap();

    // Category Label
    let selected_type  = selected_module.get_module_type();
    let mut selected_name = selected_module.get_screen_name();
    module_title = format!("{module_title} Selecionado: {selected_name}");
    ui.label(module_title);

    let mut did_module_change = false;
    // Draw the options
    for module in modules {
        let text = module.get_screen_name();
        let radio = ui.radio_value(
            &mut selected_module,
            module,
            text,
        );
        if radio.clicked(){
            did_module_change = true;
            selected_name = selected_module.get_screen_name();
        }
    }

    // Update the config with the selected module
    if did_module_change{
        config
            .selected_modules
            .insert(selected_type.to_owned(), selected_name.to_owned());
        
        module_manager.reload_config();
        module_manager.update_selected_modules().await?;
    }

    // Add start modules buttons
    let button = ui.button(format!("Iniciar {selected_type}")); 
    if button.clicked() {
        if let Err(e) = selected_module.start().await {
            module_manager.error(format!(
                "Failed to start {selected_name}!",
            ));
            log::error!("{e}");
        }
    }


    Ok(())
}