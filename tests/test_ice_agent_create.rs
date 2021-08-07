use rust_ice_agent::IceAgent;

#[test]
fn test_ice_agent_create() {
    let agent = IceAgent::new();
    assert!(agent.is_ok());
}
