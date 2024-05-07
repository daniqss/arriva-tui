#![allow(unused)]
use crate::prelude::*;
use std::io::Write;
use async_std::task::sleep;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use std::{thread, time::Duration};

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
    let stops = fetch_initial_data().await?;

    let mut terminal = app::init()?;
    let app_result = App::new(stops).run(&mut terminal).await;
    app::restore()?;
    app_result
}

async fn fetch_initial_data() -> Result<Vec<Stop>> {
    let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer.set_color(ColorSpec::new().set_fg(Some(PRIMARY_COLOR_TC)))?;
    writeln!(&mut buffer, "Fetching data...")?;
    bufwtr.print(&buffer)?;
    buffer.clear();
    
    let stops = get_stops().await?;

    writeln!(&mut buffer, "Data fetched successfully!")?;
    bufwtr.print(&buffer)?;
    thread::sleep(Duration::from_secs(2));

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