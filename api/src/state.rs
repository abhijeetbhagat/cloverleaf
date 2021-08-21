use crate::payload::Payload;
use core::{
    sdp::{parse_candidate, Sdp},
    IceAgent,
};
use glib::MainContext;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::sync::RwLock;

pub struct CloverLeafState {
    agent: RwLock<IceAgent>,
}

impl CloverLeafState {
    pub fn new() -> Result<CloverLeafState, String> {
        let main_ctx = MainContext::new();
        let agent = match IceAgent::new(main_ctx) {
            Ok(agent) => agent,
            _ => return Err("error init'ing agent".into()),
        };
        Ok(Self {
            agent: RwLock::new(agent),
        })
    }

    pub fn handle(&self, payload: Json<Payload>) {
        match &payload.pt {
            Offer => {}
            Answer => {
                println!("we recvd an answer: {}", payload.payload);
                let sdp = Sdp::from(payload.payload.as_str());
                self.agent
                    .read()
                    .unwrap()
                    .set_remote_credentials(&sdp.ufrag, &sdp.pwd);
            }
            Candidate => {
                if let Ok(candidate) = parse_candidate(&payload.payload) {
                    self.agent.write().unwrap().set_remote_candidate(candidate);
                }
            }
        }
    }

    pub fn get_credentials(&self) -> Result<(String, String), String> {
        self.agent.read().unwrap().get_local_credentials()
    }
}
