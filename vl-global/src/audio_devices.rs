use std::collections::HashSet;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default,
)]
pub struct AudioDevices {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum AudioDeviceStatus {
    SelectedAndAvailable,
    NotSelectedButAvailable,
    SelectedButNotAvailable,
}

#[derive(Eq, PartialEq, Hash)]
pub enum AudioDeviceType {
    INPUT,
    OUTPUT,
}

pub struct AudioDevicesComparison(
    pub  IndexMap<
        AudioDeviceType,
        IndexMap<AudioDeviceStatus, Vec<String>>,
    >,
);

impl AudioDevices {
    fn _compare_lists(
        data: &mut IndexMap<AudioDeviceStatus, Vec<String>>,
        available: &[String],
        selected: &[String],
    ) {
        let available_set: HashSet<_> = available.iter().collect();
        let selected_set: HashSet<_> = selected.iter().collect();

        let selected_and_available: Vec<String> = selected_set
            .intersection(&available_set)
            .map(|&s| s.clone())
            .collect();
        data.insert(
            AudioDeviceStatus::SelectedAndAvailable,
            selected_and_available,
        );

        let not_selected_but_available: Vec<String> = available_set
            .difference(&selected_set)
            .map(|&s| s.clone())
            .collect();
        data.insert(
            AudioDeviceStatus::NotSelectedButAvailable,
            not_selected_but_available,
        );

        let selected_but_not_available: Vec<String> = selected_set
            .difference(&available_set)
            .map(|&s| s.clone())
            .collect();

        data.insert(
            AudioDeviceStatus::SelectedButNotAvailable,
            selected_but_not_available,
        );
    }

    pub fn compare_lists(
        available_list: &AudioDevices,
        selected_list: &AudioDevices,
    ) -> AudioDevicesComparison {
        let mut result: IndexMap<
            AudioDeviceType,
            IndexMap<AudioDeviceStatus, Vec<String>>,
        > = IndexMap::new();

        // ---------------
        // Input Handling
        // ---------------
        let mut input: IndexMap<AudioDeviceStatus, Vec<String>> =
            IndexMap::new();

        // Process input devices
        Self::_compare_lists(
            &mut input,
            &available_list.input_devices,
            &selected_list.input_devices,
        );

        result.insert(AudioDeviceType::INPUT, input);

        // ---------------
        // Output Handling
        // ---------------
        let mut output: IndexMap<AudioDeviceStatus, Vec<String>> =
            IndexMap::new();

        // Process input devices
        Self::_compare_lists(
            &mut output,
            &available_list.output_devices,
            &selected_list.output_devices,
        );

        result.insert(AudioDeviceType::OUTPUT, output);

        AudioDevicesComparison(result)
    }
}
