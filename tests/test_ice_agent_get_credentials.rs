use glib::MainContext;
use rust_ice_agent::IceAgent;

#[test]
fn test_ice_agent_get_local_credentials() {
    let agent = IceAgent::new(MainContext::new());
    assert!(agent.is_ok());
    let creds = agent.unwrap().get_local_credentials();
    assert!(creds.is_ok());
}
