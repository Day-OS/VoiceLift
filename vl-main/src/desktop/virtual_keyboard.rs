#[cfg(target_os = "android")]
use crate::android::keyboard::show_soft_input;
use egui_virtual_keyboard::VirtualKeyboard;

use bevy::prelude::*;
use bevy_egui::input::EguiInputEvent;

#[derive(bevy::prelude::Resource, Default)]
pub struct Keyboard {
    pub base: VirtualKeyboard,
    pub context: Option<Entity>,
}

/// Bevy system that creates key events when the virtual keyboard buffer contains
/// pending keys
pub fn keyboard_output_event(
    mut keyboard: ResMut<Keyboard>,
    mut event_w: EventWriter<EguiInputEvent>,
) {
    if keyboard.context.is_none() {
        return;
    }
    for key_event in keyboard.base.events.iter() {
        event_w.write(EguiInputEvent {
            context: keyboard.context.unwrap(),
            event: key_event.clone(),
        });
    }
    keyboard.base.events.clear();
}

/// Bevy system that reads `EguiInputEvent`s and stores their context to be used later
pub fn keyboard_input_event(
    mut keyboard: ResMut<Keyboard>,
    mut event_r: EventReader<EguiInputEvent>,
) {
    for event in event_r.read() {
        keyboard.context = Some(event.context);
    }
}
