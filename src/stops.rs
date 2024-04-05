use serde::Deserialize;
use serde_json::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Deserialize)]
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
    longitud:Option<f64>
}

impl Debug for Stop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Stop: {} \n\t{} \n\t{} \n\t{} \n\t{} \n\t{} \n\t{} \n\t{}",
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
        write!(f, "Parada: {} 
            \tNombre: {} 
            \tLatitud: {} 
            \tLongitud: {} 
            \n",
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
