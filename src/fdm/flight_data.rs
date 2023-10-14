use crate::fsuipc::{Fsuipc, FsuipcData, FsuipcResult};

#[derive(Debug)]
pub struct Position {
    pub latitude: FsuipcData<f64, i64>,
    pub longitude: FsuipcData<f64, i64>,
}

#[derive(Debug)]
pub struct Speed {
    pub gs: FsuipcData<f64, i32>,
    pub tas: FsuipcData<f64, i32>,
    pub ias: FsuipcData<f64, i32>,
}

#[derive(Debug)]
pub struct FlightData {
    pub position: Position,
    pub speed: Speed,
    pub baro: FsuipcData<f64, i32>,
    pub altitude: FsuipcData<f64, i32>,
    pub ground_elevation: FsuipcData<f64, i32>,
    pub si_unit: FsuipcData<u8, u8>,
}

impl FlightData {
    pub fn new() -> Self {
        Self {
            position: Position {
                latitude: FsuipcData {
                    offset: 0x0560,
                    data: 0.0,
                    raw_data: 0,
                },
                longitude: FsuipcData {
                    offset: 0x0568,
                    data: 0.0,
                    raw_data: 0,
                },
            },
            speed: Speed {
                gs: FsuipcData {
                    offset: 0x02B4,
                    data: 0.0,
                    raw_data: 0,
                },
                tas: FsuipcData {
                    offset: 0x02B8,
                    data: 0.0,
                    raw_data: 0,
                },
                ias: FsuipcData {
                    offset: 0x02BC,
                    data: 0.0,
                    raw_data: 0,
                },
            },
            baro: FsuipcData {
                offset: 0x3324,
                data: 0.0,
                raw_data: 0,
            },
            altitude: FsuipcData {
                offset: 0x0574,
                data: 0.0,
                raw_data: 0,
            },
            ground_elevation: FsuipcData {
                offset: 0x0020,
                data: 0.0,
                raw_data: 0,
            },
            si_unit: FsuipcData {
                offset: 0x0C18,
                data: 0,
                raw_data: 0,
            },
        }
    }

    pub fn update(&mut self, fsuipc: &mut Fsuipc) -> FsuipcResult<()> {
        self.position.latitude.read_raw(fsuipc)?;
        self.position.longitude.read_raw(fsuipc)?;
        self.speed.gs.read_raw(fsuipc)?;
        self.speed.tas.read_raw(fsuipc)?;
        self.speed.ias.read_raw(fsuipc)?;
        self.baro.read_raw(fsuipc)?;
        self.altitude.read_raw(fsuipc)?;
        self.ground_elevation.read_raw(fsuipc)?;
        self.si_unit.read_raw(fsuipc)?;
        fsuipc.process()?;

        self.position.latitude.data =
            self.position.latitude.raw_data as f64 * (90.0 / (10001750.0 * 65536.0 * 65536.0));
        self.position.longitude.data =
            self.position.longitude.raw_data as f64 * (360.0 / (65536_f64.powf(4.0)));
        self.speed.gs.data = self.speed.gs.raw_data as f64 / 65536.0 * 1.943844; // converting m/s to knots
        self.speed.tas.data = self.speed.tas.raw_data as f64 / 128.0;
        self.speed.ias.data = self.speed.ias.raw_data as f64 / 128.0;
        self.baro.data = if self.si_unit.raw_data != 2 {
            self.baro.raw_data as f64
        } else {
            // converting meters to feet
            self.baro.raw_data as f64 * 3.28084
        };
        self.altitude.data = self.altitude.raw_data as f64 * 3.28084;
        self.ground_elevation.data = self.ground_elevation.raw_data as f64 * 3.28084 / 256.0;

        Ok(())
    }
}
