use std::io::stdout;
use crossterm::{event, execute};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use rspotify::Credentials;
use std::time::{Duration, Instant};

use crate::action::Action;
use crate::backend_interface::BackendInterface;
use crate::data::Data;
use crate::renderer::Renderer;

#[derive(Default)]
pub struct App
{
    data : Data
}

impl App
{
    pub async fn run(&mut self, creds : Credentials) -> color_eyre::Result<()>
    {
        let mut backend = BackendInterface::new(creds).await?;
        let mut renderer = Renderer::new();

        execute!(stdout(), EnableMouseCapture)?;
        
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut last_update = Instant::now();

        loop
        {
            if let Some(data) = backend.get_data()
            {
                self.data = data;
            }
            
            renderer.render(&self.data)?;

            let timeout_dur = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout_dur)?
            {
                let event = event::read()?;
                
                if let Event::Key(event) = event
                {
                    if event.code == KeyCode::Char('q') { break; }
                }
                
                match Self::event_to_action(event) 
                {
                    None => (),
                    Some(action) => backend.request_action(action).await?
                }
            }

            if last_update.elapsed().as_millis() >= 500
            {
                backend.request_action(Action::Update).await?;
                last_update = Instant::now();
            }

            last_tick = Instant::now();
        }

        execute!(stdout(), DisableMouseCapture)?;
        renderer.cleanup()
    }

    fn event_to_action(event: Event) -> Option<Action>
    {
        match event
        {
            Event::Key(event) => 
            {
                match event.code
                {
                    KeyCode::Up   => Some(Action::VolumeUp(2)),
                    KeyCode::Down => Some(Action::VolumeDown(2)),

                    KeyCode::Left  => Some(Action::Previous),
                    KeyCode::Right => Some(Action::Next),

                    KeyCode::Char(' ') => Some(Action::PlayOrResume),

                    KeyCode::Char('s') => Some(Action::ToggleShuffle),

                    KeyCode::Char('r') => Some(Action::ToggleRepeat),
                    
                    _ => None
                }
            }
            _ => None,
        }
    }
}