use std::time::Duration;
use rspotify::model::RepeatState;
use crate::expiring_value::ExpiringValue;
use crate::player_info::PlayerInfo;

// Some data isn't updated in the api immediately, and thus we store the future value which is valid
// For a period of time, before falling back to the received value from the api
// e.g. volume can take up to 3 seconds to be updated in the api
pub struct TrackedPlayerInfo
{
    pub is_playing:     ExpiringValue<bool>,
    pub shuffle_state:  ExpiringValue<bool>,
    pub repeat_state:   ExpiringValue<RepeatState>,
    pub device_name:    ExpiringValue<String>,
    pub volume:         ExpiringValue<u8>,
    pub progress:       ExpiringValue<f32>,
}

impl TrackedPlayerInfo
{
    pub fn new(duration : Duration) -> Self
    {
        Self
        {
            is_playing:     ExpiringValue::new(false, duration),
            shuffle_state:  ExpiringValue::new(false, duration),
            repeat_state:   ExpiringValue::new(RepeatState::Off, duration),
            device_name:    ExpiringValue::new("N/A".to_string(), duration),
            volume:         ExpiringValue::new(0, duration),
            progress:       ExpiringValue::new(0.0, duration),
        }
    }

    pub fn import(&mut self, info: PlayerInfo)
    {
        self.is_playing.set_expired(info.is_playing);
        self.shuffle_state.set_expired(info.shuffle_state);
        self.repeat_state.set_expired(info.repeat_state);
        self.device_name.set_expired(info.device_name);
        self.volume.set_expired(info.volume);
        self.progress.set_expired(info.progress);
    }

    pub fn export(&self) -> PlayerInfo
    {
        PlayerInfo
        {
            is_playing:     self.is_playing.get(),
            shuffle_state:  self.shuffle_state.get(),
            repeat_state:   self.repeat_state.get(),
            device_name:    self.device_name.get(),
            volume:         self.volume.get(),
            progress:       self.progress.get(),
        }
    }
}