use rspotify::model::RepeatState;

#[derive(Clone)]
pub struct PlayerInfo
{
    pub is_playing:     bool,
    pub shuffle_state:  bool,
    pub repeat_state:   RepeatState,
    pub device_name:    String,
    pub volume:         u8,
    pub progress:       f32,
}

impl Default for PlayerInfo
{
    fn default() -> Self
    {
        Self
        {
            is_playing:     false,
            shuffle_state:  false,
            repeat_state:   RepeatState::Off,
            device_name:    "N/A".to_string(),
            volume:         0,
            progress:       0.0,
        }
    }
}