#[macro_use]
extern crate rocket;
use api::connect::{initiate, recv_answer};
use api::state::CloverLeafState;

#[launch]
fn entry() -> _ {
    let state = CloverLeafState::new().unwrap();
    rocket::build()
        .manage(state)
        // .mount("/", [initiate])
        .mount("/", routes![initiate, recv_answer])
}
