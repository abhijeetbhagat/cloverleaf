use crate::payload::{Payload, PayloadType};
use cloverleaf_core::{
    sdp::{parse_candidate, Sdp},
    IceAgent,
};
use glib::MainContext;
use rocket::serde::json::Json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct CloverLeafState {
    streams: Arc<RwLock<HashMap<String, Vec<IceAgent>>>>,
}

unsafe impl Send for CloverLeafState {}
unsafe impl Sync for CloverLeafState {}

impl CloverLeafState {
    pub fn new() -> Result<CloverLeafState, String> {
        Ok(Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn handle(&self, payload: Json<Payload>) {
        match &payload.pt {
            PayloadType::Offer => {}
            PayloadType::Answer => {
                println!("we recvd an answer: {}", payload.payload);
                let sdp = Sdp::from(payload.payload.as_str());
                self.agent
                    .read()
                    .unwrap()
                    .set_remote_credentials(&sdp.ufrag, &sdp.pwd)
                    .unwrap();
            }
            PayloadType::Candidate => {
                if let Ok(candidate) = parse_candidate(&payload.payload) {
                    self.agent.write().unwrap().set_remote_candidate(candidate);
                }
            }
            PayloadType::Watch => {
                // TODO abhi: initiate streaming from the source like rtsp
                let id = &payload.payload;
                let main_ctx = MainContext::new();
                let agent = match IceAgent::new(main_ctx) {
                    Ok(agent) => agent,
                    _ => return Err("error init'ing agent".into()),
                };

                let streams = self.streams.write().unwrap();
                if streams[id] {}
            }
        }
    }

    pub fn get_credentials(&self) -> Result<(String, String), String> {
        self.agent.read().unwrap().get_local_credentials()
    }
}
