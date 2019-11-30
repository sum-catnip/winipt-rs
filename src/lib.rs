#![feature(option_result_contains)] // its convenient ^^

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::raw::HANDLE;
    use winapi::um::processthreadsapi;

    #[test]
    fn can_get_ipt_buffer_version() {
        assert_ne!(0, ipt_buffer_version().unwrap());
    }

    #[test]
    fn can_get_ipt_trace_version() {
        assert_ne!(0, ipt_trace_version().unwrap());
    }

    #[test]
    fn can_enable_ipt() {
        enable_ipt().unwrap();
    }

    // I don't think this test works lmao, can't find fields - it'll be fixed by wrapper
    // #[test]
    // fn start_process_tracing() {
    //     let hwnd: HANDLE;
    //     let opt: winipt_sys::IPT_OPTIONS; // Most of these opts come from https://github.com/ionescu007/winipt/blob/master/ipttool/ipttool.c
    //     opt.OptionVersion = 1;
    //     opt.TimingSettings = 2;
    //     opt.MtcFrequency = 3;
    //     opt.CycThreshold = 1;
    //     opt.TopaPagesPow2 = 128; // 128 B
    //     opt.MatchSettings = 0;
    //     opt.Inherit = 0;
    //     opt.ModeSettings = 4 ;
    //     opt.Reserved = 0;
    //     unsafe { hwnd = processthreadsapi::GetCurrentProcess() as HANDLE; }
    //     start_process_tracing(hwnd, opt)
    // }

    // this is will fail cuz there is no proccess trace running for the current proc
    // to make this work well need to bind the startprocesstracing func
    #[test]
    fn get_process_trace_sz() {
        // ipt_process_trace_sz(child);
        let hwnd: HANDLE;
        unsafe { hwnd = processthreadsapi::GetCurrentProcess() as HANDLE; }
        ipt_process_trace_sz(hwnd).unwrap();
    }


}

mod bindings {
    use std::os::windows::raw::HANDLE;
    use std::ffi::c_void;
    use modular_bitfield::prelude::*;

    // i have no idea if this works
    #[bitfield]
    #[derive(Debug, PartialEq, Eq)]
    struct IPT_OPTIONS {
        OptionVersion: B4,
        TimingSettings: B4,

        MtcFrequency: B4,
        CycThreshold: B4,

        TopaPagesPow2: B4,
        MatchSettings: B3,
        Inherit: bool,

        ModeSettings: B4,
        Reserved: B36
    }

    // the raw function bindings go here
    // those names need to be the same as in the library were binding
    // not all of em are tested as u can see so maybe i fucked on up
    // all of those need to be wrapped in a rust like way
    // examples of that are below
    extern {
        //pub fn GetIptTraceVersion(version: *mut u32) -> bool;
        //pub fn GetProcessIptTraceSize(proc: HANDLE, sz: *mut u32) -> bool;
        //pub fn GetProcessIptTrace(proc: HANDLE, trace: *mut c_void, sz: u32) -> bool;
        //pub fn StartProcessIptTracing(proc: HANDLE, opt: IPT_OPTIONS) -> bool;
        //pub fn StopProcessIptTracing(proc: HANDLE) -> bool;
        //pub fn StartCoreIptTracing(opt: IPT_OPTIONS, tries: u32, duration: u32) -> bool;
        pub fn PauseThreadIptTracing(thread: HANDLE, res: *mut bool) -> bool;
        pub fn ResumeThreadIptTracing(thread: HANDLE, res: *mut bool) -> bool;
        pub fn QueryProcessIptTracing(proc: HANDLE, opt: *mut IPT_OPTIONS) -> bool;
        pub fn QueryCoreIptTracing(opt: *mut IPT_OPTIONS) -> bool;
        pub fn RegisterExtendedImageForIptTracing(
            img_path: *const u16,
            filtered_path: *const u16,
            opt: IPT_OPTIONS,
            tries: u32,
            duration: u32
        ) -> bool;
    }
}

use winapi::um::winsvc;
use std::vec::Vec;
use std::ptr::{null, null_mut};
use std::io::Error;
use std::ffi::OsStr;
use std::ffi::c_void;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::raw::HANDLE;
use winipt_sys;

// if the param is false this will returns the last os error
// used so i can ch_last_error(res)?
// which will abort the current function and just return the error
fn ch_last_error(condition: bool) -> Result<(), Error> {
    match condition {
        false => Err(Error::last_os_error()),
        true => Ok(())
    }
}

// this is not a binding
// i took this from the example program, to enable the driver
pub fn enable_ipt() -> Result<(), Error> {
    unsafe {
        let mgr = winsvc::OpenSCManagerW(
            null(), null(), winsvc::SC_MANAGER_CONNECT
        );

        ch_last_error(mgr != null_mut())?;
        let ipt_wstr: Vec<u16> = OsStr::new("Ipt")
            .encode_wide()
            .chain(once(0))
            .collect();

        let srv = winsvc::OpenServiceW(
            mgr, ipt_wstr.as_ptr(), winsvc::SERVICE_START
        );
        ch_last_error(srv != null_mut())?;
        let started = winsvc::StartServiceW(srv, 0, null_mut());
        winsvc::CloseServiceHandle(srv);

        // ERROR_SERVICE_ALREADY_RUNNING
        // it was unsigned in the api so i just made a const
        const ALREADY_RUNNING: i32 = 1056;
        match ch_last_error(started > 0) {
            Err(x) if x.raw_os_error().contains(&ALREADY_RUNNING) => Ok(()),
            //Err(x) if x.raw_os_error().contains(&ALREADY_RUNNING) => Ok(()),
            y => y
        }?;
    }

    Ok(())
}

