use std::sync::Arc;

#[cfg(target_os = "android")]
use crate::android::keyboard::show_soft_input;

use async_lock::{Mutex, RwLock};
use bevy::{
    ecs::entity::unique_slice::Windows,
    log::{Level, LogPlugin},
    platform::collections::HashMap,
    prelude::*,
    window::{
        CursorOptions, WindowMode, WindowResized, WindowResolution,
    },
};
use bevy_egui::{
    EguiContextPass, EguiContexts, EguiPlugin,
    egui::{self, Color32, CornerRadius, Frame, Margin, Vec2},
};

use super::screens::{
    Screen, ScreenManager, main_screen::MainScreen,
};

pub fn run() {
    let mut app: App = App::new();
    app.insert_resource(ClearColor(Color::NONE));
    let mut main_screen = MainScreen::default();
    let mut screen_manager =
        ScreenManager::new(Arc::new(RwLock::new(main_screen)));
    screen_manager.register_systems(&mut app);
    app.insert_resource(screen_manager);
    app.insert_resource(LoremIpsum::default());
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
    .add_systems(EguiContextPass, egui_screen)
    .add_systems(Update, on_resize_system)
    // .add_systems(Update, keyboard_test)
    // .insert_resource(WinitSettings::mobile())
    // .add_event::<VirtualKeyboardEvent>()
    .run();
}

#[derive(Resource)]
struct LoremIpsum(String);
impl Default for LoremIpsum {
    fn default() -> Self {
        Self("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string())
    }
}

// #[derive(Event)]
// /// Wraps Egui events emitted by [`crate::EguiInputSet`] systems.
// pub struct VirtualKeyboardEvent {
//     pub key: egui::Key,
// }

// fn keyboard_test(
//     mut text: ResMut<LoremIpsum>,
//     mut kbdevent: EventReader<VirtualKeyboardEvent>,
// ) {
//     for event in kbdevent.read() {
//         text.0 += event.key.name();
//         println!("{:?}", event.key)
//     }
// }

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        // When resolution is being changed
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}

fn egui_screen(
    mut contexts: EguiContexts,
    mut screen: ResMut<ScreenManager>,
    mut window: Single<&mut Window>,
) {
    let mut screen_size = screen.get_size();
    window.resolution.set(screen_size.x, screen_size.y);
    window.position.center(MonitorSelection::Current);
    // window.mode =
    // WindowMode::BorderlessFullscreen(MonitorSelection::Current);

    let ctx = contexts.ctx_mut();
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
    egui::Window::new("MainScreen")
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
        .fixed_size(Vec2 { x: 10., y: 10. })
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(&ctx.clone(), |ui| {
            screen.draw(ui, ctx, screen_size);
        });
}
