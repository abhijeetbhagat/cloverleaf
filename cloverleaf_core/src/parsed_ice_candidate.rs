use crate::{candidate_type::CandidateType, transport::Transport};

#[derive(Debug)]
pub struct ParsedIceCandidate {
    pub foundation: String,
    pub component: u32,
    pub transport: Transport,
    pub priority: u32,
    pub ip: String,
    pub port: u16,
    pub typ: CandidateType,
}
