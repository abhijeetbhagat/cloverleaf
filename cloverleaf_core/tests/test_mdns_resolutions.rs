use cloverleaf_core::mdns_resolve;
#[ignore]
#[test]
fn test_mdns_resolutions() {
    let result = mdns_resolve("localhost");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "127.0.0.1".to_owned());
}

#[test]
fn test_mdns_resolution_local() {
    let result = mdns_resolve("7b03a07f-3702-4a33-a8e8-83ae730ee340.local");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "192.168.1.10".to_owned());
}
