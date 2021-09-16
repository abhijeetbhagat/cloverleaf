use cloverleaf_rtsp::client::RTSPSource;
use cloverleaf_rtsp::MediaType;

#[test]
fn test_source() {
    let mut source = RTSPSource::new("").unwrap();
    source.on_packet(|p| {
        println!("{:?}", p);
    });
    source.start(MediaType::All);
}
