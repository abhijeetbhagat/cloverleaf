use crate::request::Payload;
use crate::state::CloverLeafState;
use core::sdp::{create_sdp, Sdp};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[get("/")]
pub fn initiate(state: CloverLeafState) {
    if let Ok((ufrag, pwd)) = state.get_credentials() {
        let sdp = Sdp { ufrag, pwd };
        let sdp = create_sdp(sdp);
        status::Custom(
            Status::Accepted,
            content::Json(format!("{\"offer\": \"{}\"}", sdp)),
        )
    }
}

#[post("/answer", data = "<payload>")]
fn recv_answer(state: CloverLeafState, payload: Json<Payload>) {
    state.handle(payload);
}
