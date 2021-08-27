use cloverleaf_core::IceAgent;
use glib::MainContext;

#[test]
fn test_ice_agent_get_local_credentials() {
    let agent = IceAgent::new(MainContext::new());
    assert!(agent.is_ok());
    let creds = agent.unwrap().get_local_credentials();
    assert!(creds.is_ok());
    let (ufrag, pwd) = creds.unwrap();
    println!("{}, {}", ufrag, pwd);
    assert_eq!(ufrag.len(), 4);
}
