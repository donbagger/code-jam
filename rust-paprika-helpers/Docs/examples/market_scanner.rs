use paprika_helpers::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Rust Paprika Helpers Demo");
    println!("{}", "=".repeat(50));

    // Example 1: Get Networks
    println!("\n📡 Available Networks:");
    match get_networks().await {
        Ok(networks) => {
            println!("Found {} networks", networks.len());
            for (i, network) in networks.iter().take(5).enumerate() {
                println!("  {}. {} ({})", i + 1, network.display_name, network.id);
            }
        }
        Err(e) => println!("Error getting networks: {}", e),
    }

    // Example 2: Get Ethereum Pools
    println!("\n🏊 Top Ethereum Pools:");
    let params = ApiParams::new()
        .limit(5)
        .order_by("volume_usd");
    
    match get_network_pools("ethereum", Some(params)).await {
        Ok(response) => {
            for (i, pool) in response.pools.iter().enumerate() {
                println!("  {}. {}: ${}",
                    i + 1,
                    pool.dex_name,
                    format_number(pool.volume_usd, 0)
                );
            }
        }
        Err(e) => println!("Error getting pools: {}", e),
    }

    // Example 3: Search for Uniswap
    println!("\n🔍 Search Results for 'uniswap':");
    match search_entities("uniswap").await {
        Ok(response) => {
            println!("  Tokens: {}, Pools: {}, DEXes: {}",
                response.tokens.len(),
                response.pools.len(), 
                response.dexes.len()
            );
        }
        Err(e) => println!("Error searching: {}", e),
    }

    // Example 4: System Stats
    println!("\n📊 System Statistics:");
    match get_system_stats().await {
        Ok(stats) => {
            println!("  Chains: {}, Factories: {}, Pools: {}, Tokens: {}",
                stats.chains, stats.factories, stats.pools, stats.tokens);
        }
        Err(e) => println!("Error getting stats: {}", e),
    }

    // Example 5: Filter and Analyze
    println!("\n📈 Analysis Examples:");
    if let Ok(pools_response) = get_network_pools("ethereum", Some(ApiParams::new().limit(20))).await {
        let pools = &pools_response.pools;

        // Filter by volume
        let high_volume = filter_by_volume(pools, 50_000_000.0); // $50M+
        println!("  High volume pools (>$50M): {}", high_volume.len());

        // Get top movers
        let top_movers = filter_by_price_change(pools, 5.0); // 5%+ change
        println!("  Significant movers (>5% change): {}", top_movers.len());

        // Detect anomalies
        let anomalies = detect_anomalies(pools, "volume_usd", 2.0);
        println!("  Volume anomalies detected: {}", anomalies.len());

        // Analyze pool activity
        if let Some(pool) = pools.first() {
            let analysis = analyze_pool_activity(pool);
            if let Some(activity_score) = analysis.get("activity_score") {
                println!("  First pool activity score: {:.2}", 
                    activity_score.as_f64().unwrap_or(0.0));
            }
        }
    }

    // Example 6: Utility Functions
    println!("\n🛠️ Utility Examples:");
    println!("  Formatted number: {}", format_number(1234567.89, 2));
    println!("  Formatted percentage: {}", format_percentage(15.567, 2));
    println!("  Price change calculation: {:.2}%", calculate_price_change(110.0, 100.0));

    let is_valid = validate_token_address("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
    println!("  USDC address validation: {}", is_valid);

    let timestamp = create_timestamp(7);
    println!("  Timestamp 7 days ago: {}", &timestamp[..10]);

    // Example 7: Async Operations
    println!("\n⚡ Async Operations:");
    let networks = vec!["ethereum".to_string(), "polygon".to_string()];
    match async_get_multiple_pools(&networks, 3).await {
        Ok(results) => {
            println!("  Got concurrent data from {} networks", results.len());
            for network in &networks {
                if results.contains_key(network) {
                    println!("    - {}: ✅", network);
                }
            }
        }
        Err(e) => println!("  Error with async pools: {}", e),
    }

    // Example 8: Market Overview
    println!("\n🌍 Market Overview:");
    match get_market_overview().await {
        Ok(overview) => {
            println!("  System: {} chains, {} pools",
                overview.system_stats.chains, overview.system_stats.pools);
            println!("  Networks analyzed: {}", overview.network_overview.len());
        }
        Err(e) => println!("  Error getting overview: {}", e),
    }

    // Example 9: Advanced Analysis
    println!("\n🔬 Advanced Analysis:");
    let networks_to_analyze = vec!["ethereum".to_string(), "solana".to_string()];
    
    match async_get_multiple_pools(&networks_to_analyze, 20).await {
        Ok(results) => {
            println!("🔥 Multi-Network Market Scanner");
            println!("{}", "=".repeat(50));
            
            for (network, data) in results {
                if let Ok(pools_response) = serde_json::from_value::<PoolsResponse>(data) {
                    let pools = &pools_response.pools;
                    let top_movers = filter_by_price_change(pools, 15.0);
                    let anomalies = detect_anomalies(pools, "volume_usd", 2.5);
                    
                    println!("\n🔥 {} - Top Movers:", network.to_uppercase());
                    for (i, pool) in top_movers.iter().take(3).enumerate() {
                        let change = format_percentage(pool.last_price_change_usd_24h, 2);
                        let volume = format_number(pool.volume_usd, 0);
                        println!("  {}. {}: {} | ${} volume", 
                            i + 1, pool.dex_name, change, volume);
                    }
                    
                    if !anomalies.is_empty() {
                        println!("🚨 {} - Anomalies Detected:", network.to_uppercase());
                        for (i, anomaly) in anomalies.iter().take(2).enumerate() {
                            let volume = format_number(anomaly.value, 0);
                            if let Ok(pool) = serde_json::from_value::<Pool>(anomaly.item.clone()) {
                                println!("  {}. {}: ${} volume (Z-score: {:.2})", 
                                    i + 1, pool.dex_name, volume, anomaly.z_score);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error in advanced analysis: {}", e),
    }

    println!("\n✅ Demo completed successfully!");
    println!("🚀 Rust helpers are ready for the Paprika Vibe-Code challenge!");

    Ok(())
} 