mod error;
mod fetch_data;
mod stops;
mod expeditions;
use error::Error;
use fetch_data::fetch_data;
use stops::{Stop, deserialize_stops};
use expeditions::{Expedition, ExpeditionRequest};


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Fetch data from from https://arriva.gal/plataforma/api/
    let stops = match fetch_data(
            "https://arriva.gal/plataforma/api/superparadas/index/buscador.json",
            "application/json",
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

    let wanted_stops = match get_wanted_stop_from_args(stops) {
        (Some(from_stop), Some(to_stop)) => (from_stop, to_stop),
        _ => return Ok(()),
    };

    let expedition = ExpeditionRequest::from_stops(wanted_stops, String::from("18-04-2024"));
    let expedition_result = match fetch_data(
            "https://arriva.es/es/galicia/para-viajar/arriva",
            "application/x-www-form-urlencoded, charset=UTF-8",
            "controller=buses&method=goSearch&data%5Bfrom%5D=5274&data%5Bto%5D=4802&data%5Bdate%5D=19-04-2024",
        ).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };
    println!("{:?}", expedition_result);

    Ok(())
}

fn get_wanted_stop_from_args(stops: Vec<stops::Stop>) -> (Option<Stop>, Option<Stop>) {
    let args: Vec<String> = std::env::args().collect();
    let invalid_args = format!("Uso: {} <from: usize> <to: usize> <date: String>", args[0]);

    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(from) => {
                match args[2].parse::<usize>() {
                    Ok(to) => {
                        let from_stop = stops.iter().find(|stop| stop.get_parada() == from);
                        let to_stop = stops.iter().find(|stop| stop.get_parada() == to);
                        match (from_stop, to_stop) {
                            (Some(&ref from_stop), Some(&ref to_stop)) => return (Some(from_stop.clone()), Some(to_stop.clone())),
                            _ => println!("Not founded stops"),
                        }
                    },
                    Err(_) => println!("Invalid to stop"),
                }
            },
            Err(_) => println!("Invalid from stop"),
        }
    }
    println!("{}", invalid_args);
    (None, None)
}