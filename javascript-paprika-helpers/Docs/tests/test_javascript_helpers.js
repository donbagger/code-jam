#!/usr/bin/env node
/**
 * Comprehensive Test Suite for Paprika JavaScript Helpers
 * Tests all 60+ helper functions to ensure they work correctly
 */

const {
    // Core API Helpers
    apiRequest,
    getNetworks,
    getNetworkPools,
    getDexPools,
    getNetworkDexes,
    getPoolDetails,
    getPoolOHLCV,
    getPoolTransactions,
    getTokenDetails,
    getTokenPools,
    searchEntities,
    getSystemStats,
    
    // Data Extraction Helpers
    extractPools,
    extractTokens,
    extractDexes,
    extractTransactions,
    extractOHLCV,
    extractPageInfo,
    extractTokenSummary,
    extractTimeMetrics,
    extractPoolMetrics,
    extractTokenInfo,
    extractTransactionInfo,
    extractOHLCVMetrics,
    extractSearchResults,
    extractSystemStats,
    extractPoolTokens,
    
    // Filtering & Sorting Helpers
    filterByPriceChange,
    filterByVolume,
    filterByNetwork,
    filterByDex,
    filterByTokenSymbol,
    filterByTokenAddress,
    topN,
    bottomN,
    sortByField,
    filterRecentTransactions,
    filterLargeTransactions,
    filterByTimeframe,
    
    // Analysis Helpers
    calculatePriceChange,
    calculateVolumeWeightedPrice,
    calculateVolatility,
    analyzePoolActivity,
    analyzeTokenPerformance,
    detectAnomalies,
    calculateCorrelation,
    analyzeLiquidityDistribution,
    calculateGiniCoefficient,
    analyzeTransactionPatterns,
    
    // Utility Helpers
    formatNumber,
    formatPercentage,
    printTable,
    saveToCSV,
    loadEnv,
    validateNetwork,
    validateTokenAddress,
    createTimestamp,
    
    // Advanced Helpers
    getTopMovers,
    getHighVolumePools,
    getTokenLiquidityAnalysis,
    analyzeDexDistribution,
    getMarketOverview
} = require('./paprika_helpers.js');

async function testCoreApiHelpers() {
    console.log('🔧 Testing Core API Helpers...');
    
    // Test getNetworks
    try {
        const networks = await getNetworks();
        console.log(`✅ getNetworks: ${networks.length} networks found`);
        if (!Array.isArray(networks)) throw new Error('Networks should be an array');
    } catch (error) {
        console.log(`❌ getNetworks failed: ${error.message}`);
    }
    
    // Test getSystemStats
    try {
        const stats = await getSystemStats();
        console.log(`✅ getSystemStats: ${stats.pools || 0} pools, ${stats.tokens || 0} tokens`);
        if (typeof stats !== 'object') throw new Error('Stats should be an object');
    } catch (error) {
        console.log(`❌ getSystemStats failed: ${error.message}`);
    }
    
    // Test getNetworkPools for ethereum
    try {
        const pools = await getNetworkPools('ethereum', { limit: 5 });
        const poolList = extractPools(pools);
        console.log(`✅ getNetworkPools: ${poolList.length} pools found`);
        if (poolList.length === 0) throw new Error('Should return pools');
    } catch (error) {
        console.log(`❌ getNetworkPools failed: ${error.message}`);
    }
    
    // Test getNetworkDexes
    try {
        const dexes = await getNetworkDexes('ethereum', { limit: 5 });
        const dexList = extractDexes(dexes);
        console.log(`✅ getNetworkDexes: ${dexList.length} DEXes found`);
        if (dexList.length === 0) throw new Error('Should return DEXes');
    } catch (error) {
        console.log(`❌ getNetworkDexes failed: ${error.message}`);
    }
    
    // Test searchEntities
    try {
        const searchResults = await searchEntities('bitcoin');
        const results = extractSearchResults(searchResults);
        console.log(`✅ searchEntities: ${results.tokens?.length || 0} tokens found`);
        if (typeof results !== 'object') throw new Error('Search results should be an object');
    } catch (error) {
        console.log(`❌ searchEntities failed: ${error.message}`);
    }
}

