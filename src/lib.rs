#![feature(option_result_contains)] // its convenient ^^

pub mod settings;
mod options;
mod ipt;
pub use self::ipt::*;
pub use self::options::Options;

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::raw::HANDLE;
    use winapi::um::processthreadsapi;
    use std::panic;

    #[test]
    fn libipt_available() {
        enable_ipt().expect(concat!(
            "failed to enable intelpt driver! ",
            "make sure your system has support for the ipt driver"))
    }

    #[test]
    fn valid_buffer_version() {
        assert_ne!(0, buffer_version().unwrap());
    }

    #[test]
    fn valid_trace_version() {
        assert_ne!(0, trace_version().unwrap());
    }

    #[test]
    fn record_process_trace() {
        // lets record this process
        let hwnd: HANDLE;
        unsafe { hwnd = processthreadsapi::GetCurrentProcess() as HANDLE; }
        let opt = Options::new();
        // configure buffer and trace
    }

    // this is will fail cuz there is no proccess trace running for the current proc
    // to make this work well need to bind the startprocesstracing func
    #[test]
    fn get_process_trace_sz() {
        // ipt_process_trace_sz(child);
        let hwnd: HANDLE;
        unsafe { hwnd = processthreadsapi::GetCurrentProcess() as HANDLE; }
        process_trace_sz(hwnd).unwrap();
    }


}