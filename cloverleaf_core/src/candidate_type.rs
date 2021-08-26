#[derive(Debug, PartialEq)]
pub enum CandidateType {
    HostUdp,
    HostTcp(String),
    ServerReflexive(String, u16),
}
