use crate::Options;
use crate::ch_last_error;

use std::io::Error;
use std::mem::zeroed;

/// A type for controlling core tracing.
///
/// Creating an instance is not needed since there is no state
#[derive(Copy, Clone, Debug)]
pub struct CoreTracer {}
impl CoreTracer {
    /// Start a core trace
    pub fn start(opt: Options) -> Result<(), Error> {
        let res: i32;
        let tries: u32 = 3; // need to find how many tries to set
        let duration: u32 = 60; // need to find what duration to set (in seconds)
        unsafe { 
            res = winipt_sys::StartCoreIptTracing(opt.0, tries, duration);
        }
        ch_last_error(res > 0)
    }

    /// Get the option for the current core trace
    pub fn options() -> Result<Options, Error> {
        let res: i32;
        let mut opt: Options = unsafe { zeroed() };
        unsafe { res = winipt_sys::QueryCoreIptTracing(&mut opt.0); }
        ch_last_error(res > 0).map(|_| opt)
    }
}