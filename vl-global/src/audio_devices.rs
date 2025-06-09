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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum AudioDeviceStatus {
    SelectedAndAvailable,
    NotSelectedButAvailable,
    SelectedButNotAvailable,
}

impl AudioDeviceStatus {
    pub fn is_selected(&self) -> bool {
        match self {
            AudioDeviceStatus::SelectedAndAvailable
            | AudioDeviceStatus::SelectedButNotAvailable => true,
            AudioDeviceStatus::NotSelectedButAvailable => false,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum AudioDeviceType {
    INPUT,
    OUTPUT,
}

#[derive(Debug, Clone)]
pub struct AudioDevicesComparison(
    pub  IndexMap<
        AudioDeviceType,
        IndexMap<AudioDeviceStatus, Vec<String>>,
    >,
);

impl AudioDevicesComparison {
    pub fn are_there_reconnected_devices(
        old: &Self,
        new: &Self,
    ) -> bool {
        for (audio_type, old_status) in &old.0 {
            let new_status = new.0.get(audio_type).unwrap();

            // Verify each other
            let were_not_available = old_status
                .get(&AudioDeviceStatus::SelectedButNotAvailable)
                .unwrap();

            let are_available_now = new_status
                .get(&AudioDeviceStatus::SelectedAndAvailable)
                .unwrap();

            if were_not_available
                .iter()
                .any(|device| are_available_now.contains(device))
            {
                return true;
            }
        }
        false
    }
}

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
