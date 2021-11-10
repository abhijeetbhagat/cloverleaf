use cloverleaf_rtsp::RTPPacket;
use glib::ffi::{g_free, g_malloc0, g_slist_append, gpointer, GSList};
use glib::object::ObjectExt;
use glib::translate::ToGlibPtr;
use glib::{signal::connect_raw, MainContext, MainLoop};
use glib_sys::GMainContext;
use gobject_sys::g_object_set;
use libc::{c_char, c_uint};
// use libnice_sys::*;
use libnice::sys::*;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;
use std::{ptr, thread};

use crate::ice_candidate::IceCandidate;
use crate::transport::Transport;
use crate::CandidateType;
const INET6_ADDRSTRLEN: usize = 46;
/// an ICE agent
pub struct IceAgent {
    main_ctx: MainContext,
    inner: NonNull<NiceAgent>,
    stream_id: u32,
    component_id: u32,
    candidates: Vec<IceCandidate>,
}

unsafe impl Send for IceAgent {}
unsafe impl Sync for IceAgent {}

impl IceAgent {
    /// creates a new `IceAgent` with the given `MainContext`
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

            agent = NonNull::new(nice_agent_new(
                main_ctx.to_glib_full() as *mut GMainContext,
                NiceCompatibility_NICE_COMPATIBILITY_RFC5245,
            ))
            .ok_or::<String>("agent creation failed".into())?;

            gobject_sys::g_object_set(
                agent.as_ptr() as *mut _,
                b"upnp\0".as_ptr() as *const _,
                0,
                std::ptr::null() as *const libc::c_void,
            );
            gobject_sys::g_object_set(
                agent.as_ptr() as *mut _,
                b"controlling-mode\0".as_ptr() as *const _,
                0,
                std::ptr::null() as *const libc::c_void,
            );

