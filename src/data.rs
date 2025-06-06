use rspotify::model::{CurrentPlaybackContext, PlayableItem};
use crate::player_info::PlayerInfo;
use crate::track_info::TrackInfo;

#[derive(Clone, Default)]
pub struct Data
{
    pub player: PlayerInfo,
    pub track:  TrackInfo
}

impl Data
{
    pub fn from_context(ctx : CurrentPlaybackContext) -> Self
    {
        let item_info : (String, String, i32) = match ctx.item
        {
            Some(PlayableItem::Track(track)) =>
            {
                let artists = track.artists
                    .iter()
                    .map(|a| a.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                (track.name, artists, track.duration.num_milliseconds() as i32)
            }
            Some(PlayableItem::Episode(ep)) =>
                (ep.name, ep.show.publisher, ep.duration.num_milliseconds() as i32),

            _ => ("N/A".into(), "N/A".into(), 0)
        };

        let progress = match ctx.progress
        {
            None => 0.0,
            Some(duration) => duration.num_milliseconds() as f32 / item_info.2 as f32
        };

        Self
        {
            player: PlayerInfo
            {
                is_playing:     ctx.is_playing,
                shuffle_state:  ctx.shuffle_state,
                repeat_state:   ctx.repeat_state,
                device_name:    ctx.device.name,
                volume:         ctx.device.volume_percent.unwrap_or(0) as u8,
                progress:       progress,
            },

            track: TrackInfo
            {
                title:   item_info.0,
                authors: item_info.1,
                duration:item_info.2,
            },
        }
    }
}