use cloverleaf_core::sdp::parse_candidate;
use cloverleaf_core::sdp::Sdp;
use cloverleaf_core::CandidateType;

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

    let text = "candidate:4 1 TCP 2105458943 9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local 9 typ host tcptype active";
    let candidate = parse_candidate(text);
    assert!(candidate.is_ok());
    let candidate = candidate.unwrap();
    assert_eq!(
        candidate.typ,
        CandidateType::HostTcp("9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local".into())
    );

    let text = "candidate:1 1 UDP 1685987327 103.208.69.28 19828 typ srflx raddr 0.0.0.0 rport 0";
    let candidate = parse_candidate(text);
    assert!(candidate.is_ok());
    let candidate = candidate.unwrap();
    assert_eq!(
        candidate.typ,
        CandidateType::ServerReflexive("0.0.0.0".into(), "0".parse().unwrap())
    );

    let text = "candidate:4077567720 1 udp 2113937151 068fdcb4-42c6-417c-b5c3-089ac79ce82d.local 43023 typ host generation 0 ufrag pfsh network-cost 999";
    let candidate = parse_candidate(text);
    match &candidate {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
        }
    }
    assert!(candidate.is_ok());
    let candidate = candidate.unwrap();
    assert_eq!(candidate.typ, CandidateType::HostUdp);
}
