#[macro_use]
extern crate rocket;
use api::connect::{initiate, recv_answer};
use api::state::CloverLeafState;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Add CORS headers to response",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn entry() -> _ {
    let state = CloverLeafState::new().unwrap();

    rocket::build()
        .manage(state)
        // .mount("/", [initiate])
        .mount("/", routes![initiate, recv_answer])
        .attach(CORS)
}
