use crate::connect::initiate;
use crate::state::CloverLeafState;

fn main() {
    let state = CloverLeafState::new();
    rocket::ignite()
        .manage(state)
        .mount("/", [initiate])
        .launch();
}
