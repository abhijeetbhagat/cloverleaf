use crate::MediaType;
use crate::RTPPacket;
use insight::connection::RtspConnection;

/// An RTSP source
pub struct RTSPSource {
    conn: RtspConnection,
    callback: Box<dyn Fn(RTPPacket)>,
}

impl RTSPSource {
    /// creates a new RTSP source that connects to the `url`
    pub fn new<S: Into<String>>(url: S) -> Result<Self, String> {
        Ok(RTSPSource {
            conn: RtspConnection::new(url)?,
            callback: Box::new(|p| {
                println!("{:?}", p);
            }),
        })
    }

    /// attach a callback that recvs an RTP packet
    pub fn on_packet<C: Fn(RTPPacket) + 'static>(&mut self, callback: C) {
        self.callback = Box::new(callback);
    }

    /// starts reading RTP packets from the network in an infinite loop
    /// and passes them to the attached callback
    pub fn start(&mut self, media_type: MediaType) {
        self.conn.describe();
        self.conn.setup(media_type);
        self.conn.play();
        loop {
            let packet = self.conn.read_server_stream();
            if let Some(packet) = packet {
                (self.callback)(packet);
            }
        }
    }
}