            connect_raw::<gpointer>(
                agent.as_ptr() as *mut _,
                b"candidate-gathering-done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    candidate_gathering_done as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
                agent.as_ptr() as *mut _,
                b"new-selected-pair-full\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_selected_pair as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
                agent.as_ptr() as *mut _,
                b"component-state-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    component_state_changed as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
                agent.as_ptr() as *mut _,
                b"new-remote-candidate-full\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_remote_candidate as *const (),
                )),
                std::ptr::null_mut(),
            );
            connect_raw::<gpointer>(
                agent.as_ptr() as *mut _,
                b"new-candidate-full\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_local_candidate as *const (),
                )),
                std::ptr::null_mut(),
            );
            let ret = nice_agent_add_local_address(agent.as_ptr(), addr);
            if ret != 0 {
                println!("added local addr to agent");
            } else {
                return Err("couldn't add local addr to agent".into());
            }

            stream_id = nice_agent_add_stream(agent.as_ptr(), 1);
            let result = nice_agent_gather_candidates(agent.as_ptr(), stream_id);
            if result != 0 {
                println!("host candidate allocated");
            } else {
                return Err("either invalid stream id or couldnt allocate host candidate".into());
            }
            nice_agent_attach_recv(
                agent.as_ptr(),
                stream_id,
                1,
                main_ctx.to_glib_full() as *mut GMainContext,
                Some(recvr),
                ptr::null_mut(),
            );
        }

        Ok(IceAgent {
            main_ctx,
            inner: agent,
            stream_id,
            component_id: 1, // 1 is rtp, 2 is rtcp
            candidates: vec![],
        })
    }

    /// gets local candidates
    pub fn get_local_candidates(&self) -> Result<Vec<IceCandidate>, String> {
        unsafe {
            let lcands = nice_agent_get_local_candidates(
                self.inner.as_ptr(),
                self.stream_id,
                self.component_id,
            );
            let mut ptr = lcands;
            let mut candidates = vec![];
            while !ptr.is_null() {
                let candidate: *mut NiceCandidate = (*ptr).data as *mut _;
                let addr: *mut c_char = g_malloc0(INET6_ADDRSTRLEN) as *mut _;
                nice_address_to_string(ptr::addr_of!((*candidate).addr), addr);
                let port = nice_address_get_port(ptr::addr_of!((*candidate).addr));
                if (*candidate).type_ == NiceCandidateType_NICE_CANDIDATE_TYPE_HOST {
                    if (*candidate).transport == NiceCandidateTransport_NICE_CANDIDATE_TRANSPORT_UDP
                    {
                        let c_str = CStr::from_ptr((*candidate).foundation.as_ptr() as *mut _);
                        candidates.push(
                            IceCandidate::new(
                                c_str.to_owned().into_string().unwrap(),
                                self.component_id,
                                Transport::Udp,
                                (*candidate).priority,
                                CString::from_raw(addr).into_string().unwrap(),
                                port as u16,
                                CandidateType::HostUdp,
                            )
                            .unwrap(),
                        );
                    }
                }
                ptr = (*ptr).next;
            }
            Ok(candidates)
        }
    }

    /// gets local creds to be sent in the offer sdp
    pub fn get_local_credentials(&self) -> Result<(String, String), String> {
        unsafe {
            let mut ufrag: *mut c_char = ptr::null_mut();
            let mut pwd: *mut c_char = ptr::null_mut();
            nice_agent_get_local_credentials(
                self.inner.as_ptr(),
                self.stream_id,
                (&mut ufrag) as *mut *mut _,
                (&mut pwd) as *mut *mut _,
            );

            // https://stackoverflow.com/questions/24145823/how-do-i-convert-a-c-string-into-a-rust-string-and-back-via-ffi
            let ufrag_string = CStr::from_ptr(ufrag).to_str().unwrap().to_owned();
            let pwd_string = CStr::from_ptr(pwd).to_str().unwrap().to_owned();

            g_free(ufrag as *mut _);
            g_free(pwd as *mut _);

            Ok((ufrag_string, pwd_string))
        }
    }

    /// sets remote creds that are extracted from an answer sdp
    pub fn set_remote_credentials(&self, ufrag: &str, pwd: &str) -> Result<(), String> {
        let ufrag = CString::new(ufrag).unwrap();
        let pwd = CString::new(pwd).unwrap();

        unsafe {
            match nice_agent_set_remote_credentials(
                self.inner.as_ptr(),
                self.stream_id,
                ufrag.as_ptr(),
                pwd.as_ptr(),
            ) {
                0 => Err("couldn't set remote creds".into()),
                _ => Ok(()),
            }
        }
    }

    /// adds a remote candidate
    pub fn add_remote_candidate(&mut self, candidate: IceCandidate) {
        self.candidates.push(candidate);
    }

    /// sets the remote candidates for the agent
    pub fn done(&self) {
        unsafe {
            let list: *mut GSList = std::ptr::null_mut();
            for candidate in &self.candidates {
                g_slist_append(list, candidate.get_ptr() as *mut _);
            }

            nice_agent_set_remote_candidates(
                self.inner.as_ptr(),
                self.stream_id,
                1,
                list as *const _,
            );
        }
    }

    /// sends buf to the remote peer.
    ///
    /// this is 'virtually' a non-blocking operation in non-reliable (UDP) mode.
    pub fn send_msg(&mut self, packet: &RTPPacket) -> Result<(), String> {
        let buf = Vec::<u8>::from(packet);
        unsafe {
            let ret = nice_agent_send(
                self.inner.as_ptr(),
                self.stream_id,
                self.component_id,
                packet.payload.len() as u32,
                buf.as_ptr() as *const _,
            );
            if ret < 0 {
                return Err("there was a problem sending the packet to the browser".into());
            }
        }
        Ok(())
    }
}

impl Drop for IceAgent {
    fn drop(&mut self) {
        unsafe {
            gobject_sys::g_object_unref(self.inner.as_ptr() as *mut _);
        }
    }
}

unsafe extern "C" fn recvr(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    len: c_uint,
    buf: *mut c_char,
    user_data: gpointer,
) {
}

unsafe extern "C" fn candidate_gathering_done(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    ice: gpointer,
) {
    println!("candidate gathering done callback called");
}

unsafe extern "C" fn new_selected_pair(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    local: *mut NiceCandidate,
    remote: *mut NiceCandidate,
    ice: gpointer,
) {
    println!("new-selected-pair cb called");
}

unsafe extern "C" fn component_state_changed(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    state: c_uint,
    ice: gpointer,
) {
    println!("component stated changed callback called");
}

unsafe extern "C" fn new_remote_candidate(
    agent: *mut NiceAgent,
    remote: *mut NiceCandidate,
    ice: gpointer,
) {
    println!("new remote candidate cb called");
}

unsafe extern "C" fn new_local_candidate(
    agent: *mut NiceAgent,
    remote: *mut NiceCandidate,
    ice: gpointer,
) {
    // this will be useful for full trickle
    // call nice_agent_get_local_candidates here
    println!("new local candidate callback called");
}
