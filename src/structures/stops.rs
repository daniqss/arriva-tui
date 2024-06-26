use serde::Deserialize;
use serde_json::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Deserialize, Clone, PartialEq)]
pub struct Stop {
    parada: usize,
    nombre: String,
    nom_web: String,
    peso: isize,

    // They can be null or float64
    #[serde(default)]
    lat: Option<f64>,
    #[serde(default)]
    lon: Option<f64>,
    #[serde(default)]
    latitud: Option<f64>,
    #[serde(default)]
    longitud: Option<f64>,
}

impl Stop {
    pub fn new(
        parada: usize,
        nombre: String,
        nom_web: String,
        peso: isize,
        lat: Option<f64>,
        lon: Option<f64>,
        latitud: Option<f64>,
        longitud: Option<f64>,
    ) -> Self {
        Self {
            parada,
            nombre,
            nom_web,
            peso,
            lat,
            lon,
            latitud,
            longitud,
        }
    }

    pub fn get_parada(&self) -> usize {
        self.parada
    }

    pub fn get_nombre(&self) -> String {
        self.nombre.clone()
    }
}

impl Debug for Stop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\nStop: {} \n\t{} \n\t{} \n\t{} \n\t{} \n\t{} \n\t{} \n\t{}",
            self.parada,
            self.nombre,
            self.nom_web,
            self.peso,
            self.format_option_f64(&self.lat),
            self.format_option_f64(&self.lon),
            self.format_option_f64(&self.latitud),
            self.format_option_f64(&self.longitud),
        )
    }
}

impl Display for Stop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\nParada: {}\nNombre: {}\nLatitud: {}\nLongitud: {}\n",
            self.parada,
            self.nom_web,
            self.format_option_f64(&self.latitud),
            self.format_option_f64(&self.longitud),
        )
    }
}

impl Stop {
    // Función auxiliar para formatear Option<f64>
    fn format_option_f64(&self, value: &Option<f64>) -> String {
        match value {
            Some(val) => val.to_string(),
            None => String::from("None"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct StopList {
    paradas: Vec<Stop>,
}

pub fn deserialize_stops(response: String) -> Result<Vec<Stop>, Error> {
    let stop_list: StopList = serde_json::from_str(&response)?;
    Ok(stop_list.paradas)
}
