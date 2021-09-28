#[macro_use]
extern crate rocket;
use cloverleaf_api::connect::{initiate, recv_answer, recv_candidate};
use cloverleaf_api::state::CloverLeafState;

#[launch]
fn entry() -> _ {
    let state = CloverLeafState::new().unwrap();

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::build()
        .manage(state)
        .mount("/", routes![initiate, recv_answer, recv_candidate])
        .attach(cors)
}
