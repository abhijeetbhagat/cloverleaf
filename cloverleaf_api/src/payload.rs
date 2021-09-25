use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PayloadType {
    Offer,
    Answer,
    Candidate,
    Watch,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload {
    pub pt: PayloadType,
    pub payload: String,
    pub id: String,
    pub session: String,
}
