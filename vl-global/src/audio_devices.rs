use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default,
)]
pub struct AudioDevices {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
}

pub struct AudioDevicesComparison {
    pub selected_and_available: AudioDevices,
    pub not_selected_but_available: AudioDevices,
    pub selected_but_not_available: AudioDevices,
}

impl AudioDevices {
    fn _compare_lists(
        available: &[String],
        selected: &[String],
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let available_set: HashSet<_> = available.iter().collect();
        let selected_set: HashSet<_> = selected.iter().collect();

        let selected_and_available: Vec<String> = selected_set
            .intersection(&available_set)
            .map(|&s| s.clone())
            .collect();

        let not_selected_but_available: Vec<String> = available_set
            .difference(&selected_set)
            .map(|&s| s.clone())
            .collect();

        let selected_but_not_available: Vec<String> = selected_set
            .difference(&available_set)
            .map(|&s| s.clone())
            .collect();

        (
            selected_and_available,
            not_selected_but_available,
            selected_but_not_available,
        )
    }

    pub fn compare_lists(
        available_list: &AudioDevices,
        selected_list: &AudioDevices,
    ) -> AudioDevicesComparison {
        // Process input devices
        let (input_sa, input_nba, input_sna) = Self::_compare_lists(
            &available_list.input_devices,
            &selected_list.input_devices,
        );

        // Process output devices
        let (output_sa, output_nba, output_sna) =
            Self::_compare_lists(
                &available_list.output_devices,
                &selected_list.output_devices,
            );

        AudioDevicesComparison {
            selected_and_available: AudioDevices {
                input_devices: input_sa,
                output_devices: output_sa,
            }
            .sort(),
            not_selected_but_available: AudioDevices {
                input_devices: input_nba,
                output_devices: output_nba,
            }
            .sort(),
            selected_but_not_available: AudioDevices {
                input_devices: input_sna,
                output_devices: output_sna,
            }
            .sort(),
        }
    }
    fn sort(mut self) -> Self {
        self.input_devices.sort();
        self.output_devices.sort();
        self
    }
}
