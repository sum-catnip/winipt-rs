#![feature(option_result_contains)] // its convenient ^^

pub mod settings;
pub mod options;
mod ipt;
pub use self::ipt::*;

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::raw::HANDLE;
    use winapi::um::processthreadsapi;

    #[test]
    fn can_get_ipt_buffer_version() {
        assert_ne!(0, buffer_version().unwrap());
    }

    #[test]
    fn can_get_ipt_trace_version() {
        assert_ne!(0, trace_version().unwrap());
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
        process_trace_sz(hwnd).unwrap();
    }


}
*/