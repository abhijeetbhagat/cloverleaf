use cloverleaf_core::IceAgent;
use glib::MainContext;

#[test]
fn test_create() {
    let agent = IceAgent::new(MainContext::new());
    assert!(agent.is_ok());
}

#[test]
fn test_getting_local_candidates() {
    let agent = IceAgent::new(MainContext::new());
    assert!(agent.is_ok());
    let agent = agent.unwrap();
    let lcands = agent.get_local_candidates();
    assert!(lcands.is_ok());
    let lcands = lcands.unwrap();
    assert!(lcands.len() > 0);
}
