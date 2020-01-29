use bitflags::bitflags;
use winipt_sys::{
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

bitflags! {
    pub struct TimingSettings: i32 {
        const ENABLE_CYC_PACKETS = IPT_TIMING_SETTINGS_IptEnableCycPackets;
        const ENABLE_MTC_PACKETS = IPT_TIMING_SETTINGS_IptEnableMtcPackets;
        const NO_TIMING_PACKETS  = IPT_TIMING_SETTINGS_IptNoTimingPackets;
    }
}

bitflags! {
    pub struct MatchSettings : i32 {
        const MATCH_BY_ANY_APP        = _IPT_MATCH_SETTINGS_IptMatchByAnyApp;
        const MATCH_BY_ANY_PACKAGE    = _IPT_MATCH_SETTINGS_IptMatchByAnyPackage;
        const MATCH_BY_IMAGE_FILENAME = _IPT_MATCH_SETTINGS_IptMatchByImageFileName;
        const MATCH_BY_PACKAGE_NAME   = _IPT_MATCH_SETTINGS_IptMatchByPackageName;
    }
}

bitflags! {
    pub struct ModeSettings : i32 {
        const CTL_KERNELMODE_ONLY     = _IPT_MODE_SETTINGS_IptCtlKernelModeOnly;
        const CTL_USER_AND_KERNELMODE = _IPT_MODE_SETTINGS_IptCtlUserAndKernelMode;
        const CTL_USERMODE_ONLY       = _IPT_MODE_SETTINGS_IptCtlUserModeOnly;
        const REG_KERNELMODE_ONLY     = _IPT_MODE_SETTINGS_IptRegKernelModeOnly;
        const REG_USER_AND_KERNELMODE = _IPT_MODE_SETTINGS_IptRegUserAndKernelMode;
        const REG_USERMODE_ONLY       = _IPT_MODE_SETTINGS_IptRegUserModeOnly;
    }
}