pub fn ipt_buffer_version() -> Result<u32, Error> {
    let mut ver: u32 = 0;
    let res: i32;
    unsafe { res = winipt_sys::GetIptBufferVersion(&mut ver); }
    ch_last_error(res > 0)?;
    Ok(ver)
}

pub fn ipt_trace_version() -> Result<u16, Error> {
    let mut ver: u16 = 0;
    let res: i32;
    unsafe { res = winipt_sys::GetIptTraceVersion(&mut ver); }
    ch_last_error(res > 0)?;
    Ok(ver)
}

pub fn ipt_process_trace_sz(proc: HANDLE) -> Result<u32, Error> {
    let mut sz: u32 = 0;
    let res: i32;
    unsafe { res = winipt_sys::GetProcessIptTraceSize(proc, &mut sz); }
    ch_last_error(res > 0)?;
    Ok(sz)
}

pub fn ipt_process_trace(proc: HANDLE, buf: &mut [u8]) -> Result<(), Error> {
    let res: i32;
    unsafe { res = winipt_sys::GetProcessIptTrace(
        proc, buf.as_mut_ptr() as *mut c_void, buf.len() as u32
    );}
    ch_last_error(res > 0)?;
    Ok(())
}

// We will use the IPT_OPTIONS union wrapper instead
pub fn start_process_tracing(proc: HANDLE, opt: winipt_sys::IPT_OPTIONS)
    -> Result<(), Error> {
    let res: i32;
    unsafe { res = winipt_sys::StartProcessIptTracing(proc, opt) };
    ch_last_error(res > 0)?;
    Ok(())
}

pub fn stop_process_tracing(proc: HANDLE) -> Result<(), Error> {
    let res: i32;
    unsafe { res = winipt_sys::StopProcessIptTracing(proc); }
    ch_last_error(res > 0)?;
    Ok(())
}

// We will use the IPT_OPTIONS union wrapper instead
pub fn start_core_process_tracing(opt: winipt_sys::IPT_OPTIONS) -> Result<(), Error> {
    let res: i32;
    let tries: u32 = 3; // need to find how many tries to set
    let duration: u32 = 60; // need to find what duration to set (in seconds)
    unsafe { res = winipt_sys::StartCoreIptTracing(opt, tries, duration); }
    ch_last_error(res > 0)?;
    Ok(())
}

// pub fn pause_thread_process_tracing(thread: HANDLE) -> Result<bool, Error> {
//     let mut res: i32;
//     let pbres: bool;
//     unsafe { res = winipt_sys::PauseThreadIptTracing(thread, &mut pbres); } // expects pbres to be a u8, and not a bool
//     ch_last_error(res > 0)?;
//     Ok(pbres)
// }

// pub fn resume_thread_process_tracing(thread: HANDLE) -> Result<bool, Error> {
//     let mut res: i32;
//     let pbres: bool;
//     unsafe { res = winipt_sys::ResumeThreadIptTracing(thread, &mut pbres); } // expects pbres to be a u8, and not a bool
//     ch_last_error(res > 0)?;
//     Ok(pbres)
// }

// We will use the IPT_OPTIONS union wrapper instead
// pub fn query_process_tracing(proc: HANDLE) -> Result<winipt_sys::IPT_OPTIONS, Error> {
//     let res: i32;
//     let mut opt: winipt_sys::IPT_OPTIONS;
//     unsafe { res = winipt_sys::QueryProcessIptTracing(proc, &mut opt); }
//     ch_last_error(res > 0)?;
//     Ok(opt)
// }

// We will use the IPT_OPTIONS union wrapper instead
// pub fn query_core_process_tracing() -> Result<winipt_sys::IPT_OPTIONS, Error> {
//     let mut res: i32;
//     let mut opt: winipt_sys::IPT_OPTIONS;
//     unsafe { res = winipt_sys::QueryCoreIptTracing(&mut opt); }
//     ch_last_error(res > 0)?;
//     Ok(opt)
// }

// TODO: This binding feels wrong... I don't know what it does or why img_path is an int
// We will use the IPT_OPTIONS union wrapper instead
// pub fn register_extended_image(opt: winipt_sys::IPT_OPTIONS) -> Result<(), Error> {
//     let img_path: u16 = 0;
//     let filtered_path: u16 = 0;
//     let mut tries: u32 = 0;
//     let mut duration: u32 = 0;
//     let res: i32;
//     unsafe { res = winipt_sys::RegisterExtendedImageForIptTracing(
//         &img_path, &filtered_path, &mut tries, &mut duration); } 
//     ch_last_error(res > 0)?;
//     Ok(())
// }