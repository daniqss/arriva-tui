mod fetch_data;
mod stops;
use fetch_data::fetch_data;
use stops::deserialize_stops;

#[derive(Debug)]
struct Error {
    message: String,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Fetch data from from https://arriva.gal/plataforma/api/
    let stops = match fetch_data("superparadas/index/buscador.json").await {
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