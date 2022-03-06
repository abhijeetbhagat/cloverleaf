use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PayloadType {
    Offer,
    Answer,
    Candidate,
    CandidatesDone,
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

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PayloadCandidate {
    pub candidate: String,
}
