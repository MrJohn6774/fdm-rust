use crossterm::{
    execute,
    terminal::{self, ClearType},
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, sync::atomic::Ordering, thread::sleep};
use std::{sync::atomic::AtomicBool, time::Instant};

use super::flight_data::FlightData;

pub struct Acars {
    data: Arc<Mutex<FlightData>>,
    stop_flag: Arc<AtomicBool>,
}
impl Acars {
    pub fn new(data: Arc<Mutex<FlightData>>, stop_flag: Arc<AtomicBool>) -> Self {
        Self { data, stop_flag }
    }

    pub fn start(&mut self) {
        let flight_data_ui = self.data.clone();
        let flight_data_api = self.data.clone();
        let stop_flag_ui = self.stop_flag.clone();
        let stop_flag_api = self.stop_flag.clone();
        let refresh_period_ui = 100; // milliseconds
        let refresh_period_api = 10; // seconds

        // Emit event with flight data to frontend listener
        std::thread::spawn(move || loop {
            let now = Instant::now();

            if stop_flag_ui.load(Ordering::SeqCst) {
                println!("Stopping Acars event emission...");
                break;
            }

            {
                let data = flight_data_ui.lock().unwrap();
                Self::print_to_console(&data);
            }

            if now.elapsed() < Duration::from_millis(refresh_period_ui) {
                sleep(Duration::from_millis(refresh_period_ui) - now.elapsed());
            }
            println!("{:?}", now.elapsed());
        });

        // Send ACARS data to api endpoint
        std::thread::spawn(move || loop {
            let now = Instant::now();

            if stop_flag_api.load(Ordering::SeqCst) {
                println!("Stopping Acars api update...");
                break;
            }

            if now.elapsed() < Duration::from_secs(refresh_period_api) {
                sleep(Duration::from_secs(refresh_period_api) - now.elapsed());
            }
        });
    }

    pub fn print_to_console(data: &FlightData) {
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )
        .unwrap();

        println!(
            "Latitude: {}, Longitude: {}",
            data.position.latitude.data, data.position.longitude.data
        );
        println!(
            "GS: {} knot | TAS: {} knot | IAS: {} knot",
            data.speed.gs.data, data.speed.tas.data, data.speed.ias.data
        );
        println!(
            "Baro: {} ft | Altitude: {} MSL | Ground: {} MSL",
            data.baro.data, data.altitude.data, data.ground_elevation.data
        );
    }
}
