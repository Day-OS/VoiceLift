#[cfg(target_os = "android")]
use crate::android::keyboard::show_soft_input;
use egui_virtual_keyboard::VirtualKeyboard;

use bevy::prelude::*;
use bevy_egui::input::EguiInputEvent;

#[derive(Default)]
pub struct Keyboard {
    pub base: VirtualKeyboard,
    pub context: Option<Entity>,
}

impl Keyboard {
    /// Bevy system that creates key events when the virtual keyboard buffer contains
    /// pending keys
    pub fn write_events(
        &mut self,
        mut event_w: EventWriter<EguiInputEvent>,
    ) {
        if self.context.is_none() {
            return;
        }
        for key_event in self.base.events.iter() {
            event_w.write(EguiInputEvent {
                context: self.context.unwrap(),
                event: key_event.clone(),
            });
        }
        self.base.events.clear();
    }

    /// Bevy system that reads `EguiInputEvent`s and stores their context to be used later
    pub fn read_events(
        &mut self,
        mut event_r: EventReader<EguiInputEvent>,
    ) {
        for event in event_r.read() {
            self.context = Some(event.context);
        }
    }
}
