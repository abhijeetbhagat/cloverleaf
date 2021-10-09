use cloverleaf_rtsp::client::RTSPSource;
use cloverleaf_rtsp::RTPPacket;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::broadcast::Sender;

pub struct Streamer {
    tx: Arc<RwLock<Sender<RTPPacket>>>,
}

impl Streamer {
    pub fn new(tx: Arc<RwLock<Sender<RTPPacket>>>) -> Self {
        Streamer { tx }
    }

    pub async fn run(self) {
        let mut source =
            RTSPSource::new("rtsp://wowzaec2demo.streamlock.net/vod/mp4:BigBuckBunny_115k.mov")
                .unwrap();
        let tx = Arc::clone(&self.tx);
        loop {
            let packet = source.get_packet();
            match packet {
                Some(packet) => {
                    let tx = tx.read().unwrap();
                    // broadcast to viewers
                    tx.send(packet).unwrap();
                }
                _ => {}
            }
        }
    }

    pub async fn on_packet(&self, packet: RTPPacket) {}
}
