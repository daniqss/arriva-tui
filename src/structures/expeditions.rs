use std::path::Display;

use serde_json::Value;
use super::stops::Stop;
use crate::prelude::*;

const OUTWARD_STR : &str = "ida";
const RETURN_STR : &str = "vuelta";


#[derive(Debug, Clone)]
pub struct ExpeditionRequest {
    from: usize,
    to: usize,
    date: String,
}

impl ExpeditionRequest {
    pub fn new(from: usize, to: usize, date: String) -> Self {
        Self {
            from,
            to,
            date,
        }
    }

    pub fn from_stops(stops: (&Stop, &Stop), date: String) -> Self {
        Self {
            from: stops.0.get_parada(),
            to: stops.1.get_parada(),
            date,
        }
    }

    pub fn get_payload(self) -> String {
        format!("controller=buses&method=goSearch&data%5Bfrom%5D={}&data%5Bto%5D={}&data%5Bdate%5D={}", self.from, self.to, self.date)
    } 
}

pub struct Expedition {
    name: String,
    departure: String,
    arrival: String,
    cost: u64,
}

impl Expedition {
    pub fn new(name: String, departure: String, arrival: String, cost: u64) -> Self {
        Self {
            name,
            departure,
            arrival,
            cost,
        }
    }


    pub fn from(expedition_value: &Value) -> Result<Self> {
        let name = expedition_value["Descripcion_Web"].as_str()
            .ok_or_else(|| Error::Generic("Failed to get expedition name".to_string()))?.to_string();
        let departure_value = expedition_value["hora_salida"].as_str()
            .ok_or_else(|| Error::Generic("Failed to get departure value".to_string()))?.to_string();
        let arrival_value = expedition_value["hora_llegada"].as_str()
            .ok_or_else(|| Error::Generic("Failed to get arrival value".to_string()))?.to_string();
        let cost = expedition_value["tarifa_basica"].as_u64()
            .ok_or_else(|| Error::Generic("Failed to get cost value".to_string()))?;     
        
        println!("{} != {} != {} != {}", cost,
            expedition_value["tarifa_basica"].as_i64().ok_or_else(|| Error::Generic("Failed to get cost value".to_string()))?,
            expedition_value["tarifa_basica"].as_u64().ok_or_else(|| Error::Generic("Failed to get cost value".to_string()))?,
            expedition_value["tarifa_basica"].as_f64().ok_or_else(|| Error::Generic("Failed to get cost value".to_string()))?
        );

        // Hora_Salida and Hora_Llegada are in the format "YYYY-MM-DDTHH:MM:00+02:00"
        // and we want to extract "HH:MM"

        let departure = departure_value.split("T").collect::<Vec<&str>>()[1]    // "HH:MM:00+02:00"
            .split(":").collect::<Vec<&str>>()[0..2].join(":");                 // "HH:MM"

        let arrival = arrival_value.split("T").collect::<Vec<&str>>()[1]        // "HH:MM:00+02:00"
            .split(":").collect::<Vec<&str>>()[0..2].join(":");                 // "HH:MM"
    
    
        Ok(Self::new(name, departure, arrival, cost))
    }
}

impl std::fmt::Debug for Expedition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} - {} - {}", self.name, self.departure, self.arrival, self.cost)
    }
}

impl std::fmt::Display for Expedition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nLínea:     {}\nHorario:   {} -> {}\nCoste(€):  {:.2}",
            self.name,
            self.departure,
            self.arrival,
            self.cost as f64 / 100.0
        )
    }
}

pub fn deserialize_expeditions(value: Value) -> Result<(Vec<Expedition>, Vec<Expedition>)> {
    let mut expedition_errors: (bool, bool) = (false, false);

    let outward_expeditions_value = match value["expediciones"][OUTWARD_STR].as_array() {
        Some(expeditions) => expeditions,
        None => return Err(Error::Generic("Failed to parse outward expeditions".to_string()))
    };

    let return_expeditions_value = match value["expediciones"][RETURN_STR].as_array() {
        Some(expeditions) => expeditions,
        None => return Err(Error::Generic("Failed to parse return expeditions".to_string()))
    };

    let outward_expeditions: Vec<Expedition> = outward_expeditions_value.iter().map(|expedition| {
        match Expedition::from(expedition) {
            Ok(expedition) => expedition,
            Err(err) => {
                expedition_errors.1 = true;
                Expedition::new("Error".to_string(), "Error".to_string(), "Error".to_string(), 0)
            }
        }
    }).collect();

    let return_expeditions = return_expeditions_value.iter().map(|expedition| {
        match Expedition::from(expedition) {
            Ok(expedition) => expedition,
            Err(err) => {
                expedition_errors.1 = true;
                Expedition::new("Error".to_string(), "Error".to_string(), "Error".to_string(), 0)
            }
        }
    }).collect();

    match expedition_errors {
        (false, false) => Ok((outward_expeditions, return_expeditions)),
        (true, false) => Err(Error::Generic("Failed to deserialize some outward expeditions".to_string())),
        (false, true) => Err(Error::Generic("Failed to deserialize some return expeditions".to_string())),
        _ => Err(Error::Generic("Failed to deserialize both expedition arrays".to_string()))
    }
}


# [cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expedition_new() {
        let expedition = ExpeditionRequest::new(1, 2, String::from("2021-01-01"));
        assert_eq!(expedition.from, 1);
        assert_eq!(expedition.to, 2);
        assert_eq!(expedition.date, String::from("2021-01-01"));
    }

    #[test]
    fn test_expedition_from_stops() {
        let stop1 = Stop::new(1, String::from("Stop 1"), String::from("Stop 1"), 1, Some(1.0), Some(1.0), Some(1.0), Some(1.0));

        let stop2 = Stop::new(2, String::from("Stop 2"), String::from("Stop 2"), 2, Some(2.0), Some(2.0), Some(2.0), Some(2.0));

        let expedition = ExpeditionRequest::from_stops((&stop1, &stop2), String::from("2021-01-01"));
        assert_eq!(expedition.from, 1);
        assert_eq!(expedition.to, 2);
        assert_eq!(expedition.date, String::from("2021-01-01"));
    }

    #[test]
    fn test_expedition_get_payload() {
        let expedition = ExpeditionRequest::new(1, 2, String::from("2021-01-01"));
        assert_eq!(expedition.get_payload(), "controller=buses&method=goSearch&data%5Bfrom%5D=1&data%5Bto%5D=2&data%5Bdate%5D=2021-01-01");
    }
}