#!/usr/bin/env node
/**
 * 🔥 Multi-Network Market Scanner
 * A ≤50-line utility that scans multiple networks for top movers, 
 * high volume pools, and market anomalies. Built with the complete 
 * Paprika Vibe-Code Helper Library.
 * 
 * Features:
 * - Cross-network analysis
 * - Anomaly detection
 * - Market overview
 * - Real-time alerts
 */

const {
    getMarketOverview,
    getTopMovers,
    getHighVolumePools,
    getNetworkPools,
    extractPools,
    detectAnomalies,
    formatNumber,
    formatPercentage,
    printTable
} = require('./paprika_helpers.js');

async function main() {
    // Configuration
    const networks = ['ethereum', 'solana', 'polygon'];
    const minVolume = 5000000; // $5M+ volume
    
    console.log('🚀 Multi-Network Market Scanner Starting...');
    console.log('='.repeat(50));
    
    // Get market overview
    const overview = await getMarketOverview();
    console.log(`🌐 Total Pools: ${overview.system_stats.pools.toLocaleString()}`);
    console.log(`🏗️  Total DEXes: ${overview.system_stats.factories.toLocaleString()}`);
    
    while (true) {
        for (const network of networks) {
            // Get top movers and high volume pools
            const topMovers = await getTopMovers(network, 5, minVolume);
            const highVolume = await getHighVolumePools(network, 5);
            
            // Detect anomalies
            const pools = await getNetworkPools(network, { limit: 50 });
            const poolList = extractPools(pools);
            const anomalies = detectAnomalies(poolList, 'volume_usd', 2.5);
            
            console.log(`\n🔥 ${network.toUpperCase()} - Top Movers:`);
            for (const pool of topMovers) {
                const change = formatPercentage(pool.last_price_change_usd_24h);
                const volume = formatNumber(pool.volume_usd);
                console.log(`  ${pool.dex_name}: ${change} | ${volume} volume`);
            }
            
            if (anomalies.length > 0) {
                console.log(`🚨 ${network.toUpperCase()} - Anomalies Detected:`);
                for (const anomaly of anomalies.slice(0, 3)) {
                    const volume = formatNumber(anomaly.volume_usd);
                    console.log(`  ${anomaly.dex_name}: ${volume} volume (anomaly)`);
                }
            }
        }
        
        console.log('\n' + '='.repeat(50));
        await new Promise(resolve => setTimeout(resolve, 300000)); // 5 minutes
    }
}

// Handle graceful shutdown
process.on('SIGINT', () => {
    console.log('\n👋 Shutting down gracefully...');
    process.exit(0);
});

// Run the main function
main().catch(console.error); 