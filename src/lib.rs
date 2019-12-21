#![feature(option_result_contains)] // its convenient ^^

pub mod settings;
mod options;
mod ipt;
pub use self::ipt::*;
pub use self::options::Options;

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::io::AsRawHandle;
    use std::process::Command;
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