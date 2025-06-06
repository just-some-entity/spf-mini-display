mod app;
mod format_duration;
mod backend_interface;
mod action;
mod data;
mod renderer;
mod util;
mod expiring_value;
mod player_info;
mod track_info;
mod tracked_player_info;
mod backend_handler;

use std::fs;
use color_eyre::Result;
use rspotify::Credentials;
use serde::{Deserialize};
use crate::app::{App};

#[derive(Deserialize)]
struct Config
{
    id: String,
    secret: String,
}

#[tokio::main]
async fn main() -> Result<()>
{
    let content = match fs::read_to_string("config.toml") 
    {
        Ok(str) => { str }
        Err(_) => { return Err(color_eyre::eyre::eyre!("config.toml is missing")); }
    };

    let config : Config = toml::from_str(&content)?;

    App::default().run(Credentials::new(&config.id, &config.secret)).await
}