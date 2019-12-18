#![feature(option_result_contains)] // its convenient ^^

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
use bitmask::bitmask;
use winipt_sys::{
    IPT_OPTIONS,
    IPT_TIMING_SETTINGS_IptEnableCycPackets,
    IPT_TIMING_SETTINGS_IptEnableMtcPackets,
    IPT_TIMING_SETTINGS_IptNoTimingPackets,

    _IPT_MATCH_SETTINGS_IptMatchByAnyApp,
    _IPT_MATCH_SETTINGS_IptMatchByAnyPackage,
    _IPT_MATCH_SETTINGS_IptMatchByImageFileName,
    _IPT_MATCH_SETTINGS_IptMatchByPackageName,

    _IPT_MODE_SETTINGS_IptCtlKernelModeOnly,
    _IPT_MODE_SETTINGS_IptCtlUserAndKernelMode,
    _IPT_MODE_SETTINGS_IptCtlUserModeOnly,
    _IPT_MODE_SETTINGS_IptRegKernelModeOnly,
    _IPT_MODE_SETTINGS_IptRegUserAndKernelMode,
    _IPT_MODE_SETTINGS_IptRegUserModeOnly
};

bitmask! {
    pub mask TimingSettings : i32 where flags TimingSetting {
        EnableCycPackets = IPT_TIMING_SETTINGS_IptEnableCycPackets,
        EnableMtcPackets = IPT_TIMING_SETTINGS_IptEnableMtcPackets,
        NoTimingPackets  = IPT_TIMING_SETTINGS_IptNoTimingPackets
    }
}

bitmask! {
    pub mask MatchSettings : i32 where flags MatchSetting {
        MatchByAnyApp        = _IPT_MATCH_SETTINGS_IptMatchByAnyApp,
        MatchByAnyPackage    = _IPT_MATCH_SETTINGS_IptMatchByAnyPackage,
        MatchByImageFileName = _IPT_MATCH_SETTINGS_IptMatchByImageFileName,
        MatchByPackageName   = _IPT_MATCH_SETTINGS_IptMatchByPackageName
    }
}

bitmask! {
    pub mask ModeSettings : i32 where flags ModeSetting {
        CtlKernelModeOnly    = _IPT_MODE_SETTINGS_IptCtlKernelModeOnly,
        CtlUserAndKernelMode = _IPT_MODE_SETTINGS_IptCtlUserAndKernelMode,
        CtlUserModeOnly      = _IPT_MODE_SETTINGS_IptCtlUserModeOnly,
        RegKernelModeOnly    = _IPT_MODE_SETTINGS_IptRegKernelModeOnly,
        RegUserAndKernelMode = _IPT_MODE_SETTINGS_IptRegUserAndKernelMode,
        RegUserModeOnl       = _IPT_MODE_SETTINGS_IptRegUserModeOnly
    }
}

/// a wrapper around IPT_OPTIONS
/// it contains all of the options used to manipulate the intelpt driver
pub struct Options (IPT_OPTIONS);
impl Options {
    /// creates a new IPT_OPTIONS struct
    /// and initializes it with zeroes
    /// OptionVersion will be set to 1 tho
    pub fn new() -> Self {
        let mut o = Options(IPT_OPTIONS{AsULonglong: 0});
        o.set_option_version(1);
        o
    }

