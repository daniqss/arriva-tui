mod error;
mod fetch_data;
mod stops;
use fetch_data::fetch_data;
use stops::deserialize_stops;
use error::Error;

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

    Ok(())
}