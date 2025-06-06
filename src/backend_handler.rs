use std::time::Duration;
use rspotify::AuthCodeSpotify;
use rspotify::clients::OAuthClient;
use rspotify::model::{AdditionalType, Market, RepeatState};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::action::Action;
use crate::data::Data;
use crate::track_info::TrackInfo;
use crate::tracked_player_info::TrackedPlayerInfo;
use crate::util::DivisibleBy;

pub(crate) struct BackendHandler
{
    spotify : AuthCodeSpotify,
    action_receiver : Receiver<Action>,
    data_sender: Sender<Data>,

    track_info: TrackInfo,
    player_info: TrackedPlayerInfo
}

impl BackendHandler
{
    pub(crate) async fn new(
        spotify : AuthCodeSpotify,
        action_receiver : Receiver<Action>,
        data_sender: Sender<Data>
    ) -> Self
    {
        Self
        {
            spotify,
            action_receiver,
            data_sender,
            track_info:  TrackInfo::default(),
            player_info: TrackedPlayerInfo::new(Duration::from_secs(5)),
        }
    }

    pub(crate) async fn run(&mut self)
    {
        while let Some(action) = self.action_receiver.recv().await
        {
            self.handle_action(action).await;
        }
    }

    pub(crate) async fn handle_action(&mut self, action: Action)
    {
        match action
        {
            Action::Update =>
            {
                let res =
                    self.spotify.current_playback(
                        Some(Market::FromToken),
                        Some([AdditionalType::Track].as_slice())
                    ).await;

                let data = match res
                {
                    Ok(data_opt) =>
                        {
                            match data_opt
                            {
                                None => Data::default(),
                                Some(ctx) => Data::from_context(ctx)
                            }
                        }
                    Err(_) => Data::default()
                };

                self.track_info = data.track;

                self.player_info.import(data.player);

                self.sync_data().await;
            }

            Action::PlayOrResume =>
            {
                let running = self.player_info.is_playing.get();

                match running
                {
                    true =>
                        {
                            let _ = self.spotify.pause_playback(None).await;
                            self.player_info.is_playing.set(false);
                        }
                    false =>
                        {
                            let _ = self.spotify.resume_playback(None, None).await;
                            self.player_info.is_playing.set(true);
                        }
                }

                self.sync_data().await;
            }

            Action::VolumeUp(amount) =>
            {
                let info = &mut self.player_info;

                let val = info.volume.get().min(100);
                let cl = val.ceil_to_nearest(amount as usize);

                info.volume.set((if cl == val { val + amount } else { cl }).min(100));

                let _ = self.spotify.volume(info.volume.get(), None).await;

                self.sync_data().await;
            }

            Action::VolumeDown(amount) =>
            {
                let info = &mut self.player_info;

                let val = info.volume.get().min(100);
                let fl = val.floor_to_nearest(amount as usize);

                info.volume.set(if val == 0 { 0 } else if fl == val { val - amount } else { fl });

                let _ = self.spotify.volume(info.volume.get(), None).await;
                self.sync_data().await;
            }

            Action::Next =>
            {
                let _ = self.spotify.next_track(None).await;
            }

            Action::Previous =>
            {
                let _ = self.spotify.previous_track(None).await;
            }

            Action::ToggleRepeat =>
            {
                let state = match self.player_info.repeat_state.get()
                {
                    RepeatState::Off =>     { RepeatState::Track },
                    RepeatState::Track =>   { RepeatState::Context }
                    RepeatState::Context => { RepeatState::Off }
                };

                let _ = self.spotify.repeat(state, None).await;
                self.player_info.repeat_state.set(state);
                self.sync_data().await;
            }

            Action::ToggleShuffle =>
            {
                let state = !self.player_info.shuffle_state.get();

                let _ = self.spotify.shuffle(state, None).await;
                self.player_info.shuffle_state.set(state);
                self.sync_data().await;
            }
        }
    }

    async fn sync_data(&mut self)
    {
        let data = Data
        {
            player: self.player_info.export(),
            track:  self.track_info.clone(),
        };

        let _ = self.data_sender.send(data).await;
    }
}