use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::fsuipc::Fsuipc;

use super::flight_data::FlightData;

pub struct FlightDataMonitoring {
    pub data: FlightData,
    fsuipc: Rc<RefCell<Fsuipc>>,
}

impl FlightDataMonitoring {
    pub fn new() -> Self {
        let data = FlightData::new();
        let fsuipc = Rc::new(RefCell::new(Fsuipc::new()));

        FlightDataMonitoring { data, fsuipc }
    }

    pub fn update_data(&mut self) -> Result<(), (u32, String)> {
        self.data.update(Rc::clone(&self.fsuipc))?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), (u32, String)> {
        self.fsuipc.borrow_mut().connect()?;
        execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();
        loop {
            let now = Instant::now();
            self.update_data()?;
            let elapsed = now.elapsed();

            io::stdout().execute(cursor::MoveTo(0, 0)).unwrap();

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

            io::stdout().flush().unwrap();

            thread::sleep(Duration::new(0, 100_000_000));
        }
    }
}
