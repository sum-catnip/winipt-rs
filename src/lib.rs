pub mod settings;
mod options;
pub use self::options::*;
mod process;
pub use process::ProcessTracer;
mod thread;
pub use thread::ThreadTracer;
mod core;
pub use crate::core::CoreTracer;

use std::io::Error;
use std::vec::Vec;
use std::ptr::{ null, null_mut };
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

use winapi::um::winsvc;

/// Enable the libipt windows driver if not already enabled
///
/// This is not a binding
/// I took this from the example program, to enable the driver
/// It returns an ALREADY_RUNNING error if the driver is already running (duh)
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

        ch_last_error(started > 0)
    }
}

pub fn buffer_version() -> Result<u32, Error> {
    let mut ver: u32 = 0;
    let res: i32;
    unsafe { res = winipt_sys::GetIptBufferVersion(&mut ver); }
    ch_last_error(res > 0)?;
    Ok(ver)
}

pub fn trace_version() -> Result<u16, Error> {
    let mut ver: u16 = 0;
    let res: i32;
    unsafe { res = winipt_sys::GetIptTraceVersion(&mut ver); }
    ch_last_error(res > 0)?;
    Ok(ver)
}

// we will use the IPT_OPTIONS union wrapper instead
// i have no idea what this even does tbh
pub fn register_extended_image(img_path: &str, filtered_path: &str,
                               opt: Options, tries: u32,
                               duration: u32) -> Result<(), Error> {

    let img_path_raw = OsStr::new(img_path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
        .as_mut_ptr();

    let filtered_path_raw = OsStr::new(filtered_path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
        .as_mut_ptr();

    let res: i32;
    unsafe { res = winipt_sys::RegisterExtendedImageForIptTracing(
        img_path_raw, filtered_path_raw, opt.0, tries, duration
    );} 
    ch_last_error(res > 0)?;
    Ok(())
}
// if the param is false this will returns the last os error
// used so i can ch_last_error(res)?
// which will abort the current function and just return the error
fn ch_last_error(condition: bool) -> Result<(), Error> {
    match condition {
        false => Err(Error::last_os_error()),
        true => Ok(())
    }
}

/// Make sure your systems libipt driver is running before running the tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::raw::HANDLE;
    use winapi::um::processthreadsapi;
    use std::panic;
    use self::settings::*;

    #[test]
    fn valid_buffer_version() {
        assert_ne!(0, buffer_version().unwrap());
    }

    #[test]
    fn valid_trace_version() {
        assert_ne!(0, trace_version().unwrap());
    }

    #[test]
    fn record_process_trace_userland_notiming() {
        // lets record this process
        let hwnd: HANDLE;
        unsafe { hwnd = processthreadsapi::GetCurrentProcess() as HANDLE; }
        let opt = OptionsBuilder::new()
            .topa_pages_pow2(4)
            .match_settings(MatchSettings::MATCH_BY_ANY_APP)
            .timing_settings(TimingSettings::NO_TIMING_PACKETS)
            .mode_settings(ModeSettings::CTL_USERMODE_ONLY)
            .finish();

        // start the trace
        let tracer = ProcessTracer::new_start(hwnd, opt).unwrap();
        // grab the trace
        assert_ne!(tracer.size().unwrap(), 0);
        let trace = tracer.trace().unwrap();
        assert_ne!(0, trace.len());
        println!("process trace: {:?}", trace);
        // stop the trace
        tracer.stop().unwrap();
    }
}