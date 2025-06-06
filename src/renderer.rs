use color_eyre::eyre::eyre;
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Line, Span, Style, Text};
use rspotify::model::RepeatState;
use crate::data::Data;
use crate::format_duration::FormatDuration;

pub struct Renderer
{
    terminal : DefaultTerminal,
    destroyed : bool,
}

impl Renderer
{
    pub fn new()-> Self
    {
        Self
        {
            terminal: ratatui::init(),
            destroyed : false,
        }
    }
    
    pub fn cleanup(&self) -> color_eyre::Result<()>
    {
        match self.destroyed 
        {
            true => { Err(eyre!("Already destroyed"))  }
            false => 
            {
                ratatui::restore();
                Ok(())
            }
        }
    }
    
    pub fn render(&mut self, data : &Data) -> color_eyre::Result<()>
    {
        self.terminal.draw(|frame: &mut Frame| Self::render_impl(frame, data))?;
        Ok(())
    }
    
    fn render_impl(frame: &mut Frame<'_>, data : &Data)
    {
        let spacer = Constraint::Length(1);

        let horizontal = Layout::horizontal(
            [
                spacer,
                Constraint::Fill(1),
                spacer,
            ]);

        let [_, area,_] = horizontal.areas(frame.area());

        let track_style    = Style::default().fg(Color::Rgb(125, 207, 255));
        let authors_style  = Style::default().fg(Color::Rgb(247, 118, 142));
        let metadata_style = Style::default().fg(Color::Rgb(192, 202, 245));

        let progress_str = ((data.player.progress * data.track.duration as f32) as i32).to_min_sec_str();
        let duration_str = data.track.duration.to_min_sec_str();
        let txt_ln = progress_str.len() + duration_str.len() + 2;

        let ln = area.width as usize - txt_ln;
        let p  = (ln as f32 * data.player.progress) as usize;
        let line_full  = "─".repeat(p);
        let line_empty = "─".repeat(ln - p);

        let line = Line::from(vec!
        [
            Span::styled(progress_str, metadata_style),
            Span::raw(" "),
            Span::styled(line_full,  metadata_style),
            Span::styled(line_empty, Style::default().fg(Color::Rgb(74, 85, 137))),
            Span::raw(" "),
            Span::styled(duration_str, metadata_style),
        ]);

        let rp_state = match data.player.repeat_state
        {
            RepeatState::Off =>     { "off" }
            RepeatState::Track =>   { "track" }
            RepeatState::Context => { "context" }
        };

        let st = if data.player.is_playing { "⯈".to_string() } else { "∣∣".to_string() };
        let info = Text::from(vec!
        [
            Line::raw(""),
            Line::styled(format!("{} {}", st, data.track.title), track_style),
            Line::styled(data.track.authors.clone(), authors_style),
            Line::raw(""),
            Line::styled(format!("repeat: {} | shuffle: {}", rp_state, data.player.shuffle_state), metadata_style),
            Line::styled(format!("device: {}", data.player.device_name.clone()), metadata_style),
            Line::styled(format!("volume: {}%", data.player.volume), metadata_style),
            Line::raw(""),
            line
        ]);

        frame.render_widget(info, area);
    }
}