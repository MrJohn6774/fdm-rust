use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::fsuipc::{Fsuipc, FsuipcResult};

use super::{acars::Acars, flight_data::FlightData};

pub struct FlightDataMonitoring {
    data: Arc<Mutex<FlightData>>,
    acars: Acars,
    fsuipc: Fsuipc,
    refresh_rate: u32,
    stop_flag: Arc<AtomicBool>,
}

impl FlightDataMonitoring {
    pub fn new(stop_flag: Arc<AtomicBool>) -> Self {
        let data = Arc::new(Mutex::new(FlightData::new()));
        let acars = Acars::new(Arc::clone(&data), Arc::clone(&stop_flag));
        Self {
            data,
            acars,
            fsuipc: Fsuipc::new(),
            refresh_rate: 50,
            stop_flag,
        }
    }

    pub fn run(&mut self) -> FsuipcResult<()> {
        self.fsuipc.connect()?;
        self.acars.start();
        loop {
            if self.stop_flag.load(Ordering::SeqCst) {
                println!("Stopping FDM loop...");
                break;
            }
            let now = Instant::now();

            {
                self.data.lock().unwrap().update(&mut self.fsuipc)?;
            }

            if now.elapsed() < Duration::new(0, 999_999_900 / self.refresh_rate) {
                thread::sleep(Duration::new(0, 1_000_000_000 / self.refresh_rate) - now.elapsed());
            }
        }
        Ok(())
    }
}
