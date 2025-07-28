# Paprika Helpers for Rust 🦀

[![Crates.io](https://img.shields.io/crates/v/paprika-helpers.svg)](https://crates.io/crates/paprika-helpers)
[![Documentation](https://docs.rs/paprika-helpers/badge.svg)](https://docs.rs/paprika-helpers)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**67 powerful functions** for DeFi development. Get real-time blockchain data with just a few lines of Rust! 

## 🚀 Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
paprika-helpers = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
```

## 💡 Create Something Amazing in 5 Minutes

```rust
use paprika_helpers::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Get live data from 25+ blockchains
    let networks = get_networks().await?;
    println!("📡 {} networks available", networks.len());

    // Analyze top Ethereum pools
    let pools = get_network_pools("ethereum", Some(
        ApiParams::new().limit(5)
    )).await?;
    
    // Find volume anomalies 
    let anomalies = detect_anomalies(&pools.pools, "volume_usd", 2.0);
    println!("🚨 {} volume anomalies detected", anomalies.len());

    // Search across the entire DeFi ecosystem
    let search = search_entities("uniswap").await?;
    println!("🔍 Found {} Uniswap entities", 
        search.tokens.len() + search.pools.len() + search.dexes.len());

    Ok(())
}
```

## ✨ What You Can Build

- **🔥 DeFi Portfolio Trackers** - Real-time value monitoring
- **📊 Market Analytics Dashboards** - Volume, price, liquidity analysis  
- **🚨 Anomaly Detection Systems** - Identify unusual market activity
- **🤖 Trading Bots** - Automated strategies with live data
- **📈 Price Prediction Models** - ML with historical OHLCV data
- **💎 Yield Farming Optimizers** - Find best liquidity opportunities

## 🎯 Key Features

- **🌐 25+ Blockchains** - Ethereum, Solana, BSC, Polygon, and more
- **⚡ Ultra Fast** - Zero-cost abstractions, memory-safe Rust
- **🔒 Type Safe** - Compile-time guarantees, no runtime errors
- **📊 Rich Analytics** - Volume, liquidity, anomaly detection
- **⚙️ Easy Integration** - Just add one dependency
- **📝 Complete Documentation** - Every function documented with examples

## 🏃‍♂️ Run the Example

```bash
git clone <repo>
cd paprika-helpers-rust
cargo run --bin example
```

## 📚 Function Categories

| Category | Functions | Description |
|----------|-----------|-------------|
| **🔗 Core API** | 12 | Direct blockchain data access |
| **📊 Data Extraction** | 15 | Parse and structure responses |
| **🔍 Filtering & Sorting** | 12 | Find exactly what you need |
| **📈 Analytics** | 10 | Advanced calculations and insights |
| **🛠️ Utilities** | 8 | Formatting, validation, I/O |
| **🚀 Advanced** | 5 | High-level market analysis |
| **⚡ Async** | 5 | Concurrent, high-performance operations |

## 🔗 More Resources

- [📖 Full Documentation](https://docs.rs/paprika-helpers)
- [🎯 More Examples](./example.rs)
- [🏗️ API Reference](https://api.dexpaprika.com)
- [💬 Community Discord](https://discord.gg/paprika)

## 📄 License

MIT License - build amazing things! 🚀

---

**⚡ Ready to build the future of DeFi? Start coding now! 🦀** 