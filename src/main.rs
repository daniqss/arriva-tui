#![allow(unused)]
use crate::prelude::*;
use std::io::Write;
use async_std::task::sleep;
use crossterm::terminal;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use std::{thread, time::Duration, io::Stdout};
use chrono::prelude::Local;
use ratatui::{backend::CrosstermBackend, prelude::buffer::Buffer, Terminal};

mod error;
mod prelude;
mod utils;
mod structures;
mod app;

use structures::*;
use utils::fetch_data;
use app::App;


#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = app::init()?;
    
    let stops = fetch_initial_data(&mut terminal).await?;
    terminal.set_cursor(0, 0);
    thread::sleep(Duration::from_secs(1));
    terminal.clear()?;

    let app_result = App::new(stops).run(&mut terminal).await;
    app::restore()?;
    app_result
}

async fn fetch_initial_data(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<Vec<Stop>> {
    let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer.set_color(ColorSpec::new().set_fg(Some(PRIMARY_COLOR_TC)))?;
    writeln!(&mut buffer, "Fetching data...")?;
    bufwtr.print(&buffer)?;
    terminal.set_cursor(0, 0);
    buffer.clear();
    
    let stops = get_stops().await?;

    writeln!(&mut buffer, "Data fetched successfully!")?;
    bufwtr.print(&buffer)?;
    terminal.set_cursor(0, 0);

    Ok(stops)
}

async fn get_stops() -> Result<Vec<Stop>> {
    match fetch_data(
            "https://arriva.gal/plataforma/api/superparadas/index/buscador.json",
            "application/json; charset=UTF-8",
            r#"{"key":"value"}"#,
        ).await {
        Ok(response) => {
            match deserialize_stops(response) {
                Ok(stops) => Ok(stops),
                Err(error) => return Err(error.into()),
            }
        },
        Err(e) => return Err(e.into()),
    }
}

async fn get_expeditions(stops: (&Stop, &Stop), date: Option<String>) -> Result<Value> {

    let date = match date {
        Some(date) => date,
        None => Local::now().format("%d-%m-%Y").to_string(),
    };

    let expedition_req = ExpeditionRequest::from_stops(stops, date);
    match fetch_data(
            "https://arriva.es/es/galicia/para-viajar/arriva",
            "application/x-www-form-urlencoded; charset=UTF-8",
            &expedition_req.get_payload(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_stops() {
        let stop_1 = Stop::new(
            5274,
            "Estación de Coruña (A CORUÑA)".to_string(), 
            "Estación de Coruña".to_string(),
            516,
            Some(43.3531),
            Some(-8.4053),
            Some(43.3531),
            Some(-8.4053),
        );

        let stops = match get_stops().await {
            Ok(stops) => {
                // println!("Stops: {:?}", stops);
                stops
            },
            Err(err) => panic!("Error fetching stops: \n {:?}", err),
        };

        assert_eq!(stops[0], stop_1);
    }

    #[tokio::test]
    async fn test_get_expeditions() {
        let stop_1 = Stop::new(
            5274,
            "Estación de Coruña (A CORUÑA)".to_string(), 
            "Estación de Coruña".to_string(),
            516,
            Some(43.3531),
            Some(-8.4053),
            Some(43.3531),
            Some(-8.4053),
        );

        let stop_2 = Stop::new(
            5714,
            "Laracha (LARACHA)".to_string(), 
            "Laracha".to_string(),
            121,
            Some(43.2492),
            Some(-8.5872),
            Some(43.2492),
            Some(-8.5872),
        );

        let date = Local::now().format("%d-%m-%Y").to_string();

        let expeditions_value: Value = match get_expeditions((&stop_1, &stop_2), Some(date)).await {
            Ok(expeditions) => {
                expeditions
            },
            Err(err) => panic!("Error fetching expeditions: \n {:?}", err),
        };

        let expeditions: (Vec<Expedition>, Vec<Expedition>) = match deserialize_expeditions(expeditions_value) {
            Ok(expeditions) => expeditions,
            Err(err) => panic!("Error deserializing expeditions: \n {:?}", err),
        };

        println!("Outward expeditions:");
        for expedition in expeditions.0 {
            println!("{}", expedition);
        }

        println!("\nReturn expeditions:");
        for expedition in expeditions.1 {
            println!("{}", expedition);
        }
    }
}