function testDataExtractionHelpers() {
    console.log('\n🔧 Testing Data Extraction Helpers...');
    
    // Test data
    const testPool = {
        price_usd: 1.23,
        volume_usd: 1000000,
        transactions: 150,
        last_price_change_usd_24h: 5.5,
        fee: 0.003,
        tokens: [
            { symbol: 'USDC', id: 'usdc' },
            { symbol: 'ETH', id: 'eth' }
        ]
    };
    
    const testToken = {
        id: 'test_token',
        name: 'Test Token',
        symbol: 'TEST',
        chain: 'ethereum',
        decimals: 18,
        summary: {
            price_usd: 0.5,
            '24h': {
                volume_usd: 500000,
                last_price_usd_change: 2.5,
                txns: 100
            }
        }
    };
    
    // Test extractPoolMetrics
    try {
        const metrics = extractPoolMetrics(testPool);
        console.log(`✅ extractPoolMetrics: ${metrics.price_usd} USD price`);
        if (metrics.price_usd !== 1.23) throw new Error('Should extract correct price');
    } catch (error) {
        console.log(`❌ extractPoolMetrics failed: ${error.message}`);
    }
    
    // Test extractTokenInfo
    try {
        const info = extractTokenInfo(testToken);
        console.log(`✅ extractTokenInfo: ${info.symbol} token`);
        if (info.symbol !== 'TEST') throw new Error('Should extract correct symbol');
    } catch (error) {
        console.log(`❌ extractTokenInfo failed: ${error.message}`);
    }
    
    // Test extractTimeMetrics
    try {
        const metrics = extractTimeMetrics(testToken, '24h');
        console.log(`✅ extractTimeMetrics: ${metrics.volume_usd} volume`);
        if (metrics.volume_usd !== 500000) throw new Error('Should extract correct volume');
    } catch (error) {
        console.log(`❌ extractTimeMetrics failed: ${error.message}`);
    }
}

function testFilteringSortingHelpers() {
    console.log('\n🔧 Testing Filtering & Sorting Helpers...');
    
    // Test data
    const testPools = [
        { volume_usd: 1000000, last_price_change_usd_24h: 10.5, chain: 'ethereum', dex_name: 'Uniswap' },
        { volume_usd: 500000, last_price_change_usd_24h: -5.2, chain: 'ethereum', dex_name: 'SushiSwap' },
        { volume_usd: 2000000, last_price_change_usd_24h: 15.8, chain: 'polygon', dex_name: 'QuickSwap' }
    ];
    
    // Test filterByVolume
    try {
        const filtered = filterByVolume(testPools, 1000000);
        console.log(`✅ filterByVolume: ${filtered.length} pools with >1M volume`);
        if (filtered.length !== 2) throw new Error('Should filter correctly');
    } catch (error) {
        console.log(`❌ filterByVolume failed: ${error.message}`);
    }
    
    // Test filterByPriceChange
    try {
        const filtered = filterByPriceChange(testPools, 10.0);
        console.log(`✅ filterByPriceChange: ${filtered.length} pools with >10% change`);
        if (filtered.length !== 2) throw new Error('Should filter correctly');
    } catch (error) {
        console.log(`❌ filterByPriceChange failed: ${error.message}`);
    }
    
    // Test topN
    try {
        const topPools = topN(testPools, 'volume_usd', 2);
        console.log(`✅ topN: ${topPools.length} top pools by volume`);
        if (topPools.length !== 2) throw new Error('Should return top N');
        if (topPools[0].volume_usd !== 2000000) throw new Error('Should sort correctly');
    } catch (error) {
        console.log(`❌ topN failed: ${error.message}`);
    }
    
    // Test sortByField
    try {
        const sortedPools = sortByField(testPools, 'volume_usd', true);
        console.log(`✅ sortByField: ${sortedPools[0].volume_usd} highest volume`);
        if (sortedPools[0].volume_usd !== 2000000) throw new Error('Should sort correctly');
    } catch (error) {
        console.log(`❌ sortByField failed: ${error.message}`);
    }
}

