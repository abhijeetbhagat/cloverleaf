use std::fmt::Display;

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

impl Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transport::Udp => write!(f, "UDP"),
            Transport::Tcp => write!(f, "TCP"),
            Transport::Invalid => Err(std::fmt::Error),
        }
    }
}
