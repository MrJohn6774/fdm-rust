use super::flight_data::FlightData;

pub struct FlightDataMonitoring {
    pub data: FlightData,
}

impl FlightDataMonitoring {
    pub fn new() -> Self {
        let data = FlightData::new();

        FlightDataMonitoring { data }
    }

    pub fn update_data(&mut self) {
        self.data.update().unwrap();
    }
}

impl Default for FlightDataMonitoring {
    fn default() -> Self {
        Self::new()
    }
}
