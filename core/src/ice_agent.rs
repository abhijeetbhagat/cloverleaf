// use glib_sys::*;
use glib::object::ObjectExt;
use glib::translate::ToGlibPtr;
use glib::{signal::connect_raw, MainContext, MainLoop};
use gobject_sys::g_object_set;
use libc::c_char;
use libnice_sys::*;
use std::ffi::{CStr, CString};
use std::{ptr, thread};

pub struct IceAgent {
    main_ctx: MainContext,
    inner: *mut NiceAgent,
    stream_id: u32,
}

unsafe impl Send for IceAgent {}
unsafe impl Sync for IceAgent {}

impl IceAgent {
    pub fn new(main_ctx: MainContext) -> Result<Self, String> {
        let agent;
        let stream_id;

        unsafe {
            let addr = nice_address_new();
            let result = nice_address_set_from_string(addr, b"127.0.0.1\0".as_ptr() as *const _);
            println!("result is {}", result);
            if result != 0 {
                println!("we have an address set!");
            } else {
                return Err("an error occurred while setting the addr".into());
            }

            let main_loop = MainLoop::new(Some(&main_ctx), false);
            thread::spawn(move || main_loop.run());

            agent = nice_agent_new(
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

            connect_raw::<gpointer>(
                agent as *mut _,
                b"candidate-gathering-done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    candidate_gathering_done as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
                agent as *mut _,
                b"new-selected-pair-full\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_selected_pair as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
                agent as *mut _,
                b"component-state-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    component_state_changed as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
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

            stream_id = nice_agent_add_stream(agent, 1);
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
        Ok(IceAgent {
            main_ctx,
            inner: agent,
            stream_id,
        })
    }

    /// Gets local creds to be sent in the offer sdp
    pub fn get_local_credentials(&self) -> Result<(String, String), String> {
        unsafe {
            let mut ufrag: *mut gchar = ptr::null_mut();
            let mut pwd: *mut gchar = ptr::null_mut();
            nice_agent_get_local_credentials(
                self.inner,
                self.stream_id,
                (&mut ufrag) as *mut *mut _,
                (&mut pwd) as *mut *mut _,
            );

            // let ufrag_cstr = CStr::from_ptr(ufrag);
            // let pwd_cstr = CStr::from_ptr(pwd);
            let ufrag_len = libc::strlen(ufrag);
            let pwd_len = libc::strlen(pwd);

            let mut ufrag_vec = Vec::with_capacity(ufrag_len + 1);
            let mut pwd_vec = Vec::with_capacity(pwd_len + 1);
            ptr::copy_nonoverlapping(ufrag, ufrag_vec.as_mut_ptr() as *mut _, ufrag_len);
            ptr::copy_nonoverlapping(pwd, pwd_vec.as_mut_ptr() as *mut _, pwd_len);

            g_free(ufrag as *mut _);
            g_free(pwd as *mut _);

            ufrag_vec.set_len(ufrag_len + 1);
            let ufrag = CString::from_vec_unchecked(ufrag_vec)
                .into_string()
                .unwrap();

            pwd_vec.set_len(ufrag_len + 1);
            let pwd = CString::from_vec_unchecked(pwd_vec).into_string().unwrap();

            Ok((ufrag, pwd))
        }
    }

    /// Sets remote creds that are extracted from an answer sdp
    pub fn set_remote_credentials(&self, ufrag: &str, pwd: &str) -> Result<(), String> {
        let ufrag = CString::new(ufrag).unwrap();
        let pwd = CString::new(pwd).unwrap();

        unsafe {
            match nice_agent_set_remote_credentials(
                self.inner,
                self.stream_id,
                ufrag.as_ptr(),
                pwd.as_ptr(),
            ) {
                0 => Err("couldn't set remote creds".into()),
                _ => Ok(()),
            }
        }
    }

    /// Sends buf to the remote peer
    pub fn send_msg(&self, buf: &[u8]) -> Result<(), String> {
        todo!()
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
