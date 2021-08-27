use cloverleaf_core::IceAgent;
use glib::MainContext;

#[test]
fn test_ice_agent_create() {
    let agent = IceAgent::new(MainContext::new());
    assert!(agent.is_ok());
}
