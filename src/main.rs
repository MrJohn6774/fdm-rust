use std::{thread, time};

use fdm::fdm::FlightDataMonitoring;
use fsuipc::Fsuipc;

pub mod fdm;
pub mod fsuipc;

fn main() {
    {
        let mut fsuipc = Fsuipc::new();
        fsuipc.connect().unwrap();

        println!("Version: {}", get_version(&mut fsuipc));
        println!("{}", get_aircraft_name(&mut fsuipc));
    }

    thread::sleep(time::Duration::from_secs(2));

    let mut fdm = FlightDataMonitoring::new();

    fdm.run().unwrap();
}

fn get_version(fsuipc: &mut Fsuipc) -> String {
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

fn get_aircraft_name(fsuipc: &mut Fsuipc) -> String {
    let mut afc_title_raw = [0_u8; 256];
    let mut afc_type_raw = [0_u8; 24];
    let mut afc_livery_raw = [0_u8; 24];

    fsuipc.read(0x3D00, &mut afc_title_raw).unwrap();
    fsuipc.read(0x3160, &mut afc_type_raw).unwrap();
    fsuipc.read(0x3148, &mut afc_livery_raw).unwrap();
    fsuipc.process().unwrap();

    let afc_livery = get_string_from_bytes(&afc_livery_raw)
        .expect("Failed to convert ATC airline name to UTF-8");

    let afc_type =
        get_string_from_bytes(&afc_type_raw).expect("Failed to convert ATC aircraft type to UTF-8");

    let afc_title =
        get_string_from_bytes(&afc_title_raw).expect("Failed to convert aircraft name to UTF-8");

    format!(
        "0x3D00: {} | 0x3160: {} | 0x3148: {}",
        afc_title, afc_type, afc_livery
    )
}

fn get_string_from_bytes(bytes: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    let null_index = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let valid_bytes = &bytes[..null_index];
    String::from_utf8(valid_bytes.to_vec())
}
