// use glib_sys::*;
use glib::object::ObjectExt;
use glib::translate::ToGlibPtr;
use glib::{MainContext, MainLoop};
use gobject_sys::*;
use libc::c_char;
use libnice_sys::*;
use std::{ptr, thread};

pub struct IceAgent {}

impl IceAgent {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let addr = nice_address_new();
            let result = nice_address_set_from_string(addr, b"127.0.0.1\0".as_ptr() as *const _);
            println!("result is {}", result);
            if result != 0 {
                println!("we have an address set!");
            } else {
                return Err("an error occurred while setting the addr".into());
            }

            let main_ctx = MainContext::new();
            let main_loop = MainLoop::new(Some(&main_ctx), false);
            thread::spawn(move || main_loop.run());

            let agent = nice_agent_new(
                main_ctx.to_glib_full() as *mut _GMainContext,
                NiceCompatibility_NICE_COMPATIBILITY_RFC5245,
            );
            gobject_sys::g_object_set(
                agent as *mut _,
                b"upnp\0".as_ptr() as *const _,
                0,
                std::ptr::null() as *const libc::c_void,
            );
            gobject_sys::g_object_set(
                agent as *mut _,
                b"controlling-mode\0".as_ptr() as *const _,
                0,
                std::ptr::null() as *const libc::c_void,
            );

            glib::signal::connect_raw::<gpointer>(
                agent as *mut _,
                b"candidate-gathering-done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    candidate_gathering_done as *const (),
                )),
                std::ptr::null_mut(),
            );
            glib::signal::connect_raw::<gpointer>(
                agent as *mut _,
                b"new-selected-pair-full\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_selected_pair as *const (),
                )),
                std::ptr::null_mut(),
            );
            glib::signal::connect_raw::<gpointer>(
                agent as *mut _,
                b"component-state-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    component_state_changed as *const (),
                )),
                std::ptr::null_mut(),
            );
            glib::signal::connect_raw::<gpointer>(
                agent as *mut _,
                b"new-remote-candidate-full\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_remote_candidate as *const (),
                )),
                std::ptr::null_mut(),
            );
            let ret = nice_agent_add_local_address(agent, addr);
            if ret != 0 {
                println!("added local addr to agent");
            } else {
                return Err("couldn't add local addr to agent".into());
            }

            let stream_id = nice_agent_add_stream(agent, 1);
            let result = nice_agent_gather_candidates(agent, stream_id);
            if result != 0 {
                println!("candidates gathering succeeded");
            } else {
                return Err("couldnt gather candidates".into());
            }
            nice_agent_attach_recv(
                agent,
                stream_id,
                1,
                main_ctx.to_glib_full() as *mut _GMainContext,
                Some(recvr),
                ptr::null_mut(),
            );
        }

        // nice_agent_new();
        Ok(IceAgent {})
    }
}

unsafe extern "C" fn recvr(
    agent: *mut NiceAgent,
    stream_id: guint,
    component_id: guint,
    len: guint,
    buf: *mut gchar,
    user_data: gpointer,
) {
}

unsafe extern "C" fn candidate_gathering_done(
    agent: *mut NiceAgent,
    stream_id: guint,
    ice: gpointer,
) {
    println!("candidate gathering done callback called");
}

unsafe extern "C" fn new_selected_pair(
    agent: *mut NiceAgent,
    stream_id: guint,
    component_id: guint,
    local: *mut NiceCandidate,
    remote: *mut NiceCandidate,
    ice: gpointer,
) {
    todo!()
}

unsafe extern "C" fn component_state_changed(
    agent: *mut NiceAgent,
    stream_id: guint,
    component_id: guint,
    state: guint,
    ice: gpointer,
) {
    println!("component stated changed callback called");
}

unsafe extern "C" fn new_remote_candidate(
    agent: *mut NiceAgent,
    remote: *mut NiceCandidate,
    ice: gpointer,
) {
    todo!()
}
