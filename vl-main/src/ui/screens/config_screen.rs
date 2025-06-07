
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Color32;
use bevy_egui::egui::RichText;
use bevy_egui::egui::Vec2;
use busrt::ipc::Config;
use egui_taffy::taffy::prelude::auto;
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::prelude::percent;
use egui_taffy::taffy::AlignItems;
use egui_taffy::taffy::Style;
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use futures::executor;
use tokio::runtime;
use vl_global::audio_devices::AudioDeviceStatus;
use vl_global::audio_devices::AudioDevices;
use egui_extras::{Column, TableBuilder};
use crate::modules::module_event::ModuleEvent;
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
    ) {
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
                let mut file_dialog = executor::block_on(file_dialog.write());
                let config_clone = &mut module_manager.config.clone();
                let mut config = executor::block_on(config_clone.write());
                config
                    .modify_and_save(
                        |config: &mut vl_global::vl_config::VlConfig| {
                            ui.heading("Configurações");

                            module_manager.show_configs(ui, config, &mut module_event_w, &mut tokio);

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
                                    file_dialog.pick_file();
                                }
                                if let Some(path) = file_dialog.take_picked() {
                                    let path = path.to_path_buf();
                                    if linux.validate_piper_tts_model(&path){
                                        linux.piper_tts_model = path.display().to_string();
                                    }
                                }

                                self.show_devices_widget(ui, module_manager, &mut module_event_w, config);

                                    

                                //linux.piper_tts_model;
                            }
                            Ok(())
                        },
                    ).unwrap();
                file_dialog.update(_ctx);
            });
    });
        // module_manager
    }
}

impl ConfigScreen{
    pub fn show_devices_widget(
        &self,
        ui: &mut egui::Ui,
        module_manager: ResMut<'_, ModuleManager>,
        module_event_w: &mut EventWriter<'_, ModuleEvent>,
        config: &mut vl_global::vl_config::VlConfig,
    ){
        let devices = module_manager.available_devices.clone();
        let comparison = AudioDevices::compare_lists(&devices, &config.devices);
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
                                                module_event_w.write(ModuleEvent::UpdateDeviceSelection { selected, device_type: device_type.clone(), name: device.clone() });
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

        //for (device_type, status) in comparison.0 {
            // let (id, name) = match device_type {
            //     vl_global::audio_devices::AudioDeviceType::INPUT => ("input_panel", "Dispositivos de Entrada"),
            //     vl_global::audio_devices::AudioDeviceType::OUTPUT => ("output_panel", "Dispositivos de Saída"),
            // };

            // egui::SidePanel::left(id).exact_width(100.).show_inside(ui, |ui| {
            //     egui::ScrollArea::vertical().show(ui, |ui| {
            //         let mut table = TableBuilder::new(ui)
            //             .striped(true)
            //             .resizable(false)
            //             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            //             .column(Column::auto())
            //             .column(
            //                 Column::remainder()
            //                     .at_least(40.0)
            //                     .clip(true)
            //                     .resizable(true),
            //             )
            //             .column(Column::auto())
            //             .column(Column::remainder())
            //             .column(Column::remainder())
            //             .min_scrolled_height(0.0)
            //             .max_scroll_height(available_height);

            //         table.body(|mut body| {
            //             for (status, devices) in status {
            //                 let mut devices = devices.clone();
            //                 devices.sort();
            //                 for device in devices {
            //                     body.row(18., |mut row| {
            //                         row.col(|ui| {
            //                             ui.label(":3");
            //                         });
            //                         row.col(|ui| {
            //                             ui.label(device);
            //                         });
            //                     });
            //                 }
            //             }
            //         });
            //     })
            // });
        //}
    }
}