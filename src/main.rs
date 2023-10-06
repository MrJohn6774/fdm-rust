use crossterm::{
    execute,
    terminal::{self, ClearType},
};
use std::io;
use std::{thread, time};

pub mod fsuipc;

use crate::fsuipc::Fsuipc;

fn main() {
    let mut fsuipc = Fsuipc::new();
    fsuipc.connect().unwrap();

    println!("Version: {}", get_version(&mut fsuipc));

    thread::sleep(time::Duration::from_secs(2));

    loop {
        let mut lat_raw = 0_i64;
        let mut lng_raw = 0_i64;
        let mut gs_raw = 0_i32;
        let mut tas_raw = 0_i32;
        let mut ias_raw = 0_i32;
        let mut alt_raw = 0_i32;
        let mut hgt_raw = 0_i32;
        let mut ground_elevation_raw = 0_i32;
        let mut si_unit = 0_u16;
        fsuipc.read(0x0560, &mut lat_raw).unwrap();
        fsuipc.read(0x0568, &mut lng_raw).unwrap();
        fsuipc.read(0x02B4, &mut gs_raw).unwrap();
        fsuipc.read(0x02B8, &mut tas_raw).unwrap();
        fsuipc.read(0x02BC, &mut ias_raw).unwrap();
        fsuipc.read(0x3324, &mut alt_raw).unwrap();
        fsuipc.read(0x0574, &mut hgt_raw).unwrap();
        fsuipc.read(0x0020, &mut ground_elevation_raw).unwrap();
        fsuipc.read(0x0C18, &mut si_unit).unwrap();
        fsuipc.process().unwrap();

        let lat = lat_raw as f64 * (90.0 / (10001750.0 * 65536.0 * 65536.0));
        let lng = lng_raw as f64 * (360.0 / (65536_f64.powf(4.0)));
        let gs = gs_raw as f64 / 65536.0 * 1.943844; // converting m/s to knots
        let tas = tas_raw as f64 / 128.0;
        let ias = ias_raw as f64 / 128.0;
        let alt = if si_unit != 2 {
            alt_raw as f64
        } else {
            // converting meters to feet
            alt_raw as f64 * 3.28084
        };
        let hgt = hgt_raw as f64 * 3.28084; // converting meters to feet
        let ground_elevation = ground_elevation_raw as f64 * 3.28084 / 256.0; // converting meters to feet

        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )
        .unwrap();

        println!(
            "Latitude: {}, Longitude: {} | UNIT: {} (2 = metric)",
            lat, lng, si_unit
        );
        println!("GS: {} knot | TAS: {} knot | IAS: {} knot", gs, tas, ias);
        println!(
            "Baro: {} ft | Altitude: {} MSL | Ground: {} MSL",
            alt, hgt, ground_elevation
        );

        thread::sleep(time::Duration::from_millis(500));
    }
}

fn get_version(fsuipc: &mut Fsuipc<'_>) -> String {
    let mut fsuipc_ver = 0_u32;
    fsuipc.read(0x3304, &mut fsuipc_ver).unwrap();
    fsuipc.process().unwrap();

    format!(
        "{:x}.{:x}.{:x}{:x}",
        (0x0f & (fsuipc_ver >> 28)),
        (0x0f & (fsuipc_ver >> 24)),
        (0x0f & (fsuipc_ver >> 20)),
        (0x0f & (fsuipc_ver >> 16))
    )
}
