use bitmask::bitmask;
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
