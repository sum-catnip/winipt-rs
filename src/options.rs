use winipt_sys::IPT_OPTIONS;
use crate::settings::{
    TimingSettings, MatchSettings, ModeSettings,
    TimingSetting,  MatchSetting,  ModeSetting
};

// bindgen is wrong about the bitfield sizes so im casting them to prevent ub

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
    pub fn option_version(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.OptionVersion() as u32 }
    }

    /// Must be set to 1 (will be by deault)
    pub fn set_option_version(&mut self, version: u32) {
        unsafe { self.0.__bindgen_anon_1.set_OptionVersion(version as u64) }
    }

    // TODO find a way to turn this u32 into a TimingSettings
    /// gets IPT_TIMING_SETTINGS
    pub fn timing_settings(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.TimingSettings() as u32 }
    }

    /// sets IPT_TIMING_SETTINGS
    pub fn set_timing_settings(&mut self, timing: TimingSetting) {
        let mut settings = TimingSettings::none();
        settings.set(timing);
        unsafe { self.0.__bindgen_anon_1.set_TimingSettings(*settings as u64) }
    }

    /// gets Bits 14:17 in IA32_RTIT_CTL
    pub fn mtc_frequency(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.MtcFrequency() as u32 }
    }

    /// sets Bits 14:17 in IA32_RTIT_CTL
    pub fn set_mtc_frequency(&mut self, freq: u32) {
        unsafe { self.0.__bindgen_anon_1.set_MtcFrequency(freq as u64) }
    }

    /// gets Bits 19:22 in IA32_RTIT_CTL
    pub fn cyc_threshold(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.CycThreshold() as u32 }
    }

    /// sets Bits 19:22 in IA32_RTIT_CTL
    pub fn set_cyc_threshold(&mut self, threshold: u32) {
        unsafe { self.0.__bindgen_anon_1.set_CycThreshold(threshold as u64) }
    }

    /// gets the Size of buffer in ToPA, as 4KB powers of 2 (4KB->128MB).
    /// Multiple buffers will be used if CPUID.(EAX=014H,ECX=0H):ECX[1]= 1
    pub fn topa_pages_pow2(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.TopaPagesPow2() as u32 }
    }

    /// sets the Size of buffer in ToPA, as 4KB powers of 2 (4KB->128MB).
    /// Multiple buffers will be used if CPUID.(EAX=014H,ECX=0H):ECX[1]= 1
    /// 
    /// __**WARNING: I HAVE NO IDEA WHAT HAPPENS WITH UNALIGNED VALUES**__
    pub fn set_topa_pages_pow2(&mut self, size: u32) {
        unsafe { self.0.__bindgen_anon_1.set_TopaPagesPow2(size as u64) }
    }

    /// get IPT_MATCH_SETTINGS
    pub fn match_settings(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.MatchSettings() as u32 }
    }

    /// set IPT_MATCH_SETTINGS
    pub fn set_match_settings(&mut self, match_stg: MatchSetting) {
        let mut settings = MatchSettings::none();
        settings.set(match_stg);
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
    pub fn mode_settings(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.ModeSettings() as u32 }
    }

    /// set IPT_MODE_SETTINGS
    pub fn set_mode_settings(&mut self, mode: ModeSetting) {
        let mut settings = ModeSettings::none();
        settings.set(mode);
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(*settings as u64) }
    }

    /// returns the original IPT_OPTIONS struct
    /// internal use only
    pub fn wrapped(&self) -> IPT_OPTIONS { self.0 }

    /// get mutable reference to inner IPT_OPTIONS struct
    /// internal use only
    pub fn wrapped_mut(&mut self) -> &mut IPT_OPTIONS { &mut self.0 }
}
