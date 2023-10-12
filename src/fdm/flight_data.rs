use std::{cell::RefCell, rc::Rc};

use crate::fsuipc::Fsuipc;

#[derive(Debug)]
pub struct RawData {
    lat: (u16, i64),
    lng: (u16, i64),
    gs: (u16, i32),
    tas: (u16, i32),
    ias: (u16, i32),
    baro: (u16, i32),
    alt: (u16, i32),
    ground_elevation: (u16, i32),
    si_unit: (u16, u16),
}

impl RawData {
    pub fn new() -> Self {
        RawData {
            lat: (0x0560, 0),
            lng: (0x0568, 0),
            gs: (0x02B4, 0),
            tas: (0x02B8, 0),
            ias: (0x02BC, 0),
            baro: (0x3324, 0),
            alt: (0x0574, 0),
            ground_elevation: (0x0020, 0),
            si_unit: (0x0C18, 0),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct Speed {
    pub gs: f64,
    pub tas: f64,
    pub ias: f64,
}

#[derive(Debug)]
pub struct FlightData {
    pub position: Position,
    pub speed: Speed,
    pub baro: f64,
    pub altitude: f64,
    pub ground_elevation: f64,
    raw_data: RawData,
}

impl FlightData {
    pub fn new() -> Self {
        FlightData {
            position: Position {
                latitude: 0.0,
                longitude: 0.0,
            },
            speed: Speed {
                gs: 0.0,
                tas: 0.0,
                ias: 0.0,
            },
            baro: 0.0,
            altitude: 0.0,
            ground_elevation: 0.0,
            raw_data: RawData::new(),
        }
    }

    pub fn update(&mut self, fsuipc_clone: Rc<RefCell<Fsuipc>>) -> Result<(), (u32, String)> {
        let mut fsuipc = fsuipc_clone.borrow_mut();
        fsuipc.read(self.raw_data.lat.0, &mut self.raw_data.lat.1)?;
        fsuipc.read(self.raw_data.lng.0, &mut self.raw_data.lng.1)?;
        fsuipc.read(self.raw_data.gs.0, &mut self.raw_data.gs.1)?;
        fsuipc.read(self.raw_data.tas.0, &mut self.raw_data.tas.1)?;
        fsuipc.read(self.raw_data.ias.0, &mut self.raw_data.ias.1)?;
        fsuipc.read(self.raw_data.baro.0, &mut self.raw_data.baro.1)?;
        fsuipc.read(self.raw_data.alt.0, &mut self.raw_data.alt.1)?;
        fsuipc.read(
            self.raw_data.ground_elevation.0,
            &mut self.raw_data.ground_elevation.1,
        )?;
        fsuipc.read(self.raw_data.si_unit.0, &mut self.raw_data.si_unit.1)?;
        fsuipc.process()?;

        self.position.latitude =
            self.raw_data.lat.1 as f64 * (90.0 / (10001750.0 * 65536.0 * 65536.0));
        self.position.longitude = self.raw_data.lng.1 as f64 * (360.0 / (65536_f64.powf(4.0)));
        self.speed.gs = self.raw_data.gs.1 as f64 / 65536.0 * 1.943844; // converting m/s to knots
        self.speed.tas = self.raw_data.tas.1 as f64 / 128.0;
        self.speed.ias = self.raw_data.ias.1 as f64 / 128.0;
        self.baro = if self.raw_data.si_unit.1 != 2 {
            self.raw_data.baro.1 as f64
        } else {
            // converting meters to feet
            self.raw_data.baro.1 as f64 * 3.28084
        };
        self.altitude = self.raw_data.alt.1 as f64 * 3.28084;
        self.ground_elevation = self.raw_data.ground_elevation.1 as f64 * 3.28084 / 256.0;

        Ok(())
    }
}
