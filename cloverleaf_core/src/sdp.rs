use crate::{candidate_type::CandidateType, ice_candidate::IceCandidate};
use regex::Regex;

pub struct Sdp {
    pub ufrag: String,
    pub pwd: String,
}

impl Default for Sdp {
    fn default() -> Self {
        Sdp {
            ufrag: "".into(),
            pwd: "".into(),
        }
    }
}

impl From<&str> for Sdp {
    fn from(raw: &str) -> Self {
        let mut sdp = Sdp::default();
        for line in raw.split("\r\n") {
            if line.contains("ice-ufrag") {
                // a=ice-pwd:99ad05513f44705637769b05c7e86c0b
                //    a=ice-ufrag:ae11196c
                let re = Regex::new(r"a=ice-ufrag:([a-z0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                sdp.ufrag = caps.get(1).unwrap().as_str().into();
                continue;
            }
            if line.contains("ice-pwd") {
                let re = Regex::new(r"a=ice-pwd:([a-z0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                sdp.pwd = caps.get(1).unwrap().as_str().into();
                continue;
            }
            if line.contains("candidate") {
                parse_candidate(line).unwrap();
            }
        }
        sdp
    }
}

pub fn parse_candidate(line: &str) -> Result<IceCandidate, String> {
    // candidate:0 1 UDP 2122187007 9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local 48155 typ host
    // candidate:2 1 UDP 2122252543 475400ac-4273-4245-90fb-7b6c97fe06f7.local 53547 typ host
    // candidate:4 1 TCP 2105458943 9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local 9 typ host tcptype active
    // candidate:5 1 TCP 2105524479 475400ac-4273-4245-90fb-7b6c97fe06f7.local 9 typ host tcptype active
    // candidate:1 1 UDP 1685987327 103.208.69.28 19828 typ srflx raddr 0.0.0.0 rport 0
    //
    // int res = sscanf(candidate, "%32s %30u %3s %30u %49s %30u typ %5s %*s %39s %*s %30u",
    // let re = Regex::new(r"candidate:([0-9]+)\s([0-9]+)\s(UDP|TCP)\s([0-9]+)\s([a-z0-9\-\.]+)\s([0-9]+)\styp\s(host|srflx|prflx)\s(raddr|tcptype)\s([a-z]+|[0-9\.]+)\s(rport)\s([0-9]+)").unwrap();

    /*
    candidate:
    0 -> foundation
    1 -> component
    UDP -> transport
    2122187007 -> priority
    9971baf2-00e6-4bb3-b954-7a61b4eb8daf.local -> ip
    48155 -> port
    typ -> typ
    host -> candidate type
    */
    println!("parsing this line: {line}");
    let re = Regex::new(r"candidate:([0-9]+)\s([0-9]+)\s(UDP|TCP)\s([0-9]+)\s([a-z0-9\-\.]+)\s([0-9]+)\styp\s(host|srflx|prflx)\s*(raddr|tcptype)*\s*([a-z]+|[0-9\.]+)*\s*(rport)*\s*([0-9]+)*").unwrap();
    if let Some(caps) = re.captures(line) {
        let foundation = caps
            .get(1)
            .ok_or::<String>("can't parse foundation".into())?;
        let component = caps
            .get(2)
            .ok_or::<String>("can't parse component".into())?;
        let transport = caps
            .get(3)
            .ok_or::<String>("can't parse transport".into())?;
        let priority = caps.get(4).ok_or::<String>("can't parse priority".into())?;
        let ip = caps.get(5).ok_or::<String>("can't parse ip".into())?;
        let port = caps.get(6).ok_or::<String>("can't parse port".into())?;
        let typ = caps.get(7).ok_or::<String>("can't parse typ".into())?;

        let typ = match typ.as_str() {
            "host" if transport.as_str() == "UDP" => CandidateType::HostUdp,
            "host" if transport.as_str() == "TCP" => CandidateType::HostTcp(ip.as_str().into()),
            "srflx" => {
                let rip = caps.get(9).ok_or::<String>("can't parse rip".into())?;
                let rport = caps.get(11).ok_or::<String>("can't parse rport".into())?;
                CandidateType::ServerReflexive(rip.as_str().into(), rport.as_str().parse().unwrap())
            }
            _ => return Err(format!("unknown type: {}", typ.as_str())),
        };

        IceCandidate::new(
            foundation.as_str().into(),
            component.as_str().parse().unwrap(),
            transport.as_str().into(),
            priority.as_str().parse().unwrap(),
            ip.as_str().into(),
            port.as_str().parse().unwrap(),
            typ,
        )
    } else {
        println!("[parse_candidate] cannot parse candidate");
        Err("candidate cant be parsed".into())
    }
}

pub fn create_sdp(sdp: &Sdp, candidate: &IceCandidate, fingerprint: &str) -> String {
    format!(
        concat!(
            "v=0\\r\\n",
            "o=- 1625733337583270 1 IN IP4 192.168.1.2\\r\\n",
            "s=Mountpoint 99\\r\\n",
            "t=0 0\\r\\n",
            "a=group:BUNDLE video\\r\\n",
            "a=msid-semantic: WMS cloverleaf\\r\\n",
            "m=video 9 UDP/TLS/RTP/SAVPF 96 97\\r\\n",
            "c=IN IP4 192.168.1.2\\r\\n",
            "a=sendonly\\r\\n",
            "a=mid:video\\r\\n",
            "a=rtcp-mux\\r\\n",
            "a=ice-ufrag:{}\\r\\n",
            "a=ice-pwd:{}\\r\\n",
            "a=ice-options:trickle\\r\\n",
            // "a=fingerprint:sha-256 5B:F5:FC:7E:5F:1B:21:86:37:A4:06:EF:48:A1:E0:E5:DB:93:5F:4F:05:B3:89:9A:11:E5:1E:15:81:D7:11:13\\r\\n",
            "a=fingerprint:{}\\r\\n",
            "a=setup:actpass\\r\\n",
            "a=rtpmap:96 H264/90000\\r\\n",
            "a=fmtp:96 profile-level-id=42e01f;packetization-mode=1\\r\\n",
            "a=rtcp-fb:96 nack\\r\\n",
            "a=rtcp-fb:96 nack pli\\r\\n",
            "a=rtcp-fb:96 goog-remb\\r\\n",
            "a=extmap:1 urn:ietf:params:rtp-hdrext:sdes:mid\\r\\n",
            "a=rtpmap:97 rtx/90000\\r\\n",
            "a=fmtp:97 apt=96\\r\\n",
            "a=ssrc-group:FID 1811295701 180905187\\r\\n",
            "a=msid:cloverleaf cloverleafv0\\r\\n",
            "a=ssrc:1811295701 cname:cloverleaf\\r\\n",
            "a=ssrc:1811295701 msid:cloverleaf cloverleafv0\\r\\n",
            "a=ssrc:1811295701 mslabel:cloverleaf\\r\\n",
            "a=ssrc:1811295701 label:cloverleafv0\\r\\n",
            "a=ssrc:180905187 cname:cloverleaf\\r\\n",
            "a=ssrc:180905187 msid:cloverleaf cloverleafv0\\r\\n",
            "a=ssrc:180905187 mslabel:cloverleaf\\r\\n",
            "a=ssrc:180905187 label:cloverleafv0\\r\\n",
            "a={}\\r\\n",
            "a=end-of-candidates\\r\\n",
        ),
        sdp.ufrag, sdp.pwd, fingerprint, candidate
    )
}
