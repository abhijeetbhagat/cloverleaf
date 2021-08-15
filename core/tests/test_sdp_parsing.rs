use core::sdp::parse_candidate;
use core::sdp::Sdp;

#[test]
fn test_sdp_parsing() {
    let text = "a=ice-pwd:99ad05513f44705637769b05c7e86c0b\r\na=ice-ufrag:ae11196c\r\n";
    let sdp = Sdp::from(text);
    assert!(sdp.ufrag.as_str() == "ae11196c");
    assert!(sdp.pwd.as_str() == "99ad05513f44705637769b05c7e86c0b");
}

#[test]
fn test_candidate_parsing() {
    let text =
        "candidate:0 1 UDP 2122187007 9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local 48155 typ host";
    let candidate = parse_candidate(text);
    assert!(candidate.is_ok());
}
