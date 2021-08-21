use crate::payload::Payload;
use crate::state::CloverLeafState;
use core::sdp::{create_sdp, Sdp};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

#[get("/")]
pub fn initiate(state: &State<CloverLeafState>) -> status::Custom<content::Json<String>> {
    if let Ok((ufrag, pwd)) = state.get_credentials() {
        let sdp = Sdp { ufrag, pwd };
        let sdp = create_sdp(&sdp);
        let offer = format!("{{\"type\": \"offer\", \"sdp\": \"{}\"}}", sdp);
        return status::Custom(Status::Accepted, content::Json(offer));
    }

    status::Custom(
        Status::Accepted,
        content::Json("{{\"type\": \"error\"}}".into()),
    )
}

#[post("/answer", data = "<payload>")]
pub fn recv_answer(state: &State<CloverLeafState>, payload: Json<Payload>) {
    state.handle(payload);
}

#[post("/candidate", data = "<payload>")]
pub fn recv_candidate(state: &State<CloverLeafState>, payload: Json<Payload>) {
    state.handle(payload);
}
