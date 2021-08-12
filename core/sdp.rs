pub struct Sdp {
    ufrag: String,
    pwd: String,
}

pub fn create_sdp(sdp: &Sdp) -> String {
    format!(concat!("v=0\r\n",
        "o=- 1625733337583270 1 IN IP4 192.168.1.2\r\n",
        "s=Mountpoint 99\r\n",
        "t=0 0\r\n",
        "a=group:BUNDLE video\r\n",
        "a=msid-semantic: WMS cloverleaf\r\n",
        "m=video 9 UDP/TLS/RTP/SAVPF 96 97\r\n",
        "c=IN IP4 192.168.1.2\r\n",
        "a=sendonly\r\n",
        "a=mid:video\r\n",
        "a=rtcp-mux\r\n",
        "a=ice-ufrag:{}\r\n",
        "a=ice-pwd:{}\r\n",
        "a=ice-options:trickle\r\n",
        "a=fingerprint:sha-256 5B:F5:FC:7E:5F:1B:21:86:37:A4:06:EF:48:A1:E0:E5:DB:93:5F:4F:05:B3:89:9A:11:E5:1E:15:81:D7:11:13\r\n",
        "a=setup:actpass\r\n",
        "a=rtpmap:96 H264/90000\r\n",
        "a=fmtp:96 profile-level-id=42e01f;packetization-mode=1\r\n",
        "a=rtcp-fb:96 nack\r\n",
        "a=rtcp-fb:96 nack pli\r\n",
        "a=rtcp-fb:96 goog-remb\r\n",
        "a=extmap:1 urn:ietf:params:rtp-hdrext:sdes:mid\r\n",
        "a=rtpmap:97 rtx/90000\r\n",
        "a=fmtp:97 apt=96\r\n",
        "a=ssrc-group:FID 1811295701 180905187\r\n",
        "a=msid:cloverleaf cloverleafv0\r\n",
        "a=ssrc:1811295701 cname:cloverleaf\r\n",
        "a=ssrc:1811295701 msid:cloverleaf cloverleafv0\r\n",
        "a=ssrc:1811295701 mslabel:cloverleaf\r\n",
        "a=ssrc:1811295701 label:cloverleafv0\r\n",
        "a=ssrc:180905187 cname:cloverleaf\r\n",
        "a=ssrc:180905187 msid:cloverleaf cloverleafv0\r\n",
        "a=ssrc:180905187 mslabel:cloverleaf\r\n",
        "a=ssrc:180905187 label:cloverleafv0\r\n",
        "a=candidate:1 1 udp 2015363327 192.168.1.2 42933 typ host\r\n",
        "a=end-of-candidates\r\n",
        ), sdp.ufrag, sdp.pwd)
}