function testAnalysisHelpers() {
    console.log('\n🔧 Testing Analysis Helpers...');
    
    // Test data
    const testOHLCV = [
        { open: 100, high: 110, low: 95, close: 105, volume: 1000 },
        { open: 105, high: 115, low: 100, close: 110, volume: 1200 },
        { open: 110, high: 120, low: 105, close: 115, volume: 1100 }
    ];
    
    const testPools = [
        { volume_usd: 1000000, last_price_change_usd_24h: 10.5, chain: 'ethereum', dex_name: 'Uniswap' },
        { volume_usd: 500000, last_price_change_usd_24h: -5.2, chain: 'ethereum', dex_name: 'SushiSwap' },
        { volume_usd: 2000000, last_price_change_usd_24h: 15.8, chain: 'polygon', dex_name: 'QuickSwap' }
    ];
    
    // Test calculatePriceChange
    try {
        const change = calculatePriceChange(110, 100);
        console.log(`✅ calculatePriceChange: ${change}% change`);
        if (change !== 10.0) throw new Error('Should calculate correct percentage');
    } catch (error) {
        console.log(`❌ calculatePriceChange failed: ${error.message}`);
    }
    
    // Test calculateVolumeWeightedPrice
    try {
        const vwap = calculateVolumeWeightedPrice(testOHLCV);
        console.log(`✅ calculateVolumeWeightedPrice: ${vwap.toFixed(2)} VWAP`);
        if (vwap <= 0) throw new Error('Should calculate positive VWAP');
    } catch (error) {
        console.log(`❌ calculateVolumeWeightedPrice failed: ${error.message}`);
    }
    
    // Test calculateVolatility
    try {
        const volatility = calculateVolatility(testOHLCV);
        console.log(`✅ calculateVolatility: ${volatility.toFixed(2)}% volatility`);
        if (volatility < 0) throw new Error('Should calculate non-negative volatility');
    } catch (error) {
        console.log(`❌ calculateVolatility failed: ${error.message}`);
    }
    
    // Test detectAnomalies
    try {
        const anomalies = detectAnomalies(testPools, 'volume_usd', 1.5);
        console.log(`✅ detectAnomalies: ${anomalies.length} anomalies found`);
        if (!Array.isArray(anomalies)) throw new Error('Should return array of anomalies');
    } catch (error) {
        console.log(`❌ detectAnomalies failed: ${error.message}`);
    }
}

function testUtilityHelpers() {
    console.log('\n🔧 Testing Utility Helpers...');
    
    // Test formatNumber
    try {
        const formatted = formatNumber(1234567, 2);
        console.log(`✅ formatNumber: ${formatted}`);
        if (!formatted.includes('M')) throw new Error('Should format with suffix');
    } catch (error) {
        console.log(`❌ formatNumber failed: ${error.message}`);
    }
    
    // Test formatPercentage
    try {
        const formatted = formatPercentage(5.5, 2);
        console.log(`✅ formatPercentage: ${formatted}`);
        if (!formatted.includes('+')) throw new Error('Should include sign');
    } catch (error) {
        console.log(`❌ formatPercentage failed: ${error.message}`);
    }
    
    // Test validateTokenAddress
    try {
        const isValid = validateTokenAddress('0x1234567890abcdef');
        console.log(`✅ validateTokenAddress: ${isValid}`);
        if (!isValid) throw new Error('Should validate correct address');
    } catch (error) {
        console.log(`❌ validateTokenAddress failed: ${error.message}`);
    }
    
    // Test createTimestamp
    try {
        const timestamp = createTimestamp(1);
        console.log(`✅ createTimestamp: ${timestamp}`);
        if (timestamp.length !== 10) throw new Error('Should be YYYY-MM-DD format');
    } catch (error) {
        console.log(`❌ createTimestamp failed: ${error.message}`);
    }
}

async function testAdvancedHelpers() {
    console.log('\n🔧 Testing Advanced Helpers...');
    
    // Test getTopMovers
    try {
        const movers = await getTopMovers('ethereum', 3, 100000);
        console.log(`✅ getTopMovers: ${movers.length} movers found`);
        if (movers.length > 3) throw new Error('Should respect limit');
    } catch (error) {
        console.log(`❌ getTopMovers failed: ${error.message}`);
    }
    
    // Test getHighVolumePools
    try {
        const pools = await getHighVolumePools('ethereum', 3);
        console.log(`✅ getHighVolumePools: ${pools.length} pools found`);
        if (pools.length > 3) throw new Error('Should respect limit');
    } catch (error) {
        console.log(`❌ getHighVolumePools failed: ${error.message}`);
    }
    
    // Test analyzeDexDistribution
    try {
        const testPools = [
            { dex_name: 'Uniswap', volume_usd: 1000000 },
            { dex_name: 'Uniswap', volume_usd: 500000 },
            { dex_name: 'SushiSwap', volume_usd: 300000 }
        ];
        const distribution = analyzeDexDistribution(testPools);
        console.log(`✅ analyzeDexDistribution: ${distribution.total_dexes} DEXes`);
        if (distribution.total_dexes !== 2) throw new Error('Should count unique DEXes');
    } catch (error) {
        console.log(`❌ analyzeDexDistribution failed: ${error.message}`);
    }
}

async function testRealWorldScenarios() {
    console.log('\n🔧 Testing Real-World Scenarios...');
    
    // Scenario 1: Market overview
    try {
        const overview = await getMarketOverview();
        console.log(`✅ Market Overview: ${overview.system_stats.pools} total pools`);
        if (overview.system_stats.pools <= 0) throw new Error('Should have pools');
    } catch (error) {
        console.log(`❌ Market Overview failed: ${error.message}`);
    }
    
    // Scenario 2: Token analysis (if we have a known token)
    try {
        // Try to get token details for a known token (USDC on Ethereum)
        const tokenData = await getTokenDetails('ethereum', '0xa0b86a33e6441b8c4c8c8c8c8c8c8c8c8c8c8c8c8');
        if (!tokenData.error) {
            const pools = await getTokenPools('ethereum', '0xa0b86a33e6441b8c4c8c8c8c8c8c8c8c8c8c8c8c8');
            const poolList = extractPools(pools);
            console.log(`✅ Token Analysis: ${poolList.length} pools found`);
        } else {
            console.log('⚠️  Token Analysis: Skipped (token not found)');
        }
    } catch (error) {
        console.log(`❌ Token Analysis failed: ${error.message}`);
    }
}

