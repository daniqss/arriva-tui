#![allow(unused)]
use crate::prelude::*;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::stdout;


mod error;
mod prelude;
mod utils;
mod structures;

use structures::*;
use utils::*;


#[tokio::main]
async fn main() -> Result<()> {
    let stops = get_stops().await?;
    let expeditions = get_expeditions(&stops).await?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(format!("{} {}", stops.0.to_string(), stops.1.to_string())).centered(),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn get_stops() -> Result<(Stop, Stop)> {
    let stops = match fetch_data(
            "https://arriva.gal/plataforma/api/superparadas/index/buscador.json",
            "application/json; charset=UTF-8",
            r#"{"key":"value"}"#,
        ).await {
        Ok(response) => {
            match deserialize_stops(response) {
                Ok(stops) => stops,
                Err(error) => return Err(error.into()),
            }
        },
        Err(e) => return Err(e.into()),
    };

    match get_wanted_stop_from_args(stops) {
        (Some(from_stop), Some(to_stop)) => Ok((from_stop, to_stop)),
        _ => Err(Error::Generic("Uso: arriva-tui <from: usize> <to: usize> <date: String>".to_string())),
    }
}

async fn get_expeditions(stops: &(Stop, Stop)) -> Result<Value> {
    let expedition = ExpeditionRequest::from_stops(stops, String::from("19-04-2024"));
    match fetch_data(
            "https://arriva.es/es/galicia/para-viajar/arriva",
            "application/x-www-form-urlencoded; charset=UTF-8",
            &expedition.get_payload(),
        ).await {
        Ok(response) => {
            let parsed: Value = match serde_json::from_str(&response) {
                Ok(parsed) => parsed,
                Err(error) => return Err(error.into()),
            };
            Ok(parsed)
        },
        Err(e) => return Err(e.into()),
    }
}