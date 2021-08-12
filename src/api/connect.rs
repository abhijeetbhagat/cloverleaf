use core::IceAgent;
use rocket::http::Status;
use rocket::response::{content, status};

#[get("/")]
fn initiate(agent: IceAgent) {
    let (ufrag, pwd) = agent.get_local_credentials();
    let sdp = create_sdp(ufrag, pwd);
    status::Custom(
        Status::Accepted,
        content::Json(format!("{\"offer\": \"{}\"}", sdp)),
    )
}
