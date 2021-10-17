use crate::MediaType;
use crate::RTPPacket;
use insight::connection::RtspConnection;

/// An RTSP source
pub struct RTSPSource {
    conn: RtspConnection,
    // callback: Box<dyn Fn(RTPPacket)>,
}

impl RTSPSource {
    /// creates a new RTSP source that connects to the `url`
    pub fn new<S: Into<String>>(url: S) -> Result<Self, String> {
        Ok(RTSPSource {
            conn: RtspConnection::new(url)?,
            /*
            callback: Box::new(|p| {
                println!("{:?}", p);
            }),
            */
        })
    }

    /// attach a callback that recvs an RTP packet
    /*
    pub fn on_packet<C: Fn(RTPPacket) + 'static>(&mut self, callback: C) {
        self.callback = Box::new(callback);
    }
    */

    /// performs an rtsp handshake
    pub fn start(&mut self, media_type: MediaType) {
        self.conn.describe().unwrap();
        self.conn.setup(media_type).unwrap();
        self.conn.play().unwrap();
    }

    pub fn get_packet(&mut self) -> Option<RTPPacket> {
        let packet = self.conn.read_server_stream();
        if let Ok(packet) = packet {
            packet
        } else {
            None
        }
    }
}
