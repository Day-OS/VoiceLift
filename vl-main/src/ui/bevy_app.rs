#[cfg(target_os = "android")]
use crate::android::keyboard::show_soft_input;
use egui_virtual_keyboard::VirtualKeyboard;

use bevy::{
    color::palettes::basic::*,
    input::{
        gestures::RotationGesture,
        keyboard::{Key, KeyboardInput},
        mouse::MouseButtonInput,
        touch::TouchPhase,
    },
    log::{Level, LogPlugin},
    math::VectorSpace,
    prelude::*,
    window::{
        AppLifecycle, CursorOptions, WindowMode, WindowResolution,
    },
    winit::WinitSettings,
};
use bevy_egui::{
    EguiContextPass, EguiContexts, EguiInput, EguiPlugin,
    egui::{self, Frame, RawInput},
    input::EguiInputEvent,
};

#[derive(bevy::prelude::Resource)]
pub struct Keyboard(VirtualKeyboard);

impl Default for Keyboard {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub fn run() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::NONE));
    app.insert_resource(Keyboard::default());
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
                    resolution: WindowResolution::new(400., 400.),
                    fullsize_content_view: false,
                    position: WindowPosition::Centered(
                        MonitorSelection::Current,
                    ),
                    mode: WindowMode::Windowed,
                    // on iOS, gestures must be enabled.
                    // This doesn't work on Android
                    recognize_rotation_gesture: true,
                    // Only has an effect on iOS
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
    .add_systems(EguiContextPass, ui_example_system)
    .add_systems(Update, keyboard)
    .add_systems(Update, keyboard_test)
    .insert_resource(WinitSettings::mobile())
    .add_event::<VirtualKeyboardEvent>()
    .run();
}

#[derive(Resource)]
struct LoremIpsum(String);
impl Default for LoremIpsum {
    fn default() -> Self {
        Self("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string())
    }
}

#[derive(Event)]
/// Wraps Egui events emitted by [`crate::EguiInputSet`] systems.
pub struct VirtualKeyboardEvent {
    pub key: egui::Key,
}

fn keyboard(
    mut keyboard: ResMut<Keyboard>,
    mut kbdevent: EventWriter<VirtualKeyboardEvent>,
) {
    for event in keyboard.0.events.iter() {
        if let egui::Event::Key {
            key,
            physical_key: _,
            pressed: _,
            repeat: _,
            modifiers: _,
        } = event
        {
            kbdevent.write(VirtualKeyboardEvent { key: *key });
        };
    }
    keyboard.0.events.clear();
}

fn keyboard_test(
    mut text: ResMut<LoremIpsum>,
    mut kbdevent: EventReader<VirtualKeyboardEvent>,
) {
    for event in kbdevent.read() {
        text.0 += event.key.name();
        println!("{:?}", event.key)
    }
}

fn ui_example_system(
    mut text: ResMut<LoremIpsum>,
    mut contexts: EguiContexts,
    mut keyboard: ResMut<Keyboard>,
    mut cursor_moved_reader: EventReader<MouseButtonInput>,
) {
    let ctx = contexts.ctx_mut();
    let ss = ctx.screen_rect().size();
    let screen_size = bevy_egui::egui::Vec2 { x: ss.x, y: ss.y };

    egui::Window::new("Hello")
        .anchor(egui::Align2::LEFT_TOP, bevy_egui::egui::Vec2::ZERO)
        .frame(Frame::NONE)
        .default_rect(ctx.screen_rect())
        .fixed_size(screen_size)
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("world");
            ui.text_edit_multiline(&mut text.0);
            keyboard.0.show(ui)
        });
}
