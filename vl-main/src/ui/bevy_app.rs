use std::sync::Arc;

#[cfg(target_os = "android")]
use crate::android::keyboard::show_soft_input;
use crate::{
    events::{
        module_event::{
            ModuleEvent, initialize_module_manager,
            module_event_handler, module_manager_ticker,
        },
        screen_event::ScreenEvent,
    },
    manager::Manager,
    modules::module_manager::ModuleManager,
    ui::{screen_manager::ScreenManager, screens::ScreenParameters},
};

use async_lock::RwLock;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    render::{
        RenderApp,
        batching::gpu_preprocessing::{
            GpuPreprocessingMode, GpuPreprocessingSupport,
        },
    },
    window::{CursorOptions, WindowMode, WindowResolution},
};
use bevy_egui::{
    EguiContextPass, EguiContexts, EguiPlugin,
    egui::{self, Color32, CornerRadius, Frame, Margin, Vec2},
};
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};

use super::screens::{
    config_screen::ConfigScreen, main_screen::MainScreen,
};

pub fn run() {
    let mut app: App = App::new();
    app.insert_resource(ClearColor(Color::NONE));
    let main_screen = MainScreen::default();
    let mut module_manager = ModuleManager::new();
    module_manager.modify_app(&mut app);
    app.insert_resource(module_manager);
    let mut screen_manager =
        ScreenManager::new(Arc::new(RwLock::new(main_screen)));
    screen_manager
        .add_screen(Arc::new(RwLock::new(ConfigScreen::default())));
    screen_manager.modify_app(&mut app);
    app.insert_resource(screen_manager);
    app.add_event::<ModuleEvent>();
    app.add_systems(Startup, initialize_module_manager);
    app.add_systems(
        Update,
        (module_event_handler, module_manager_ticker),
    );
    app.add_plugins(TokioTasksPlugin::default());
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                // This will show some log events from Bevy to the native logger.
                level: Level::DEBUG,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=trace"
                    .to_string(),
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    cursor_options: CursorOptions {
                        // Allow inputs to pass through to apps behind this app.
                        hit_test: false,
                        ..default()
                    },
                    window_level:
                        bevy::window::WindowLevel::AlwaysOnTop,
                    transparent: true,
                    decorations: false,
                    resizable: false,
                    resolution: WindowResolution::new(100., 100.),
                    fullsize_content_view: false,
                    position: WindowPosition::Centered(
                        MonitorSelection::Current,
                    ),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
    )
    // Make the winit loop wait more aggressively when no user input is received
    // This can help reduce cpu usage on mobile devices
    .add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_systems(EguiContextPass, egui_screen);
    // .add_systems(Update, keyboard_test)
    // .insert_resource(WinitSettings::mobile())
    // .add_event::<VirtualKeyboardEvent>()
    app.sub_app_mut(RenderApp).insert_resource(
        GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::None,
        },
    );
    app.run();
}

fn egui_screen(
    mut contexts: EguiContexts,
    module_manager: ResMut<ModuleManager>,
    mut screen: ResMut<ScreenManager>,
    mut window: Single<&mut Window>,
    screen_event_w: EventWriter<ScreenEvent>,
    module_event_w: EventWriter<ModuleEvent>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    // window.mode =
    // WindowMode::BorderlessFullscreen(MonitorSelection::Current);

    let ctx = contexts.ctx_mut();

    let mut screen_size = screen.get_size();
    egui_material_icons::initialize(ctx);
    let stroke: f32 = 6.;
    let margin: i8 = 10;
    screen_size = screen_size
        - Vec2 {
            x: stroke * 2.,
            y: stroke * 2.,
        }
        - Vec2 {
            x: (margin * 2) as f32,
            y: (margin * 2) as f32,
        };
    let window_response = egui::Window::new("MainScreen")
        .anchor(egui::Align2::LEFT_TOP, bevy_egui::egui::Vec2::ZERO)
        .frame(Frame {
            fill: Color32::from_rgb(32, 32, 32),
            inner_margin: Margin {
                left: margin,
                right: margin,
                top: margin,
                bottom: margin,
            },
            stroke: egui::Stroke::new(
                stroke,
                Color32::from_rgb(0, 0, 0),
            ),
            corner_radius: CornerRadius::ZERO,
            outer_margin: Margin::ZERO,
            shadow: egui::Shadow::NONE,
        })
        .default_rect(ctx.screen_rect())
        // .fixed_size(Vec2 { x: 10., y: 10. })
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(&ctx.clone(), |ui| {
            let params = ScreenParameters {
                ctx,
                module_event_w,
                module_manager,
                screen_event_w,
                ui,
                keyboard: screen.keyboard.clone(),
                runtime,
                work_area: screen_size,
            };
            screen.draw(params);
        });
    let mut new_resolution = Vec2::ZERO;
    if screen.is_collapsable() {
        if let Some(inner) = window_response {
            let window_rect = inner.response.rect;
            new_resolution = window_rect.size();
        }
    } else {
        new_resolution = screen.get_size();
    }
    window.resolution.set(new_resolution.x, new_resolution.y);
    window.position.center(MonitorSelection::Current);
}
