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
    use self::settings::*;

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
    fn record_process_trace_userland_notiming() {
        // lets record this process
        let hwnd: HANDLE;
        unsafe { hwnd = processthreadsapi::GetCurrentProcess() as HANDLE; }
        let mut opt = Options::new();
        opt.set_topa_pages_pow2(4 * 1024);
        // matching doesnt apply to process tracing
        let mut settings = MatchSettings::none();
        settings.set(MatchSetting::MatchByAnyApp);
        opt.set_match_settings(settings);
        // timing settings
        let mut settings = TimingSettings::none();
        settings.set(TimingSetting::NoTimingPackets);
        opt.set_timing_settings(settings);
        // set usermode
        let mut settings = ModeSettings::none();
        settings.set(ModeSetting::CtlUserModeOnly);
        opt.set_mode_settings(settings);

        // start the trace
        start_process_tracing(hwnd, opt).unwrap();
        // grab the trace
        let mut buf = vec![0;process_trace_sz(hwnd).unwrap() as usize];
        process_trace(hwnd, &mut buf).unwrap();
        assert_ne!(0, buf.len());
        println!("process trace: {:?}", buf);
        // stop the trace
        stop_process_tracing(hwnd).unwrap();
    }
}