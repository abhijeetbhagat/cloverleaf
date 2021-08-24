use core::IceAgent;
use glib::MainContext;

#[test]
fn test_ice_agent_set_remote_credentials() {
    let agent = IceAgent::new(MainContext::new());
    assert!(agent.is_ok());
    let status = agent.unwrap().set_remote_credentials("abhi", "pass");
    assert!(status.is_ok());
}
