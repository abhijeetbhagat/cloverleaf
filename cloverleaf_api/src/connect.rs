use crate::payload::Payload;
use crate::state::CloverLeafState;
// use cloverleaf_core::sdp::{create_sdp, Sdp};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;

#[get("/")]
pub async fn initiate(state: &State<CloverLeafState>) -> status::Custom<content::Json<String>> {
    match state.create_session() {
        Ok((session, sdp)) => {
            let offer = format!(
                "{{\"type\": \"offer\", \"session\": \"{}\", \"sdp\": \"{}\"}}",
                session, sdp
            );
            return status::Custom(Status::Accepted, content::Json(offer));
        }
        Err(e) => status::Custom(
            Status::Accepted,
            content::Json(format!("{{\"type\": \"{}\"}}", e)),
        ),
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
pub fn candidates_done(
    state: &State<CloverLeafState>,
    payload: Json<Payload>,
) -> Json<CloverLeafResponse> {
    state.candidates_done(payload);
    Json(CloverLeafResponse {
        pt: "msg".into(),
        status: "success".into(),
    })
}

#[post("/watch", data = "<payload>")]
pub fn watch(state: &State<CloverLeafState>, payload: Json<Payload>) -> Json<CloverLeafResponse> {
    state.start(payload);
    Json(CloverLeafResponse {
        pt: "msg".into(),
        status: "success".into(),
    })
}
