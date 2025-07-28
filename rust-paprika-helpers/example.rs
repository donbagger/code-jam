//! Simple example showing how to use Paprika Helpers for Rust
//! 
//! This demonstrates the power of the 67 helper functions - 
//! creating a useful DeFi application in just a few lines!

use paprika_helpers::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 Paprika Helpers - Simple Example");
    println!("{}", "=".repeat(40));

    // Get all supported blockchain networks
    let networks = get_networks().await?;
    println!("📡 Found {} blockchain networks", networks.len());

    // Get top Ethereum pools
    let pools = get_network_pools("ethereum", Some(ApiParams::new().limit(3))).await?;
    println!("\n🏊 Top 3 Ethereum Pools:");
    
    for (i, pool) in pools.pools.iter().enumerate() {
        let volume = format_number(pool.volume_usd, 0);
        println!("  {}. {}: ${}", i + 1, pool.dex_name, volume);
    }

    // Search for protocols
    let search = search_entities("uniswap").await?;
    let total = search.tokens.len() + search.pools.len() + search.dexes.len();
    println!("\n🔍 Found {} Uniswap-related entities", total);

    // System statistics
    let stats = get_system_stats().await?;
    println!("\n📊 DeFi Ecosystem: {} chains, {} pools, {} tokens", 
        stats.chains, stats.pools, stats.tokens);

    println!("\n✅ That's it! Powerful DeFi data in just a few lines of Rust! 🚀");
    
    Ok(())
} 