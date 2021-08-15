use libnice_sys::{
    nice_candidate_free, nice_candidate_new, NiceCandidate,
    NiceCandidateType_NICE_CANDIDATE_TYPE_HOST,
    NiceCandidateType_NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE,
};
use std::ptr::NonNull;

use crate::{candidate_type::CandidateType, transport::Transport};

pub struct IceCandidate {
    inner: NonNull<NiceCandidate>,
    foundation: u8,
    component: u8,
    transport: Transport,
    priority: u32,
    ip: String,
    port: u16,
    typ: CandidateType,
}

impl IceCandidate {
    pub fn new(
        foundation: u8,
        component: u8,
        transport: Transport,
        priority: u32,
        ip: String,
        port: u16,
        typ: CandidateType,
    ) -> Result<Self, String> {
        let inner;
        unsafe {
            match &typ {
                CandidateType::HostTcp(_) | CandidateType::HostUdp => {
                    inner = NonNull::new(nice_candidate_new(
                        NiceCandidateType_NICE_CANDIDATE_TYPE_HOST,
                    ))
                    .ok_or::<String>("candidate creation failed".into())?
                }
                CandidateType::ServerReflexive(_, _) => {
                    inner = NonNull::new(nice_candidate_new(
                        NiceCandidateType_NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE,
                    ))
                    .ok_or::<String>("candidate creation failed".into())?
                }
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
}

impl Drop for IceCandidate {
    fn drop(&mut self) {
        unsafe {
            nice_candidate_free(self.inner.as_ptr() as *mut _);
        }
    }
}
