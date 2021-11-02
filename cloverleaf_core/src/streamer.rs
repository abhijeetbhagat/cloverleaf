use cloverleaf_rtsp::client::RTSPSource;
use cloverleaf_rtsp::MediaType;
use cloverleaf_rtsp::RTPPacket;
use std::sync::Arc;
use std::sync::RwLock;
// use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::Sender;

pub struct Streamer {
    // tx: Arc<RwLock<Sender<RTPPacket>>>,
    tx: Sender<RTPPacket>,
    url: String,
}

impl Streamer {
    // pub fn new(tx: Arc<RwLock<Sender<RTPPacket>>>) -> Self {
    pub fn new(tx: Sender<RTPPacket>, url: String) -> Self {
        Streamer { tx, url }
    }

    pub async fn run(self) {
        let mut source = RTSPSource::new(self.url).unwrap();
        // let tx = Arc::clone(&self.tx);
        source.start(MediaType::Video);
        loop {
            let packet = source.get_packet();
            match packet {
                Some(packet) => {
                    // let tx = self.tx.read().unwrap();
                    // broadcast to viewers
                    println!("recvd packet. broadcasting ...");
                    /*
                    match tx.send(packet) {
                        Ok(n) => println!("sent to {} recvrs", n),
                        _ => println!("there was an error sending data to the recvrs"),
                    }
                    */
                    self.tx.send(packet).await;
                }
                _ => {
                    println!("did not recv packet. trying again.")
                }
            }
        }
    }

    pub async fn on_packet(&self, packet: RTPPacket) {}
}
