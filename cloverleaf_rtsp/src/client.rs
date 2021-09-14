use insight::connection::RtspConnection;

pub struct RTSPSource {
    conn: RtspConnection,
}

impl RTSPSource {
    pub fn new<S: Into<String>>(url: S) -> Result<Self, String> {
        Ok(RTSPSource {
            conn: RtspConnection::new(url).unwrap(),
        })
    }
}
