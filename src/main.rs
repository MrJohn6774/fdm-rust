use crossterm::{
    terminal::{self, ClearType},
    ExecutableCommand,
};
use fdm::fdm::FlightDataMonitoring;
use fsuipc::Fsuipc;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time,
};

pub mod fdm;
pub mod fsuipc;

fn main() {
    {
        let mut fsuipc = Fsuipc::new();
        fsuipc.connect().unwrap();

        println!("Version: {}", get_version(&mut fsuipc));
        println!("{}", get_aircraft_name(&mut fsuipc));
        println!("The program will continue after 2 seconds. Press 'X' to stop FDM loop")
    }

    thread::sleep(time::Duration::from_secs(2));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_fdm = stop_flag.clone();
    let stop_flag_cli = stop_flag.clone();

    handles.push(thread::spawn(move || {
        let mut fdm = FlightDataMonitoring::new(stop_flag_fdm);
        fdm.run().unwrap();
    }));

    handles.push(thread::spawn(move || {
        let mut stdout = std::io::stdout();
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();

        loop {
            if let Ok(key_event) = crossterm::event::read() {
                if let crossterm::event::Event::Key(key_event) = key_event {
                    match key_event.code {
                        crossterm::event::KeyCode::Char('X')
                        | crossterm::event::KeyCode::Char('x') => {
                            stop_flag_cli.store(true, Ordering::SeqCst);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }));

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
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
