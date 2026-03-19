use log::info;
use solana_stream_sdk::{
    shreds_udp::{decode_udp_datagram, ShredsUdpConfig, ShredsUdpState},
    UdpShredReceiver,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let mut cfg = ShredsUdpConfig::defaults();
    cfg.bind_addr = "0.0.0.0:20000".to_string();

    let mut receiver = UdpShredReceiver::bind(&cfg.bind_addr, None).await?;
    info!("listening on {}", receiver.local_addr()?);

    let state = ShredsUdpState::new(&cfg);

    loop {
        let datagram = match receiver.recv_raw().await {
            Ok(d) => d,
            Err(_) => continue,
        };

        let Some(decoded) = decode_udp_datagram(&datagram, &state, &cfg).await else {
            continue;
        };

        println!("slot={}", decoded.shred.slot());
    }
}
