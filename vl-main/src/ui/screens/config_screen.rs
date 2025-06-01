
use bevy_egui::egui::Vec2;
use egui_taffy::taffy::prelude::auto;
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::prelude::percent;
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use futures::executor;

use crate::ui::screens::ScreenParameters;

use super::Screen;
use super::ScreenEvent;
use super::main_screen::MainScreen;

#[derive(Default)]
pub struct ConfigScreen {
}


impl Screen for ConfigScreen {
    fn get_size(&self) -> Vec2 {
        Vec2::new(800., 500.)
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

                                //linux.piper_tts_model;
                            }
                        },
                    )
                    .unwrap();
                file_dialog.update(_ctx);
            });
    });
        // module_manager
    }
}
