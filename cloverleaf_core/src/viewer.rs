use crate::IceAgent;
use cloverleaf_rtsp::RTPPacket;
use tokio::sync::broadcast::Receiver;

pub struct Viewer {
    ice_agent: IceAgent,
    rx: Receiver<RTPPacket>,
}

impl Viewer {
    pub fn new(ice_agent: IceAgent, rx: Receiver<RTPPacket>) -> Self {
        Viewer { ice_agent, rx }
    }

    pub async fn listen_rtp_packets(mut self) {
        loop {
            let packet = self.rx.recv().await.unwrap();
            if let Ok(_) = self.ice_agent.send_msg(&Vec::<u8>::from(packet)) {}
        }
    }
}
