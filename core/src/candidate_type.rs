pub(crate) enum CandidateType {
    HostUdp,
    HostTcp(String),
    ServerReflexive(String, u16),
}

impl From<&str> for CandidateType {
    fn from(typ: &str) -> Self {
        match typ {
            "host" => 
        }
    }
}
