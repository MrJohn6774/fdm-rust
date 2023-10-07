#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::size_of;

include!("../libuipc/libuipc.rs");

pub struct Fsuipc<'a> {
    errMsg: [&'a str; 16],
    ipc: FSUIPC_IPCUser,
}

impl Fsuipc<'_> {
    pub fn new() -> Self {
        let ipc = FSUIPC_IPCUser {
            Version: 0,
            FSVersion: 0,
            LibVersion: 2002,
            windowHandle: std::ptr::null_mut(),
            msgId: 0,
            atom: 0,
            mapHandle: std::ptr::null_mut(),
            viewPointer: std::ptr::null_mut(),
            nextPointer: std::ptr::null_mut(),
            destinations: std_vector {
                _Mypair: std::ptr::null_mut(),
            },
        };
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
            ],
            ipc,
        }
    }

    pub fn connect(&mut self) -> Result<(), (u32, String)> {
        let mut result: FSUIPC_Error = 0;
        let return_value = unsafe {
            self.ipc
                .Open(FSUIPC_Simulator_ANY, &mut result as *mut FSUIPC_Error)
        };
        if return_value {
            println!("Connected");
            Ok(())
        } else {
            Err((result, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn read<T: Sized>(&mut self, offset: u16, value: &mut T) -> Result<(), (u32, String)> {
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
            Err((result, self.errMsg[result as usize].to_string()))
        }
    }

    pub fn process(&mut self) -> Result<(), (u32, String)> {
        let mut result: FSUIPC_Error = 0;
        let return_value = unsafe { self.ipc.Process(&mut result as *mut FSUIPC_Error) };
        if return_value {
            Ok(())
        } else {
            Err((result, self.errMsg[result as usize].to_string()))
        }
    }
}

impl Drop for Fsuipc<'_> {
    fn drop(&mut self) {
        println!("Closing connection");
        unsafe { self.ipc.Close() };
    }
}
