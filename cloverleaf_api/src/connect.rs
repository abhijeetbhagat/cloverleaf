use crate::payload::Payload;
use crate::state::CloverLeafState;
use cloverleaf_core::sdp::{create_sdp, Sdp};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;

#[get("/")]
pub async fn initiate(state: &State<CloverLeafState>) -> status::Custom<content::Json<String>> {
    if let Ok((session, sdp)) = state.create_session() {
        let offer = format!(
            "{{\"type\": \"offer\", \"session\": \"{}\", \"sdp\": \"{}\"}}",
            session, sdp
        );
        return status::Custom(Status::Accepted, content::Json(offer));
    } else {
        status::Custom(
            Status::Accepted,
            content::Json("\"type\": \"error\"".into()),
        )
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CloverLeafResponse {
    pt: String,
    status: String,
}

#[post("/answer", data = "<payload>")]
pub fn recv_answer(
    state: &State<CloverLeafState>,
    payload: Json<Payload>,
) -> Json<CloverLeafResponse> {
    state.process_answer(payload);
    Json(CloverLeafResponse {
        pt: "msg".into(),
        status: "success".into(),
    })
}

#[post("/candidate", data = "<payload>")]
pub fn recv_candidate(
    state: &State<CloverLeafState>,
    payload: Json<Payload>,
) -> Json<CloverLeafResponse> {
    state.add_candidate(payload);
    Json(CloverLeafResponse {
        pt: "msg".into(),
        status: "success".into(),
    })
}

#[post("/done", data = "<payload>")]
pub fn candidates_done(state: &State<CloverLeafState>, payload: Json<Payload>) {
    state.start(payload);
}
