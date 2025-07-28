#!/usr/bin/env node
/**
 * 🟨 Paprika Helpers - Simple JavaScript Example
 * 
 * This demonstrates the power of the 67 helper functions - 
 * creating a useful DeFi application in just a few lines!
 */

// Import from the internal implementation
const path = require('path');
const fs = require('fs');

// Read the paprika helpers from Docs/internal
const helpersPath = path.join(__dirname, 'Docs', 'internal', 'paprika_helpers.js');
const helpersCode = fs.readFileSync(helpersPath, 'utf8');

// Evaluate the helpers code to make functions available
eval(helpersCode);

async function main() {
    console.log("🟨 Paprika Helpers - Simple Example");
    console.log("=".repeat(40));

    try {
        // Get all supported blockchain networks
        const networks = await get_networks();
        console.log(`📡 Found ${networks.length} blockchain networks`);

        // Get top Ethereum pools
        const pools = await get_network_pools("ethereum", { limit: 3 });
        console.log(`\n🏊 Top 3 Ethereum Pools:`);
        
        for (let i = 0; i < pools.pools.length; i++) {
            const pool = pools.pools[i];
            const volume = format_number(pool.volume_usd, 0);
            console.log(`  ${i + 1}. ${pool.dex_name}: $${volume}`);
        }

        // Search for protocols
        const search = await search_entities("uniswap");
        const total = search.tokens.length + search.pools.length + search.dexes.length;
        console.log(`\n🔍 Found ${total} Uniswap-related entities`);

        // System statistics
        const stats = await get_system_stats();
        console.log(`\n📊 DeFi Ecosystem: ${stats.chains} chains, ${stats.pools} pools, ${stats.tokens} tokens`);

        console.log("\n✅ That's it! Powerful DeFi data in just a few lines of JavaScript! 🚀");

    } catch (error) {
        console.error("Error:", error.message);
    }
}

// Run the example
if (require.main === module) {
    main().catch(console.error);
} 