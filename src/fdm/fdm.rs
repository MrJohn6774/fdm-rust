use std::{
    io, thread,
    time::{Duration, Instant},
};

use crossterm::{
    execute,
    terminal::{self, ClearType},
};

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

    pub fn run(&mut self) {
        loop {
            let now = Instant::now();
            self.update_data();
            let elapsed = now.elapsed();

            execute!(
                io::stdout(),
                terminal::Clear(ClearType::All),
                crossterm::cursor::MoveTo(0, 0)
            )
            .unwrap();

            println!(
                "Latitude: {}, Longitude: {}",
                self.data.position.latitude, self.data.position.longitude
            );
            println!(
                "GS: {} knot | TAS: {} knot | IAS: {} knot",
                self.data.speed.gs, self.data.speed.tas, self.data.speed.ias
            );
            println!(
                "Baro: {} ft | Altitude: {} MSL | Ground: {} MSL",
                self.data.baro, self.data.altitude, self.data.ground_elevation
            );
            println!("Elapsed: {:?}", elapsed);

            thread::sleep(Duration::new(0, 8_000_000));
        }
    }
}

impl Default for FlightDataMonitoring {
    fn default() -> Self {
        Self::new()
    }
}
