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
    use std::os::windows::io::AsRawHandle;
    use std::process::Command;
    use self::settings::*;

    #[test]
    fn valid_buffer_version() {
        assert_ne!(0, buffer_version().unwrap());
    }

    #[test]
    fn valid_trace_version() {
        assert_ne!(0, trace_version().unwrap());
    }

    fn assert_options_equal(opt1: &Options, opt2: &Options) {
        assert_eq!(opt1.topa_pages_pow2(), opt2.topa_pages_pow2());
        assert_eq!(opt1.timing_settings(), opt2.timing_settings());
        assert_eq!(opt1.option_version(),  opt2.option_version());
        assert_eq!(opt1.mtc_frequency(),   opt2.mtc_frequency());
        assert_eq!(opt1.mode_settings(),   opt2.mode_settings());
        assert_eq!(opt1.match_settings(),  opt2.match_settings());
        assert_eq!(opt1.inherit(),         opt2.inherit());
        assert_eq!(opt1.cyc_threshold(),   opt2.cyc_threshold());
    }

    fn userland_notiming_options() -> Options {
        let mut opt = Options::new();
        opt.set_match_settings(MatchSetting::MatchByAnyApp);
        opt.set_timing_settings(TimingSetting::NoTimingPackets);
        opt.set_mode_settings(ModeSetting::CtlUserModeOnly);
        opt
    }

    #[test]
    fn record_process_trace_userland_notiming() {
        let mut target = Command::new("testing/test_target.exe")
            .spawn()
            .expect("failed running `testing_target.exe` for testing");

        let hwnd = target.as_raw_handle();
        let opt = userland_notiming_options();

        // start the trace
        start_process_tracing(hwnd, &opt).unwrap();
        // grab the trace
        let mut buf = vec![0;process_trace_sz(hwnd).unwrap() as usize];
        process_trace(hwnd, &mut buf).unwrap();
        println!("{:?}", buf.len());
        assert_ne!(0, buf.len());
        // query the options and check if they match
        let expectopt = query_process_tracing(hwnd).unwrap();
        assert_options_equal(&opt, &expectopt);
        // stop the trace
        stop_process_tracing(hwnd).unwrap();
        target.kill().unwrap();
    }
}