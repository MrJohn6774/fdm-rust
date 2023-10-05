#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::size_of;

include!("../libfsuipc/libfsuipc.rs");

pub struct Fsuipc<'a> {
    errMsg: [&'a str; 16]
}

impl Fsuipc<'_> {
    pub fn new() -> Self {
        Fsuipc {
            errMsg: [
                "Okay",
                "Attempt to Open when already Open",
                "Cannot link to FSUIPC or WideClient",
                "Failed to Register common message with Windows",
                "Failed to create Atom for mapping filename",
                "Failed to create a file mapping object",
                "Failed to open a view to the file map",
                "Incorrect version of FSUIPC, or not FSUIPC",
                "Sim is not version requested",
                "Call cannot execute, link not Open",
                "Call cannot execute: no requests accumulated",
                "IPC timed out all retries",
                "IPC sendmessage failed all retries",
                "IPC request contains bad data",
                "Maybe running on WideClient, but FS not running on Server, or wrong FSUIPC",
                "Read or Write request cannot be added, memory for Process is full",
            ]
        }
    }
    
    pub fn connect(&self) -> Result<(), (u32, String)> {
        let mut result: DWORD = 0;
        let return_value = unsafe { FSUIPC_Open(SIM_ANY, &mut result) != 0 };
        if return_value {
            println!("Connected");
            Ok(())
        } else {
            Err((result, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn read<T: Sized>(&self, offset: u16, value: &mut T) -> Result<(), (u32, String)> {
        let mut result: DWORD = 0;
        // let size: DWORD = (size_of(value)).into() * 8;
        let return_value = unsafe { FSUIPC_Read(offset as DWORD, size_of::<T>() as DWORD, value as *mut T as *mut std::os::raw::c_void, &mut result) };
        if return_value != 0 {
            Ok(())
        } else {
            Err((result, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn process(&self) -> Result<(), (u32, String)> {
        let mut result: DWORD = 0;
        let return_value = unsafe { FSUIPC_Process(&mut result) };
        if return_value != 0 {
            Ok(())
        } else {
            Err((result, self.errMsg[result as usize].to_string()))
        }
    }
}

impl Drop for Fsuipc<'_> {
    fn drop(&mut self) {
        unsafe { FSUIPC_Close() };
    }
}