#[macro_use]
extern crate rocket;
use core::IceAgent;
use glib::MainContext;

#[get("/")]
fn initiate() {
    todo!()
}

fn main() {
    let main_ctx = MainContext::new();
    let agent = IceAgent::new(main_ctx);
    rocket::ignite()
        .manage(agent)
        .mount("/", [initiate])
        .launch()
}
