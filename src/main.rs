use crossterm::{
    execute,
    terminal::{self, ClearType},
};
use std::io;
use std::{thread, time};

pub mod fsuipc;

use crate::fsuipc::Fsuipc;

fn main() {
    let fsuipc = Fsuipc::new();
    fsuipc.connect().unwrap();

    println!("Version: {}", get_version(&fsuipc));

    // exit(0);

    loop {
        let mut lat_raw = 0_u64;
        let mut lng_raw = 0_u64;
        let mut spd_raw = 0_u32;
        let mut alt_raw = 0_u32;
        let mut hgt_raw = 0_u32;
        fsuipc.read(0x0560, &mut lat_raw).unwrap();
        fsuipc.read(0x0568, &mut lng_raw).unwrap();
        fsuipc.read(0x02B4, &mut spd_raw).unwrap();
        fsuipc.read(0x3324, &mut alt_raw).unwrap();
        fsuipc.read(0x0020, &mut hgt_raw).unwrap();
        fsuipc.process().unwrap();

        let lat = lat_raw as f64 * (90.0 / (10001750.0 * 65536.0 * 65536.0));
        let lng = lng_raw as f64 * (360.0 / (65536_f64.powf(4.0)));
        let spd = spd_raw as f64 * 1.943844;  // converting m/s to knots
        let alt = alt_raw as f64 * 3.28084;  // converting meters to feet
        let hgt = hgt_raw as f64 * 3.28084 / 256.0;  // converting meters to feet

        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )
        .unwrap();

        println!("Latitude: {}, Longitude: {}", lat, lng);
        println!("Ground Speed: {}, Baro Altitude: {}, Height: {}", spd, alt, hgt);

        thread::sleep(time::Duration::from_millis(500));
    }
}

fn get_version(fsuipc: &Fsuipc<'_>) -> String {
    let mut fsuipc_ver = 0_u32;
    fsuipc.read(0x3304, &mut fsuipc_ver).unwrap();
    fsuipc.process().unwrap();

    format!("{:x}.{:x}.{:x}{:x}",
        (0x0f & (fsuipc_ver >> 28)),
        (0x0f & (fsuipc_ver >> 24)),
        (0x0f & (fsuipc_ver >> 20)),
        (0x0f & (fsuipc_ver >> 16))
    )
}
