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
    cost: usize,
}

impl Expedition {
    pub fn new(name: String, departure: String, arrival: String, cost: usize) -> Self {
        Self {
            name,
            departure,
            arrival,
            cost,
        }
    }


    pub fn from(expeditions: &Value, expedition_direction: &str) -> Result<Self> {
        let expedition_value = expeditions["expeditions"][expedition_direction].clone();

        let name = expedition_value["Descripcion_Web"].as_str()
            .ok_or_else(|| Error::Generic("Failed to get expedition name".to_string()))?.to_string();
        let departure_value = expedition_value["Hora_Salida"].as_str()
            .ok_or_else(|| Error::Generic("Failed to get departure value".to_string()))?.to_string();
        let arrival_value = expedition_value["Hora_Llegada"].as_str()
            .ok_or_else(|| Error::Generic("Failed to get arrival value".to_string()))?.to_string();
        let cost = expedition_value["tarifa_basica"].as_u64()
            .ok_or_else(|| Error::Generic("Failed to get cost value".to_string()))? as usize;     
    
        // Hora_Salida and Hora_Llegada are in the format "YYYY-MM-DDTHH:MM:00+02:00"
        // and we want to extract "HH:MM"

        let departure = departure_value.split("T").collect::<Vec<&str>>()[1]    // "HH:MM:00+02:00"
            .split(":").collect::<Vec<&str>>()[0..2].join(":");                 // "HH:MM"

        let arrival = arrival_value.split("T").collect::<Vec<&str>>()[1]        // "HH:MM:00+02:00"
            .split(":").collect::<Vec<&str>>()[0..2].join(":");                 // "HH:MM"
    
    
        Ok(Self::new(name, departure, arrival, cost))
    }
}
pub fn deserialize_expeditions(value: Value) -> Result<(Vec<Expedition>, Vec<Expedition>)> {
    println!("outward: {:?}", value["expediciones"][OUTWARD_STR]);
    let outward_expeditions = value["expediciones"][OUTWARD_STR].as_array()
        .ok_or_else(|| Error::Generic("Failed to get outward expeditions".to_string()))?;
        
    let return_expeditions = value["expediciones"][RETURN_STR].as_array()
        .ok_or_else(|| Error::Generic("Failed to get return expeditions".to_string()))?;
        

    let mut expedition_errors: (bool, bool) = (false, false);

    let outward_expeditions = outward_expeditions.iter().map(|expedition| {
        match Expedition::from(expedition, OUTWARD_STR) {
            Ok(expedition) => expedition,
            Err(err) => {
                expedition_errors.0 = true;
                Expedition::new("Error".to_string(), "Error".to_string(), "Error".to_string(), 0)
            }
        }
    }).collect();

    let return_expeditions = return_expeditions.iter().map(|expedition| {
        match Expedition::from(expedition, RETURN_STR) {
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