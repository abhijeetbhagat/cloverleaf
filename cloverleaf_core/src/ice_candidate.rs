use libc;
use libnice_sys::{
    nice_address_set_from_string, nice_address_set_port, nice_candidate_free, nice_candidate_new,
    NiceCandidate, NiceCandidateTransport_NICE_CANDIDATE_TRANSPORT_UDP,
    NiceCandidateType_NICE_CANDIDATE_TYPE_HOST,
    NiceCandidateType_NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE,
};
use std::{ffi::CString, fmt::Display, ptr::NonNull};

use crate::{candidate_type::CandidateType, mdns_resolve, transport::Transport};

#[derive(Debug)]
pub struct IceCandidate {
    inner: NonNull<NiceCandidate>,
    pub foundation: String,
    pub component: u32,
    pub transport: Transport,
    pub priority: u32,
    pub ip: String,
    pub port: u16,
    pub typ: CandidateType,
}

impl IceCandidate {
    pub fn new(
        foundation: String,
        component: u32,
        transport: Transport,
        priority: u32,
        ip: String,
        port: u16,
        typ: CandidateType,
    ) -> Result<Self, String> {
        let inner;
        unsafe {
            let candidate_type = match &typ {
                CandidateType::HostTcp(_) | CandidateType::HostUdp => {
                    NiceCandidateType_NICE_CANDIDATE_TYPE_HOST
                }
                CandidateType::ServerReflexive(_, _) => {
                    NiceCandidateType_NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE
                }
            };
            inner = NonNull::new(nice_candidate_new(candidate_type))
                .ok_or::<String>("candidate creation failed".into())?;
            (*inner.as_ptr()).component_id = component;
            (*inner.as_ptr()).transport = NiceCandidateTransport_NICE_CANDIDATE_TRANSPORT_UDP;

            // g_strlcpy(c->foundation, rfoundation, NICE_CANDIDATE_MAX_FOUNDATION);
            libc::strcpy(
                (*inner.as_ptr()).foundation.as_mut_ptr(),
                foundation.as_ptr() as *const _,
            );
            (*inner.as_ptr()).priority = priority;

            if let Ok(resolved_ip) = mdns_resolve(&ip) {
                let c_ip = CString::new(resolved_ip.clone()).unwrap();
                let added = nice_address_set_from_string(
                    std::ptr::addr_of_mut!((*inner.as_ptr()).addr),
                    c_ip.as_c_str().as_ptr(),
                );
                if added != 1 {
                    // nice_candidate_free(c);
                }
                nice_address_set_port(std::ptr::addr_of_mut!((*inner.as_ptr()).addr), port as u32);
            } else {
                return Err("there was a problem resolving the candidate addr".into());
            }
        }

        Ok(Self {
            inner,
            foundation,
            component,
            transport,
            priority,
            ip,
            port,
            typ,
        })
    }

    pub fn set_stream_id(&self, stream_id: u32) {
        unsafe {
            (*self.inner.as_ptr()).stream_id = stream_id;
        }
    }

    pub fn get_ptr(&self) -> *mut NiceCandidate {
        self.inner.as_ptr()
    }
}

impl Drop for IceCandidate {
    fn drop(&mut self) {
        unsafe {
            nice_candidate_free(self.inner.as_ptr() as *mut _);
        }
    }
}

impl Display for IceCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO abhi: handle other types of candidates here as well
        // candidate:0 1 UDP 2122187007 9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local 48155 typ host
        write!(
            f,
            "candidate:{} {} {} {} {} {} typ host",
            self.foundation, self.component, self.transport, self.priority, self.ip, self.port,
        )
    }
}
