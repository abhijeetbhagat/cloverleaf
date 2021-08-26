#[derive(Debug)]
pub enum Transport {
    Udp,
    Tcp,
    Invalid,
}

impl From<&str> for Transport {
    fn from(transport: &str) -> Self {
        match transport {
            "UDP" => Self::Udp,
            "TCP" => Self::Tcp,
            _ => Self::Invalid,
        }
    }
}
