pub type DWORD = ::std::os::raw::c_ulong;
pub type WORD = ::std::os::raw::c_ushort;
pub type UINT = ::std::os::raw::c_uint;
pub type BYTE = ::std::os::raw::c_uchar;

pub type ATOM = WORD;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type HANDLE = PVOID;
pub type HWND = HANDLE;

pub const FSUIPC_Error_OK: FSUIPC_Error = 0;
pub const FSUIPC_Error_OPEN: FSUIPC_Error = 1;
pub const FSUIPC_Error_NOFS: FSUIPC_Error = 2;
pub const FSUIPC_Error_REGMSG: FSUIPC_Error = 3;
pub const FSUIPC_Error_ATOM: FSUIPC_Error = 4;
pub const FSUIPC_Error_MAP: FSUIPC_Error = 5;
pub const FSUIPC_Error_VIEW: FSUIPC_Error = 6;
pub const FSUIPC_Error_VERSION: FSUIPC_Error = 7;
pub const FSUIPC_Error_WRONGFS: FSUIPC_Error = 8;
pub const FSUIPC_Error_NOTOPEN: FSUIPC_Error = 9;
pub const FSUIPC_Error_NODATA: FSUIPC_Error = 10;
pub const FSUIPC_Error_TIMEOUT: FSUIPC_Error = 11;
pub const FSUIPC_Error_SENDMSG: FSUIPC_Error = 12;
pub const FSUIPC_Error_DATA: FSUIPC_Error = 13;
pub const FSUIPC_Error_RUNNING: FSUIPC_Error = 14;
pub const FSUIPC_Error_SIZE: FSUIPC_Error = 15;
pub const FSUIPC_Error_NOPERMISSION: FSUIPC_Error = 16;
pub type FSUIPC_Error = ::std::os::raw::c_uint;
pub const FSUIPC_Simulator_ANY: FSUIPC_Simulator = 0;
pub const FSUIPC_Simulator_FS98: FSUIPC_Simulator = 1;
pub const FSUIPC_Simulator_FS2K: FSUIPC_Simulator = 2;
pub const FSUIPC_Simulator_CFS2: FSUIPC_Simulator = 3;
pub const FSUIPC_Simulator_CFS1: FSUIPC_Simulator = 4;
pub const FSUIPC_Simulator_FLY: FSUIPC_Simulator = 5;
pub const FSUIPC_Simulator_FS2K2: FSUIPC_Simulator = 6;
pub const FSUIPC_Simulator_FS2K4: FSUIPC_Simulator = 7;
pub const FSUIPC_Simulator_FSX: FSUIPC_Simulator = 8;
pub const FSUIPC_Simulator_ESP: FSUIPC_Simulator = 9;
pub const FSUIPC_Simulator_P3D: FSUIPC_Simulator = 10;
pub const FSUIPC_Simulator_FSX64: FSUIPC_Simulator = 11;
pub const FSUIPC_Simulator_P3D64: FSUIPC_Simulator = 12;
pub const FSUIPC_Simulator_MSFS: FSUIPC_Simulator = 13;
pub type FSUIPC_Simulator = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug)]
pub struct std_vector {
    _dummy_1: u64,
    _dummy_2: u64,
    _dummy_3: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct FSUIPC_IPCUser {
    pub Version: DWORD,
    pub FSVersion: DWORD,
    pub LibVersion: DWORD,
    pub windowHandle: HWND,
    pub msgId: UINT,
    pub atom: ATOM,
    pub mapHandle: HANDLE,
    pub viewPointer: *mut BYTE,
    pub nextPointer: *mut BYTE,
    pub destinations: std_vector,
}

extern "C" {
    #[link_name = "\u{1}?Open@IPCUser@FSUIPC@@QEAA_NW4Simulator@2@PEAW4Error@2@@Z"]
    pub fn FSUIPC_IPCUser_Open(
        this: *mut FSUIPC_IPCUser,
        requestedVersion: FSUIPC_Simulator,
        result: *mut FSUIPC_Error,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?Close@IPCUser@FSUIPC@@QEAAXXZ"]
    pub fn FSUIPC_IPCUser_Close(this: *mut FSUIPC_IPCUser);
}
extern "C" {
    #[link_name = "\u{1}?Write@IPCUser@FSUIPC@@QEAA_NKKPEAXPEAW4Error@2@@Z"]
    pub fn FSUIPC_IPCUser_Write(
        this: *mut FSUIPC_IPCUser,
        offset: DWORD,
        size: DWORD,
        src: *mut ::std::os::raw::c_void,
        result: *mut FSUIPC_Error,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?Process@IPCUser@FSUIPC@@QEAA_NPEAW4Error@2@@Z"]
    pub fn FSUIPC_IPCUser_Process(this: *mut FSUIPC_IPCUser, result: *mut FSUIPC_Error) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?ReadCommon@IPCUser@FSUIPC@@QEAA_N_NKKPEAXPEAW4Error@2@@Z"]
    pub fn FSUIPC_IPCUser_ReadCommon(
        this: *mut FSUIPC_IPCUser,
        special: bool,
        offset: DWORD,
        size: DWORD,
        dest: *mut ::std::os::raw::c_void,
        result: *mut FSUIPC_Error,
    ) -> bool;
}
impl FSUIPC_IPCUser {
    #[inline]
    pub unsafe fn Open(
        &mut self,
        requestedVersion: FSUIPC_Simulator,
        result: *mut FSUIPC_Error,
    ) -> bool {
        FSUIPC_IPCUser_Open(self, requestedVersion, result)
    }
    #[inline]
    pub unsafe fn Close(&mut self) {
        FSUIPC_IPCUser_Close(self)
    }
    #[inline]
    pub unsafe fn Write(
        &mut self,
        offset: DWORD,
        size: DWORD,
        src: *mut ::std::os::raw::c_void,
        result: *mut FSUIPC_Error,
    ) -> bool {
        FSUIPC_IPCUser_Write(self, offset, size, src, result)
    }
    #[inline]
    pub unsafe fn Process(&mut self, result: *mut FSUIPC_Error) -> bool {
        FSUIPC_IPCUser_Process(self, result)
    }
    #[inline]
    pub unsafe fn ReadCommon(
        &mut self,
        special: bool,
        offset: DWORD,
        size: DWORD,
        dest: *mut ::std::os::raw::c_void,
        result: *mut FSUIPC_Error,
    ) -> bool {
        FSUIPC_IPCUser_ReadCommon(self, special, offset, size, dest, result)
    }
}
