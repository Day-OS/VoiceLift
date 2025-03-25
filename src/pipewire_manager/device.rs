use crate::pipewire_manager::command::{link, unlink, LinkError, UnlinkError};

pub trait IAudioDevice {
    fn get_audio_channel(&self) -> AudioChannel;

    fn get_channels(&self) -> (String, String) {
        self.get_audio_channel().get_channels()
    }

    fn link<T: IAudioDevice>(&self, other_device: &T) -> Result<(), LinkError> {
        let self_channels = self.get_channels();
        let other_channels = other_device.get_channels();
        link(&self_channels.0, &other_channels.0)?;
        link(&self_channels.1, &other_channels.1)?;
        Ok(())
    }

    fn unlink<T: IAudioDevice>(&self, other_device: &T) -> Result<(), UnlinkError> {
        let self_channels = self.get_channels();
        let other_channels = other_device.get_channels();
        unlink(&self_channels.0, &other_channels.0)?;
        unlink(&self_channels.1, &other_channels.1)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum AudioChannel {
    Stereo {
        front_left: String,
        front_right: String,
    },
    Mono {
        mono: String,
    },
}
impl AudioChannel {
    pub fn get_channels(&self) -> (String, String) {
        match self {
            AudioChannel::Stereo {
                front_left,
                front_right,
            } => (front_left.clone(), front_right.clone()),
            AudioChannel::Mono { mono } => (mono.clone(), mono.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub nick: String,
    pub name: String,
    pub channel: AudioChannel,
}

#[derive(Debug)]
pub struct InputDevice {
    pub device: AudioDevice,
}
impl IAudioDevice for InputDevice {
    fn get_audio_channel(&self) -> AudioChannel {
        self.device.channel.clone()
    }
}

#[derive(Debug)]
pub struct OutputDevice {
    pub device: AudioDevice,
}

impl IAudioDevice for OutputDevice {
    fn get_audio_channel(&self) -> AudioChannel {
        self.device.channel.clone()
    }
}

#[derive(Debug)]
pub enum DeviceType {
    Input(InputDevice),
    Output(OutputDevice),
}
