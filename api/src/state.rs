use crate::request::Payload;
use core::IceAgent;
use glib::MainContext;
use rocket::serde::{json::Json, Deserialize, Serialize};

pub(crate) struct CloverLeafState {
    agent: IceAgent,
}

impl CloverLeafState {
    pub fn new() -> Result<CloverLeafState, String> {
        let main_ctx = MainContext::new();
        let agent = match IceAgent::new(main_ctx) {
            Ok(agent) => agent,
            _ => return Err("error init'ing agent".into()),
        };
        Ok(Self { agent })
    }

    pub fn handle(payload: Json<Payload>) {
        match payload.pt {
            Offer => {}
            Answer => {
                println!("we recvd an answer: {}", payload.payload);
            }
        }
    }

    pub fn get_credentials(&self) -> Result<(String, String)> {
        self.agent.get_local_credentials()
    }
}
