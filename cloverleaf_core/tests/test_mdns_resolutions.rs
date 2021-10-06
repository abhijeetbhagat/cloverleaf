use cloverleaf_core::mdns_resolver;
#[test]
fn test_mdns_resolutions() {
    let result = mdns_resolver("localhost");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "127.0.0.1".to_owned());
}
