use crate::stops::Stop;

#[derive(Debug, Clone)]
pub struct ExpeditionRequest {
    from: usize,
    to: usize,
    date: String,
}

pub struct Expedition {
    from: Stop,
    to: Stop,
    date: String,
}

pub struct ExpeditionList {
    expeditions: Vec<Expedition>,
}

impl ExpeditionRequest {
    pub fn new(from: usize, to: usize, date: String) -> Self {
        Self {
            from,
            to,
            date,
        }
    }

    pub fn from_stops(stops: (Stop, Stop), date: String) -> Self {
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

        let expedition = ExpeditionRequest::from_stops((stop1, stop2), String::from("2021-01-01"));
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