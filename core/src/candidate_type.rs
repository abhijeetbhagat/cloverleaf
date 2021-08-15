pub enum CandidateType {
    HostUdp,
    HostTcp(String),
    ServerReflexive(String, u16),
}
