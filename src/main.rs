use std::str::FromStr;

use log::info;
use solana_sdk::pubkey::Pubkey;
use solana_stream_sdk::{
    shreds_udp::{
        decode_udp_datagram, deshred_shreds_to_entries, insert_shred,
        DeshredPolicy, ShredInsertOutcome, ShredsUdpConfig, ShredsUdpState,
    },
    UdpShredReceiver,
};

const CREATE_DISCRIMINATOR: [u8; 8] = [0xd6, 0x90, 0x4c, 0xec, 0x5f, 0x8b, 0x31, 0xb4];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let pumpfun = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();
    let mut cfg = ShredsUdpConfig::defaults();
    cfg.bind_addr = "0.0.0.0:20000".to_string();

    let mut receiver = UdpShredReceiver::bind(&cfg.bind_addr, None).await?;
    info!("listening on {}", receiver.local_addr()?);

    let policy = DeshredPolicy {
        require_code_match: cfg.require_code_match,
    };
    let state = ShredsUdpState::new(&cfg);

    loop {
        let datagram = match receiver.recv_raw().await {
            Ok(d) => d,
            Err(_) => continue,
        };

        let Some(decoded) = decode_udp_datagram(&datagram, &state, &cfg).await else {
            continue;
        };

        match insert_shred(decoded, &datagram, &state, &cfg, &policy).await {
            ShredInsertOutcome::Ready(ready) => {
                let slot = ready.key.slot;
                match deshred_shreds_to_entries(&ready.shreds) {
                    Ok(entries) => {
                        for entry in &entries {
                            for tx in &entry.transactions {
                                let (account_keys, instructions) = match &tx.message {
                                    solana_sdk::message::VersionedMessage::Legacy(m) => {
                                        (&m.account_keys, &m.instructions)
                                    }
                                    solana_sdk::message::VersionedMessage::V0(m) => {
                                        (&m.account_keys, &m.instructions)
                                    }
                                };
                                let create_ix = instructions.iter().find(|ix| {
                                    account_keys.get(ix.program_id_index as usize)
                                        == Some(&pumpfun)
                                        && ix.data.len() >= 8
                                        && ix.data[..8] == CREATE_DISCRIMINATOR
                                });
                                let Some(ix) = create_ix else { continue };
                                let sig = tx
                                    .signatures
                                    .first()
                                    .map(|s| format!("{s}"))
                                    .unwrap_or_default();
                                let mint = ix
                                    .accounts
                                    .get(0)
                                    .and_then(|&i| account_keys.get(i as usize))
                                    .map(|k| format!("{k}"))
                                    .unwrap_or_default();
                                let creator = ix
                                    .accounts
                                    .get(5)
                                    .and_then(|&i| account_keys.get(i as usize))
                                    .map(|k| format!("{k}"))
                                    .unwrap_or_default();
                                println!(
                                    "slot={slot} sig={sig} mint={mint} creator={creator}"
                                );
                                // your logic here!!!!!!
                            }
                        }
                        state.remove_batch(&ready.key).await;
                    }
                    Err(_) => {
                        state.remove_batch(&ready.key).await;
                    }
                }
            }
            _ => {}
        }
    }
}
