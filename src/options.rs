use crate::settings::{ TimingSettings, MatchSettings, ModeSettings };
use winipt_sys::IPT_OPTIONS;

// bindgen is wrong about the bitfield sizes so im casting them to prevent ub
// u8 is as close as i can get. most insts are actually 4 bits

#[derive(Clone, Copy)]
pub struct OptionsBuilder(pub(super) IPT_OPTIONS);
impl OptionsBuilder {
    /// creates a new IPT_OPTIONS struct
    /// and initializes it with zeroes
    /// OptionVersion will be set to 1 tho
    pub fn new() -> Self {
        let o = OptionsBuilder(IPT_OPTIONS { AsULonglong: 0 });
        o.option_version(1)
    }

    /// Must be set to 1 (will be by deault)
    pub fn option_version(mut self, version: u8) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_OptionVersion(version as u64) };
        self
    }

    /// sets IPT_TIMING_SETTINGS
    pub fn timing_settings(mut self, settings: TimingSettings) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_TimingSettings(
            settings.bits() as u64)
        };

        self
    }

    /// sets Bits 14:17 in IA32_RTIT_CTL
    pub fn mtc_frequency(mut self, freq: u8) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_MtcFrequency(freq as u64) };
        self
    }

    /// sets Bits 19:22 in IA32_RTIT_CTL
    pub fn cyc_threshold(mut self, threshold: u8) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_CycThreshold(threshold as u64) };
        self
    }

    /// sets the Size of buffer in ToPA, as 4KB powers of 2 (4KB->128MB).
    /// Multiple buffers will be used if CPUID.(EAX=014H,ECX=0H):ECX[1]= 1
    /// 
    /// __**WARNING: I HAVE NO IDEA WHAT HAPPENS WITH UNALIGNED VALUES**__
    pub fn topa_pages_pow2(mut self, size: u8) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_TopaPagesPow2(size as u64) };
        self
    }

    /// set IPT_MATCH_SETTINGS
    pub fn match_settings(mut self, settings: MatchSettings) -> Self {
        // IPT_MATCH_SETTINGS is 3 bytes so casting here is fine
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(
            settings.bits() as u64
        )};

        self
    }

    // if children will be automatically added to the trace
    pub fn inherit(mut self, inherit: bool) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(inherit as u64) };
        self
    }

    /// set IPT_MODE_SETTINGS
    pub fn mode_settings(mut self, settings: ModeSettings) -> Self {
        unsafe { self.0.__bindgen_anon_1.set_MatchSettings(
            settings.bits() as u64
        )};

        self
    }

    pub fn finish(self) -> Options { Options(self.0) }
}

/// a wrapper around IPT_OPTIONS
/// it contains all of the options used to manipulate the intelpt driver
#[derive(Copy, Clone)]
pub struct Options (pub(super) IPT_OPTIONS);
impl Options {
    /// Must be set to 1 (will be by default)
    pub fn option_version(self) -> u8 {
        unsafe { self.0.__bindgen_anon_1.OptionVersion() as u8 }
    }

    /// gets IPT_TIMING_SETTINGS
    pub fn timing_settings(self) -> TimingSettings {
        TimingSettings::from_bits(
            unsafe { self.0.__bindgen_anon_1.TimingSettings() as i32 }
        ).unwrap()
    }

    /// gets Bits 14:17 in IA32_RTIT_CTL
    pub fn mtc_frequency(self) -> u8 {
        unsafe { self.0.__bindgen_anon_1.MtcFrequency() as u8 }
    }

    /// gets Bits 19:22 in IA32_RTIT_CTL
    pub fn cyc_threshold(self) -> u8 {
        unsafe { self.0.__bindgen_anon_1.CycThreshold() as u8 }
    }

    /// gets the Size of buffer in ToPA, as 4KB powers of 2 (4KB->128MB).
    /// Multiple buffers will be used if CPUID.(EAX=014H,ECX=0H):ECX[1]= 1
    pub fn topa_pages_pow2(self) -> u8 {
        unsafe { self.0.__bindgen_anon_1.TopaPagesPow2() as u8 }
    }

    /// get IPT_MATCH_SETTINGS
    pub fn match_settings(self) -> MatchSettings {
        MatchSettings::from_bits(
            unsafe { self.0.__bindgen_anon_1.MatchSettings() as i32 }
        ).unwrap()
    }

    /// if children will be automatically added to the trace
    pub fn inherit(self) -> bool {
        unsafe { self.0.__bindgen_anon_1.Inherit() > 0 }
    }

    /// get IPT_MODE_SETTINGS
    pub fn mode_settings(self) -> ModeSettings {
        ModeSettings::from_bits(
            unsafe { self.0.__bindgen_anon_1.ModeSettings() as i32 }
        ).unwrap()
    }
}
