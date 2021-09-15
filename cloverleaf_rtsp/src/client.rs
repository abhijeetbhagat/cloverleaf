use crate::RTPPacket;
use insight::connection::RtspConnection;

pub struct RTSPSource {
    conn: RtspConnection,
    callback: Box<dyn Fn(RTPPacket)>,
}

impl RTSPSource {
    pub fn new<S: Into<String>>(url: S) -> Result<Self, String> {
        Ok(RTSPSource {
            conn: RtspConnection::new(url)?,
            callback: Box::new(|p| {
                println!("{:?}", p);
            }),
        })
    }

    pub fn on_packet<C: Fn(RTPPacket) + 'static>(&mut self, callback: C) {
        self.callback = Box::new(callback);
    }
}
