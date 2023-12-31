#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::mem::size_of;

include!("../libuipc/libuipc.rs");

pub type FsuipcResult<T> = Result<T, (u8, String)>;

#[derive(Debug)]
pub struct FsuipcData<ReturnDataType, FsuipcDataType> {
    pub offset: u16,
    pub data: ReturnDataType,
    pub raw_data: FsuipcDataType,
}

impl<ReturnDataType, FsuipcDataType> FsuipcData<ReturnDataType, FsuipcDataType> {
    pub fn read_raw(&mut self, fsuipc: &mut Fsuipc) -> FsuipcResult<()> {
        fsuipc.read(self.offset, &mut self.raw_data)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Fsuipc {
    pub is_connected: bool,
    errMsg: [String; 16],
    ipc: Box<FSUIPC_IPCUser>,
}

impl Fsuipc {
    pub fn new() -> Self {
        let ipc = Box::new(FSUIPC_IPCUser {
            Version: 0,
            FSVersion: 0,
            LibVersion: 2002,
            windowHandle: std::ptr::null_mut(),
            msgId: 0,
            atom: 0,
            mapHandle: std::ptr::null_mut(),
            viewPointer: std::ptr::null_mut(),
            nextPointer: std::ptr::null_mut(),
            destinations: unsafe { std::mem::zeroed::<std_vector>() },
        });
        Fsuipc {
            is_connected: false,
            errMsg: [
                "Okay".to_owned(),
                "Attempt to Open when already Open".to_owned(),
                "Cannot link to FSUIPC or WideClient".to_owned(),
                "Failed to Register common message with Windows".to_owned(),
                "Failed to create Atom for mapping filename".to_owned(),
                "Failed to create a file mapping object".to_owned(),
                "Failed to open a view to the file map".to_owned(),
                "Incorrect version of FSUIPC, or not FSUIPC".to_owned(),
                "Sim is not version requested".to_owned(),
                "Call cannot execute, link not Open".to_owned(),
                "Call cannot execute: no requests accumulated".to_owned(),
                "IPC timed out all retries".to_owned(),
                "IPC sendmessage failed all retries".to_owned(),
                "IPC request contains bad data".to_owned(),
                "Maybe running on WideClient, but FS not running on Server, or wrong FSUIPC"
                    .to_owned(),
                "Read or Write request cannot be added, memory for Process is full".to_owned(),
            ],
            ipc,
        }
    }

    pub fn connect(&mut self) -> FsuipcResult<()> {
        let mut result: FSUIPC_Error = 0;
        let return_value = unsafe {
            self.ipc
                .Open(FSUIPC_Simulator_ANY, &mut result as *mut FSUIPC_Error)
        };
        if return_value || result == 1 {
            println!("Connected");
            self.is_connected = true;
            Ok(())
        } else {
            self.is_connected = false;
            Err((result as u8, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn test_connection(&mut self) -> FsuipcResult<()> {
        self.connect()?;
        self.close();
        Ok(())
    }

    pub fn read<T: Sized>(&mut self, offset: u16, value: &mut T) -> FsuipcResult<()> {
        let mut result: FSUIPC_Error = 0;
        // let size: DWORD = (size_of(value)).into() * 8;
        let return_value = unsafe {
            self.ipc.ReadCommon(
                false,
                offset as DWORD,
                size_of::<T>() as DWORD,
                value as *mut T as *mut std::os::raw::c_void,
                &mut result as *mut FSUIPC_Error,
            )
        };
        if return_value {
            Ok(())
        } else {
            self.is_connected = false;
            Err((result as u8, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn process(&mut self) -> FsuipcResult<()> {
        let mut result: FSUIPC_Error = 0;
        let return_value = unsafe { self.ipc.Process(&mut result as *mut FSUIPC_Error) };
        if return_value {
            Ok(())
        } else {
            self.is_connected = false;
            Err((result as u8, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn close(&mut self) {
        if self.is_connected {
            unsafe { self.ipc.Close() };
            println!("Closed");
            self.is_connected = false;
        }
    }
}

impl Drop for Fsuipc {
    fn drop(&mut self) {
        self.close();
    }
}
