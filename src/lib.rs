#![feature(option_result_contains)] // its convenient ^^

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_get_ipt_buffer_version() {
        assert_ne!(0, ipt_buffer_version().unwrap());
    }

    #[test]
    fn can_start_ipt() {
        enable_ipt().unwrap();
    }

    #[test]
    fn kektop() {
        enable_ipt().unwrap();
        assert_ne!(0, ipt_buffer_version().unwrap());
    }
}

mod bindings {
    extern {
        pub fn GetIptBufferVersion(version: *mut u32) -> bool;
    }
}

use winapi::um::winsvc;
use std::ptr::{null, null_mut};
use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

fn ch_last_error(condition: bool) -> Result<(), Error> {
    match condition {
        false => Err(Error::last_os_error()),
        true => Ok(())
    }
}

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
    let res: bool;
    unsafe { res = bindings::GetIptBufferVersion(&mut ver); }
    ch_last_error(res)?;
    Ok(ver)
}
