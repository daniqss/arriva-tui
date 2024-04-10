mod error;
mod fetch_data;
mod stops;
mod expeditions;
use error::Error;
use fetch_data::fetch_data;
use stops::{Stop, deserialize_stops};
use expeditions::Expedition;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Fetch data from from https://arriva.gal/plataforma/api/
    let stops = match fetch_data("https://arriva.gal/plataforma/api/superparadas/index/buscador.json").await {
        Ok(response) => {
            match deserialize_stops(response) {
                Ok(stops) => stops,
                Err(error) => return Err(error.into()),
            }
        },
        Err(e) => return Err(e.into()),
    };

    stops.iter().for_each(|stop| {
        println!("{}", stop);
    });

    let wanted_stops = match get_wanted_stop_from_args(stops) {
        (Some(from_stop), Some(to_stop)) => (from_stop, to_stop),
        _ => return Ok(()),
    };

    let expedition = Expedition::from_stops(wanted_stops, String::from("2021-01-01"));
    println!("{}", expedition.get_payload());

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