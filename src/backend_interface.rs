use crate::action::Action;
use crate::app::App;
use crate::backend_handler::BackendHandler;
use crate::data::Data;
use rspotify::clients::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use std::default::Default;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const SZ_BOOL: usize = size_of::<bool>();
const SZ_APP: usize = size_of::<App>();

pub struct BackendInterface
{
    action_sender : Sender<Action>,
    data_receiver : Receiver<Data>,
}

impl BackendInterface
{
    pub async fn new(creds : Credentials) -> color_eyre::Result<Self>
    {
        let spotify = Self::create_client(creds).await?;
        
        let (sender_0, receiver_0): (Sender<Action>, Receiver<Action>) = channel(SZ_BOOL * 10);
        let (sender_1, receiver_1): (Sender<Data>, Receiver<Data>)     = channel(SZ_APP * 10);
        
        thread::spawn(move ||
        {
            let rt = Runtime::new().unwrap();

            rt.block_on(async move
            {
                let mut handler = BackendHandler::new(spotify, receiver_0, sender_1).await;
                handler.run().await;
            });
        });
        
        Ok(Self { action_sender: sender_0, data_receiver: receiver_1 })
    }

    pub async fn request_action(&self, action: Action) -> color_eyre::Result<()>
    {
        self.action_sender.send(action).await?;
        Ok(())
    }

    pub fn get_data(&mut self) -> Option<Data>
    {
        match self.data_receiver.try_recv() 
        {
            Ok(data) => Some(data),
            Err(_) => None
        }
    }
    
    async fn create_client(creds : Credentials) -> color_eyre::Result<AuthCodeSpotify>
    {
        let mut oauth : OAuth = OAuth::default();
        oauth.redirect_uri = "http://localhost:8888/callback".to_string();
        oauth.scopes = scopes!(
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing"
        );

        let mut spotify = AuthCodeSpotify::new(creds.clone(), oauth);

        spotify.config = Config
        {
            token_cached: true,
            ..Config::default()
        };
        
        let url = spotify.get_authorize_url(false)?;

        spotify.prompt_for_token(&url).await?;
        
        Ok(spotify)
    }
}