use crate::ch_last_error;
use crate::Options;

use std::os::windows::raw::HANDLE;
use std::io::Error;
use std::ffi::c_void;
use std::mem::zeroed;

/// A type for controlling process tracing
#[derive(Copy, Clone, Debug)]
pub struct ProcessTracer { proc: HANDLE }
impl ProcessTracer {
    /// Create a new Process tracer without starting a trace.
    ///
    /// Usefull if you want to manage traces you didnt start
    pub fn new(proc: HANDLE) -> ProcessTracer {
        ProcessTracer{ proc }
    }

    /// Create a new instance and start tracing the process `proc`
    pub fn new_start(proc: HANDLE, opt: Options) -> Result<ProcessTracer, Error> {
        let tracer = ProcessTracer { proc };
        tracer.start(opt).map(|_| tracer)
    }

    /// Start tracing the process `proc`
    pub fn start(self, opt: Options) -> Result<(), Error> {
        let res: i32;
        unsafe { res = winipt_sys::StartProcessIptTracing(self.proc, opt.0) };
        ch_last_error(res > 0)
    }

    /// Get process trace (so far)
    pub fn trace(self) -> Result<Vec<u8>, Error> {
        let res: i32;
        let mut buf = vec![0; self.size()? as usize];
        unsafe { res = winipt_sys::GetProcessIptTrace(
            self.proc, buf.as_mut_ptr() as *mut c_void, buf.len() as u32
        );}
        ch_last_error(res > 0).map(|_| buf)
    }

    /// Get the size of the process trace in bytes 
    pub fn size(self) -> Result<u32, Error> {
        let mut sz: u32 = 0;
        let res: i32;
        unsafe { res = winipt_sys::GetProcessIptTraceSize(self.proc, &mut sz); }
        ch_last_error(res > 0).map(|_| sz)
    }

    pub fn stop(self) -> Result<(), Error> {
        let res: i32;
        unsafe { res = winipt_sys::StopProcessIptTracing(self.proc); }
        ch_last_error(res > 0)
    }

    /// Get the option of the process tracer
    pub fn options(self) -> Result<Options, Error> {
        let res: i32;
        // using the integer union field to initialize everything to 0
        let mut opt: Options = unsafe { zeroed() };
        unsafe { 
            res = winipt_sys::QueryProcessIptTracing(self.proc, &mut opt.0);
        }
        ch_last_error(res > 0).map(|_| opt)
    }
}