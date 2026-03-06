# Shred Decoder Starter Template

A Rust-based Solana shred decoder that listens for raw shreds via UDP and reassembles them into decoded transactions in real time.

## Prerequisites

- Rust (latest stable)
- A server (DigitalOcean, Vultr, AWS, or any VPS provider)

## Setup

1. **Create an account on [solami.fast](https://solami.fast)**

2. **Get a server** from DigitalOcean, Vultr, AWS, or any cloud provider. Available regions: **AMS** (Amsterdam), **FRA** (Frankfurt), and **NYC** (New York).

3. **Create a new ShredStream** from your Solami dashboard. Set your server IP and port to `20000`.

4. **Clone and run the decoder** on your server:

   ```bash
   git clone https://github.com/useSolami/shred-decoder-starter-template
   cd shred-decoder-starter-template
   cargo run -r
   ```

   The decoder will start listening on `0.0.0.0:20000` for incoming shreds.

## How It Works

The decoder receives raw Solana shreds over UDP, reassembles them by slot, deserializes the entries, and prints decoded transactions with their signatures, version (legacy/v0), and instruction count. This is a starter template — modify it to suit your needs.

## Configuration

- **Listen address/port**: Change `LISTEN_ADDR` in `src/main.rs` (default: `0.0.0.0:20000`)
