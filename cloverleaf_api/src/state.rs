use crate::payload::{Payload, PayloadCandidate};
use cloverleaf_core::sdp::create_sdp;
// use cloverleaf_core::Encryptor;
use cloverleaf_core::{
    sdp::{parse_candidate, Sdp},
    IceAgent, Streamer,
};
use cloverleaf_rtsp::RTPPacket;
use glib::MainContext;
use rocket::serde::json::{serde_json, Json};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct CloverLeafState {
    temp_streams: Arc<RwLock<HashMap<String, IceAgent>>>,
    _streams: Arc<RwLock<HashMap<String, Arc<RwLock<Sender<RTPPacket>>>>>>,
    _tx: Arc<RwLock<Sender<RTPPacket>>>,
    active: Arc<RwLock<bool>>,
    cert_path: String,
    key_path: String,
}

unsafe impl Send for CloverLeafState {}
unsafe impl Sync for CloverLeafState {}

impl CloverLeafState {
    pub fn new(cert: &str, key: &str) -> Result<CloverLeafState, String> {
        let (tx, _) = broadcast::channel(20);
        Ok(Self {
            temp_streams: Arc::new(RwLock::new(HashMap::new())),
            _streams: Arc::new(RwLock::new(HashMap::new())),
            _tx: Arc::new(RwLock::new(tx)),
            active: Arc::new(RwLock::new(false)),
            cert_path: cert.into(),
            key_path: key.into(),
        })
    }

    /// creates a session, an ice agent representing this session
    /// and returns the sdp to be sent into an offer by the caller.
    pub fn create_session(&self) -> Result<(String, String), String> {
        let uuid = Uuid::new_v4();
        let streams = self.temp_streams.clone();
        let mut streams = streams.write().unwrap();
        let main_ctx = MainContext::new();
        if let Ok(agent) = IceAgent::new(main_ctx, &self.cert_path, &self.key_path) {
            let (ufrag, pwd) = agent.get_local_credentials().unwrap();
            let sdp = Sdp { ufrag, pwd };
            if let Ok(lcands) = agent.get_local_candidates() {
                let sdp = create_sdp(&sdp, &lcands[0], &agent.get_fingerprint());

                streams.insert(uuid.to_string(), agent);
                return Ok((uuid.to_string(), sdp));
            } else {
                return Err("error getting local cands".into());
            }
        }
        Err("error creating ice agent".into())
    }

    /// extracts info from the answer
    pub fn process_answer(&self, payload: Json<Payload>) {
        println!("we recvd an answer: {}", payload.payload);
        let sdp = Sdp::from(payload.payload.as_str());
        let mut streams = self.temp_streams.write().unwrap();
        if let Some(agent) = streams.get_mut(&payload.session) {
            let _ = agent.set_remote_credentials(&sdp.ufrag, &sdp.pwd);
        }
    }

    /// adds a candidate to the ice agent associated with the
    /// supplied session.
    pub fn add_candidate(&self, payload: Json<Payload>) {
        // payload looks like: {"candidate":"candidate:2 1 TCP 2105458943 0a8aa0e9-d5f0-4377-b6d4-daa4495a6b6f.local 9 typ host tcptype active","sdpMid":"video","sdpMLineIndex":0,"usernameFragment":"7ff02998" }
        use cloverleaf_core::IceCandidate;

        if let Ok(string_candidate) = serde_json::from_str::<PayloadCandidate>(&payload.payload) {
            match parse_candidate(&string_candidate.candidate) {
                Ok(candidate) => {
                    let mut streams = self.temp_streams.write().unwrap();
                    if let Some(agent) = streams.get_mut(&payload.session) {
                        println!("adding remote candidate to the list");
                        agent.add_remote_candidate(
                            IceCandidate::new(
                                candidate.foundation,
                                candidate.component,
                                candidate.transport.into(),
                                candidate.priority,
                                candidate.ip.into(),
                                candidate.port,
                                candidate.typ,
                            )
                            .unwrap(),
                        );
                    }
                }
                Err(e) => {
                    println!("there was an error parsing candidate: {e}");
                }
            }
        } else {
            println!("there was an error parsing payload");
        }
    }

    /// adds a candidate to the ice agent associated with the
    /// supplied session.
    pub fn candidates_done(&self, payload: Json<Payload>) {
        let mut streams = self.temp_streams.write().unwrap();
        if let Some(agent) = streams.get_mut(&payload.session) {
            agent.remote_candidates_gathering_done();
        }
    }

    /// starts the requested stream in the 'payload' field of payload
    pub fn start(&self, payload: Json<Payload>) {
        let _id = &payload.id;
        let session = &payload.session;

        let (tx, mut rx) = mpsc::channel(1000);

        // spawn streaming if not running already
        if !*self.active.read().unwrap() {
            // let source = Streamer::new(self.tx.clone());
            let source = Streamer::new(tx, payload.payload.clone());
            tokio::task::spawn(source.run());
            let mut active = self.active.write().unwrap();
            *active = true;
        }

        let mut streams = self.temp_streams.write().unwrap();
        // remove session from the temp streams hashmap and transfer the ownership
        // of the ice agent to the spawned task
        if streams.contains_key(session) {
            let (_, mut agent) = streams.remove_entry(session).unwrap();
            // let encryptor = Encryptor::new(&self.cert_path, &self.key_path).unwrap();
            // let tx = self.tx.read().unwrap();
            // let mut rx = tx.subscribe();
            tokio::task::spawn(async move {
                loop {
                    println!("waiting for packet from tx");
                    /*
                    match rx.recv().await {
                        Ok(mut packet) => {
                            // TODO abhi: this ssrc is from the hardcoded sdp we are returning.
                            // fix this to be a random value.
                            packet.ssrc = 1811295701;
                            println!("sending packet to rtc agent");
                            agent.send_msg(&Vec::<u8>::from(packet));
                        }
                        Err(RecvError::Closed) => {
                            println!("rx: no sender for this channel");
                        }
                        Err(_) => {
                            println!("rx: lagged");
                        }
                    }
                    */
                    match rx.recv().await {
                        Some(mut packet) => {
                            packet.ssrc = 1811295701;
                            println!("sending packet to rtc agent");
                            if let Err(s) = agent.send_msg(&mut packet) {
                                println!("error: {}", s);
                            }
                        }
                        _ => println!("rx: None"),
                    }
                }
            });
        } else {
            println!("session not found");
        }
    }

    pub fn get_credentials(&self) -> Result<(String, String), String> {
        // self.agent.read().unwrap().get_local_credentials()
        todo!()
    }
}
