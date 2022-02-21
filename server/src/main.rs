#[macro_use]
extern crate rocket;
use cloverleaf_api::connect::{initiate, recv_answer, recv_candidate, watch};
use cloverleaf_api::state::CloverLeafState;

#[launch]
fn entry() -> _ {
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    let rocket = rocket::build();
    let figment = rocket.figment();
    let cert = figment.extract_inner("cert").unwrap();
    let key = figment.extract_inner("key").unwrap();
    let state = CloverLeafState::new(cert, key).unwrap();
    rocket
        .manage(state)
        .mount("/", routes![initiate, recv_answer, recv_candidate, watch])
        .attach(cors)
}
