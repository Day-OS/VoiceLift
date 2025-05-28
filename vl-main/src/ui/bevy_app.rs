use std::sync::Arc;

#[cfg(target_os = "android")]
use crate::android::keyboard::show_soft_input;
use crate::base_managers::{
    ModuleManager, initialize_module_manager,
};
use async_lock::{Mutex, RwLock};
use bevy::{
    ecs::entity::unique_slice::Windows,
    log::{Level, LogPlugin},
    platform::collections::HashMap,
    prelude::*,
    render::{
        RenderApp,
        batching::gpu_preprocessing::{
            GpuPreprocessingMode, GpuPreprocessingSupport,
        },
    },
    window::{
        CursorOptions, WindowMode, WindowResized, WindowResolution,
    },
};
use bevy_egui::{
    EguiContextPass, EguiContexts, EguiPlugin,
    egui::{self, Color32, CornerRadius, Frame, Margin, Vec2},
};
use bevy_tokio_tasks::TokioTasksPlugin;
use egui_notify::Toasts;
use futures::executor;
use std::time::Duration;
use vl_global::vl_config::{ConfigManager, VlConfig};

use super::screens::{
    Screen, ScreenEvent, ScreenManager, config_screen::ConfigScreen,
    main_screen::MainScreen, screen_event_handler,
};

pub fn run() {
    let app_config =
        ConfigManager::new().expect("Config should be initialized");

    let mut app: App = App::new();
    app.insert_resource(ClearColor(Color::NONE));
    let main_screen = MainScreen::default();
    let mut screen_manager =
        ScreenManager::new(Arc::new(RwLock::new(main_screen)));
    screen_manager
        .add_screen(Arc::new(RwLock::new(ConfigScreen::default())));
    screen_manager.register_systems(&mut app);
    app.insert_resource(screen_manager);
    app.insert_resource(ModuleManager::new());
    app.add_event::<ScreenEvent>();
    app.add_systems(Startup, initialize_module_manager);
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
    mut module_manager: ResMut<ModuleManager>,
    mut screen: ResMut<ScreenManager>,
    mut window: Single<&mut Window>,
    mut scree_event_w: EventWriter<ScreenEvent>,
) {
    let mut screen_size = screen.get_size();
    window.resolution.set(screen_size.x, screen_size.y);
    window.position.center(MonitorSelection::Current);
    // window.mode =
    // WindowMode::BorderlessFullscreen(MonitorSelection::Current);

    let ctx = contexts.ctx_mut();

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
            fill: Color32::from_rgb(128, 128, 128),
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
            screen.draw(
                &mut module_manager,
                &mut scree_event_w,
                ui,
                ctx,
                screen_size,
            );
        });

    if let Some(inner) = window_response {
        let window_rect = inner.response.rect;
        let size = window_rect.size();
        window.resolution.set(size.x, size.y);
    }
    window.position.center(MonitorSelection::Current);
}
