use crate::ch_last_error;

use std::io::Error;
use std::os::windows::raw::HANDLE;

/// A type for controlling thread tracing
#[derive(Copy, Clone, Debug)]
pub struct ThreadTracer { thread: HANDLE }
impl ThreadTracer {
    /// Create a new Process tracer without starting a trace.
    ///
    /// Usefull if you want to manage traces you didnt start
    pub fn new(thread: HANDLE) -> ThreadTracer {
        ThreadTracer { thread }
    }

    /// pause tracing on the thread
    ///
    /// Returns the previous tracing state of the thread
    pub fn pause(self) -> Result<bool, Error> {
        let res: i32;
        let mut pbres: u8 = 0;
        unsafe { res = winipt_sys::PauseThreadIptTracing(self.thread, &mut pbres); }
        ch_last_error(res > 0).map(|_| pbres > 0)
    }
    
    /// Resume tracing on the thread
    ///
    /// Returns the previous tracing state of the thread
    pub fn resume(self) -> Result<bool, Error> {
        let res: i32;
        let mut pbres: u8 = 0;
        unsafe { res = winipt_sys::ResumeThreadIptTracing(self.thread, &mut pbres); }
        ch_last_error(res > 0).map(|_| pbres > 0)
    }
}