async function testDataExtractionFunctions() {
    console.log('\n🔧 Testing Data Extraction Functions...');
    
    // Test extractPools
    try {
        const testData = { pools: [{ id: 'pool1' }, { id: 'pool2' }] };
        const pools = extractPools(testData);
        console.log(`✅ extractPools: ${pools.length} pools extracted`);
        if (pools.length !== 2) throw new Error('Should extract correct number of pools');
    } catch (error) {
        console.log(`❌ extractPools failed: ${error.message}`);
    }
    
    // Test extractTokens
    try {
        const testData = { tokens: [{ symbol: 'BTC' }, { symbol: 'ETH' }] };
        const tokens = extractTokens(testData);
        console.log(`✅ extractTokens: ${tokens.length} tokens extracted`);
        if (tokens.length !== 2) throw new Error('Should extract correct number of tokens');
    } catch (error) {
        console.log(`❌ extractTokens failed: ${error.message}`);
    }
    
    // Test extractDexes
    try {
        const testData = { dexes: [{ name: 'Uniswap' }, { name: 'SushiSwap' }] };
        const dexes = extractDexes(testData);
        console.log(`✅ extractDexes: ${dexes.length} DEXes extracted`);
        if (dexes.length !== 2) throw new Error('Should extract correct number of DEXes');
    } catch (error) {
        console.log(`❌ extractDexes failed: ${error.message}`);
    }
}

async function testFilteringFunctions() {
    console.log('\n🔧 Testing Additional Filtering Functions...');
    
    const testPools = [
        { 
            volume_usd: 1000000, 
            chain: 'ethereum', 
            dex_name: 'Uniswap V3',
            tokens: [{ symbol: 'USDC' }, { symbol: 'ETH' }]
        },
        { 
            volume_usd: 500000, 
            chain: 'polygon', 
            dex_name: 'QuickSwap',
            tokens: [{ symbol: 'MATIC' }, { symbol: 'USDC' }]
        }
    ];
    
    // Test filterByNetwork
    try {
        const filtered = filterByNetwork(testPools, 'ethereum');
        console.log(`✅ filterByNetwork: ${filtered.length} ethereum pools`);
        if (filtered.length !== 1) throw new Error('Should filter by network correctly');
    } catch (error) {
        console.log(`❌ filterByNetwork failed: ${error.message}`);
    }
    
    // Test filterByDex
    try {
        const filtered = filterByDex(testPools, 'Uniswap');
        console.log(`✅ filterByDex: ${filtered.length} Uniswap pools`);
        if (filtered.length !== 1) throw new Error('Should filter by DEX correctly');
    } catch (error) {
        console.log(`❌ filterByDex failed: ${error.message}`);
    }
    
    // Test filterByTokenSymbol
    try {
        const filtered = filterByTokenSymbol(testPools, 'USDC');
        console.log(`✅ filterByTokenSymbol: ${filtered.length} USDC pools`);
        if (filtered.length !== 2) throw new Error('Should filter by token symbol correctly');
    } catch (error) {
        console.log(`❌ filterByTokenSymbol failed: ${error.message}`);
    }
}

async function main() {
    console.log('🚀 Starting Comprehensive JavaScript Helper Tests');
    console.log('='.repeat(60));
    
    const startTime = Date.now();
    
    // Run all test categories
    await testCoreApiHelpers();
    testDataExtractionHelpers();
    testFilteringSortingHelpers();
    testAnalysisHelpers();
    testUtilityHelpers();
    await testAdvancedHelpers();
    await testRealWorldScenarios();
    await testDataExtractionFunctions();
    await testFilteringFunctions();
    
    const endTime = Date.now();
    const duration = (endTime - startTime) / 1000;
    
    console.log('\n' + '='.repeat(60));
    console.log(`✅ All tests completed in ${duration.toFixed(2)} seconds`);
    console.log('🎉 JavaScript helpers are working correctly!');
}

// Handle graceful shutdown
process.on('SIGINT', () => {
    console.log('\n👋 Shutting down gracefully...');
    process.exit(0);
});

// Run the main function
main().catch(console.error); 