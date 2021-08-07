// use glib_sys::*;
use glib::{MainContext, MainLoop};
use libc::c_char;
use libnice_sys::*;
use std::ffi::CString;
use std::{ptr, thread};

pub struct IceAgent {}

impl IceAgent {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let addr = nice_address_new();
            let result =
                nice_address_set_from_string(addr, CString::new("127.0.0.1").unwrap().as_ptr());
            println!("result is {}", result);
            if result != 0 {
                println!("we have an address set!");
            } else {
                return Err("an error occurred while setting the addr".into());
            }

            let main_ctx = g_main_context_new();
            let main_loop = g_main_loop_new(main_ctx, 0);
            let agent = nice_agent_new(main_ctx, NiceCompatibility_NICE_COMPATIBILITY_RFC5245);
            nice_agent_add_local_address(agent, addr);
        }
        // nice_agent_new();
        Ok(IceAgent {})
    }
}