    /// Must be set to 1 (will be by default)
    pub fn option_version(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.OptionVersion() }
    }

    /// Must be set to 1 (will be by deault)
    pub fn set_option_version(&mut self, version: u64) {
        unsafe { self.0.__bindgen_anon_1.set_OptionVersion(version) }
    }

    // TODO find a way to turn this u64 into a TimingSettings
    /// gets IPT_TIMING_SETTINGS
    pub fn timing_settings(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.TimingSettings() }
    }

    /// sets IPT_TIMING_SETTINGS
    pub fn set_timing_settings(&mut self, settings: TimingSettings) {
        // the original bitfield specified 4 bytes so were fine
        unsafe { self.0.__bindgen_anon_1.set_TimingSettings(*settings as u64) }
    }

    /// gets Bits 14:17 in IA32_RTIT_CTL
    pub fn mtc_frequency(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.MtcFrequency() }
    }

    /// sets Bits 14:17 in IA32_RTIT_CTL
    pub fn set_mtc_frequency(&mut self, freq: u64) {
        unsafe { self.0.__bindgen_anon_1.set_MtcFrequency(freq) }
    }

    /// gets Bits 19:22 in IA32_RTIT_CTL
    pub fn cyc_threshold(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.CycThreshold() }
    }

    /// sets Bits 19:22 in IA32_RTIT_CTL
    pub fn set_cyc_threshold(&mut self, threshold: u64) {
        unsafe { self.0.__bindgen_anon_1.set_CycThreshold(threshold) }
    }

    /// gets the Size of buffer in ToPA, as 4KB powers of 2 (4KB->128MB). Multiple buffers will be used if CPUID.(EAX=014H,ECX=0H):ECX[1]= 1
    pub fn topa_pages_pow2(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.TopaPagesPow2() }
    }

    /// sets the Size of buffer in ToPA, as 4KB powers of 2 (4KB->128MB). Multiple buffers will be used if CPUID.(EAX=014H,ECX=0H):ECX[1]= 1
    pub fn set_topa_pages_pow2(&mut self, size: u64) {
        unsafe { self.0.__bindgen_anon_1.set_TopaPagesPow2(size) }
    }

    /// get IPT_MATCH_SETTINGS
    pub fn match_settings(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.MatchSettings() }
    }

    /// set IPT_MATCH_SETTINGS
    pub fn set_match_settings(&mut self, settings: MatchSettings) {
        // IPT_MATCH_SETTINGS is 3 bytes so casting here is fine
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(*settings as u64) }
    }

    /// if children will be automatically added to the trace
    pub fn inherit(&self) -> bool {
        unsafe { self.0.__bindgen_anon_1.Inherit() > 0 }
    }

    // if children will be automatically added to the trace
    pub fn set_inherit(&mut self, inherit: bool) {
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(inherit as u64) }
    }

    /// get IPT_MODE_SETTINGS
    pub fn mode_settings(&self) -> u64 {
        unsafe { self.0.__bindgen_anon_1.ModeSettings() }
    }

    /// set IPT_MODE_SETTINGS
    pub fn set_mode_settings(&mut self, settings: ModeSettings) {
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(*settings as u64) }
    }

    /// returns the original IPT_OPTIONS struct
    /// internal use only
    fn wrapped(&self) -> IPT_OPTIONS { self.0 }

    /// get mutable reference to inner IPT_OPTIONS struct
    /// internal use only
    fn wrapped_mut(&mut self) -> &mut IPT_OPTIONS { &mut self.0 }
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

pub fn process_trace_sz(proc: HANDLE) -> Result<u32, Error> {
    let mut sz: u32 = 0;
    let res: i32;
    unsafe { res = winipt_sys::GetProcessIptTraceSize(proc, &mut sz); }
    ch_last_error(res > 0)?;
    Ok(sz)
}

pub fn process_trace(proc: HANDLE, buf: &mut [u8]) -> Result<(), Error> {
    let res: i32;
    unsafe { res = winipt_sys::GetProcessIptTrace(
        proc, buf.as_mut_ptr() as *mut c_void, buf.len() as u32
    );}
    ch_last_error(res > 0)?;
    Ok(())
}

// We will use the IPT_OPTIONS union wrapper instead
pub fn start_process_tracing(proc: HANDLE, opt: Options)
    -> Result<(), Error> {
    let res: i32;
    unsafe { res = winipt_sys::StartProcessIptTracing(proc, opt.wrapped()) };
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
pub fn start_core_tracing(opt: Options) -> Result<(), Error> {
    let res: i32;
    let tries: u32 = 3; // need to find how many tries to set
    let duration: u32 = 60; // need to find what duration to set (in seconds)
    unsafe { 
        res = winipt_sys::StartCoreIptTracing(opt.wrapped(), tries, duration);
    }
    ch_last_error(res > 0)?;
    Ok(())
}

pub fn pause_thread_tracing(thread: HANDLE) -> Result<bool, Error> {
    let res: i32;
    let mut pbres: u8 = 0;
    unsafe { res = winipt_sys::PauseThreadIptTracing(thread, &mut pbres); }
    ch_last_error(res > 0)?;
    Ok(pbres > 0)
}

pub fn resume_thread_tracing(thread: HANDLE) -> Result<bool, Error> {
    let res: i32;
    let mut pbres: u8 = 0;
    unsafe { res = winipt_sys::ResumeThreadIptTracing(thread, &mut pbres); }
    ch_last_error(res > 0)?;
    Ok(pbres > 0)
}

// i have no idea what this does or if it works
pub fn query_process_tracing(proc: HANDLE) -> Result<Options, Error> {
    let res: i32;
    // using the integer union field to initialize everything to 0
    let mut opt = Options::new();
    unsafe { 
        res = winipt_sys::QueryProcessIptTracing(proc, opt.wrapped_mut());
    }
    ch_last_error(res > 0)?;
    Ok(opt)
}

pub fn query_core_tracing() -> Result<Options, Error> {
    let res: i32;
    // using the integer union field to initialize everything to 0
    let mut opt = Options::new();
    unsafe { res = winipt_sys::QueryCoreIptTracing(opt.wrapped_mut()); }
    ch_last_error(res > 0)?;
    Ok(opt)
}

// we will use the IPT_OPTIONS union wrapper instead
// i have no idea what this even does tbh
pub fn register_extended_image(
    img_path: &str, filtered_path: &str,
    opt: Options, tries: u32,
    duration: u32)
    -> Result<(), Error> {

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
        img_path_raw, filtered_path_raw, opt.wrapped(), tries, duration
    );} 
    ch_last_error(res > 0)?;
    Ok(())
}