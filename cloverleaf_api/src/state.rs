use crate::payload::{Payload, PayloadType};
use cloverleaf_core::sdp::create_sdp;
use cloverleaf_core::{
    sdp::{parse_candidate, Sdp},
    IceAgent, Streamer, Viewer,
};
use cloverleaf_rtsp::RTPPacket;
use glib::MainContext;
use rocket::serde::json::Json;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock as TRwLock;
use uuid::Uuid;

pub struct CloverLeafState {
    temp_streams: Arc<RwLock<HashMap<String, IceAgent>>>,
    streams: Arc<RwLock<HashMap<String, Arc<RwLock<Sender<RTPPacket>>>>>>,
    tx: Arc<RwLock<Sender<RTPPacket>>>,
    active: Arc<RwLock<bool>>,
}

unsafe impl Send for CloverLeafState {}
unsafe impl Sync for CloverLeafState {}

impl CloverLeafState {
    pub fn new() -> Result<CloverLeafState, String> {
        let (tx, _) = broadcast::channel(20);
        Ok(Self {
            temp_streams: Arc::new(RwLock::new(HashMap::new())),
            streams: Arc::new(RwLock::new(HashMap::new())),
            tx: Arc::new(RwLock::new(tx)),
            active: Arc::new(RwLock::new(false)),
        })
    }

    /// creates a session, an ice agent representing this session
    /// and returns the sdp to be sent into an offer by the caller.
    pub fn create_session(&self) -> Result<(String, String), String> {
        let uuid = Uuid::new_v4();
        let streams = self.temp_streams.clone();
        let mut streams = streams.write().unwrap();
        let main_ctx = MainContext::new();
        if let Ok(agent) = IceAgent::new(main_ctx) {
            let (ufrag, pwd) = agent.get_local_credentials().unwrap();
            let sdp = Sdp { ufrag, pwd };
            if let Ok(lcands) = agent.get_local_candidates() {
                let sdp = create_sdp(&sdp, &lcands[0]);

                streams.insert(uuid.to_string(), agent);
                return Ok((uuid.to_string(), sdp));
            }
        }
        Err("there was an error".into())
    }

    /// extracts info from the answer
    pub fn process_answer(&self, payload: Json<Payload>) {
        println!("we recvd an answer: {}", payload.payload);
        let sdp = Sdp::from(payload.payload.as_str());
        let mut streams = self.temp_streams.write().unwrap();
        if let Some(agent) = streams.get_mut(&payload.session) {
            agent.set_remote_credentials(&sdp.ufrag, &sdp.pwd);
        }
    }

    /// adds a candidate to the ice agent associated with the
    /// supplied session.
    pub fn add_candidate(&self, payload: Json<Payload>) {
        if let Ok(candidate) = parse_candidate(&payload.payload) {
            let mut streams = self.temp_streams.write().unwrap();
            if let Some(agent) = streams.get_mut(&payload.session) {
                agent.add_remote_candidate(candidate);
            }
        }
    }

    /// starts the requested stream
    pub fn start(&self, payload: Json<Payload>) {
        let id = &payload.id;
        let session = &payload.session;
        let mut streams = self.temp_streams.write().unwrap();

        // spawn streaming if not running already
        if !*self.active.read().unwrap() {
            let source = Streamer::new(self.tx.clone());
            tokio::task::spawn(source.run());
            let mut active = self.active.write().unwrap();
            *active = true;
        }

        // remove session from the temp streams hashmap and transfer the ownership
        // of the ice agent to the spawned task
        if streams.contains_key(session) {
            let (_, mut agent) = streams.remove_entry(session).unwrap();
            let tx = self.tx.read().unwrap();
            let mut rx = tx.subscribe();
            tokio::task::spawn(async move {
                loop {
                    if let Ok(packet) = rx.recv().await {
                        agent.send_msg(&Vec::<u8>::from(packet));
                    }
                }
            });
        }
    }

    pub fn get_credentials(&self) -> Result<(String, String), String> {
        // self.agent.read().unwrap().get_local_credentials()
        todo!()
    }
}
