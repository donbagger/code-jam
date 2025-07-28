/**
 * Paprika Vibe-Code Helper Library
 * A comprehensive collection of 60+ micro-helpers for DexPaprika API integration
 * Covers ALL endpoints and data structures from the OpenAPI specification
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// For modern async requests (fallback to https if fetch not available)
let fetch;
try {
    fetch = require('node-fetch');
} catch (e) {
    // Fallback to built-in fetch in Node 18+
    if (typeof globalThis.fetch !== 'undefined') {
        fetch = globalThis.fetch;
    } else {
        // Use https module as fallback
        fetch = null;
    }
}

// Base API URL
const BASE_URL = 'https://api.dexpaprika.com';

// Cache configuration
const CACHE_DIR = '.cache';
const CACHE_DURATION = 300000; // 5 minutes in milliseconds

// Ensure cache directory exists
if (!fs.existsSync(CACHE_DIR)) {
    fs.mkdirSync(CACHE_DIR, { recursive: true });
}

// ============================================================================
// CORE API HELPERS (10 functions)
// ============================================================================

/**
 * Make API request with error handling and caching
 * @param {string} endpoint - API endpoint
 * @param {Object} params - Query parameters
 * @returns {Promise<Object>} API response
 */
async function apiRequest(endpoint, params = {}) {
    const cacheKey = crypto.createHash('md5').update(endpoint + JSON.stringify(params)).digest('hex');
    const cacheFile = path.join(CACHE_DIR, `${cacheKey}.json`);
    
    // Check cache
    if (fs.existsSync(cacheFile)) {
        const stats = fs.statSync(cacheFile);
        if (Date.now() - stats.mtime.getTime() < CACHE_DURATION) {
            try {
                const cached = JSON.parse(fs.readFileSync(cacheFile, 'utf8'));
                return cached;
            } catch (e) {
                // Cache corrupted, continue with API call
            }
        }
    }
    
    try {
        const url = new URL(endpoint, BASE_URL);
        Object.keys(params).forEach(key => url.searchParams.append(key, params[key]));
        
        const response = await new Promise((resolve, reject) => {
            https.get(url.toString(), (res) => {
                let data = '';
                res.on('data', chunk => data += chunk);
                res.on('end', () => {
                    try {
                        const jsonData = JSON.parse(data);
                        // Cache response
                        fs.writeFileSync(cacheFile, JSON.stringify(jsonData));
                        resolve(jsonData);
                    } catch (e) {
                        reject(new Error('Invalid JSON response'));
                    }
                });
            }).on('error', reject);
        });
        
        return response;
    } catch (error) {
        console.error(`API request failed: ${error.message}`);
        return { error: error.message };
    }
}

/**
 * Get all supported blockchain networks
 * @returns {Promise<Array>} List of networks
 */
async function getNetworks() {
    return await apiRequest('/networks');
}

/**
 * Get pools for a specific network
 * @param {string} network - Network ID
 * @param {Object} opts - Options (page, limit, sort, order_by)
 * @returns {Promise<Object>} Pools data
 */
async function getNetworkPools(network, opts = {}) {
    return await apiRequest(`/networks/${network}/pools`, opts);
}

/**
 * Get pools for a specific DEX on a network
 * @param {string} network - Network ID
 * @param {string} dex - DEX ID
 * @param {Object} opts - Options (page, limit, sort, order_by)
 * @returns {Promise<Object>} DEX pools data
 */
async function getDexPools(network, dex, opts = {}) {
    return await apiRequest(`/networks/${network}/dexes/${dex}/pools`, opts);
}

/**
 * Get DEXes available on a network
 * @param {string} network - Network ID
 * @param {Object} opts - Options (page, limit, sort, order_by)
 * @returns {Promise<Object>} DEXes data
 */
async function getNetworkDexes(network, opts = {}) {
    return await apiRequest(`/networks/${network}/dexes`, opts);
}

/**
 * Get detailed information about a specific pool
 * @param {string} network - Network ID
 * @param {string} poolAddress - Pool address
 * @param {boolean} inversed - Whether to invert price ratio
 * @returns {Promise<Object>} Pool details
 */
async function getPoolDetails(network, poolAddress, inversed = false) {
    return await apiRequest(`/networks/${network}/pools/${poolAddress}`, { inversed });
}

/**
 * Get OHLCV data for a pool
 * @param {string} network - Network ID
 * @param {string} poolAddress - Pool address
 * @param {string} start - Start time
 * @param {Object} opts - Options (end, limit, interval, inversed)
 * @returns {Promise<Array>} OHLCV data
 */
async function getPoolOHLCV(network, poolAddress, start, opts = {}) {
    return await apiRequest(`/networks/${network}/pools/${poolAddress}/ohlcv`, { start, ...opts });
}

/**
 * Get transactions for a specific pool
 * @param {string} network - Network ID
 * @param {string} poolAddress - Pool address
 * @param {Object} opts - Options (page, limit, cursor)
 * @returns {Promise<Object>} Transactions data
 */
async function getPoolTransactions(network, poolAddress, opts = {}) {
    return await apiRequest(`/networks/${network}/pools/${poolAddress}/transactions`, opts);
}

/**
 * Get detailed information about a specific token
 * @param {string} network - Network ID
 * @param {string} tokenAddress - Token address
 * @returns {Promise<Object>} Token details
 */
async function getTokenDetails(network, tokenAddress) {
    return await apiRequest(`/networks/${network}/tokens/${tokenAddress}`);
}

/**
 * Get pools containing a specific token
 * @param {string} network - Network ID
 * @param {string} tokenAddress - Token address
 * @param {Object} opts - Options (page, limit, sort, order_by, reorder, address)
 * @returns {Promise<Object>} Token pools data
 */
async function getTokenPools(network, tokenAddress, opts = {}) {
    return await apiRequest(`/networks/${network}/tokens/${tokenAddress}/pools`, opts);
}

/**
 * Search across tokens, pools, and DEXes
 * @param {string} query - Search query
 * @returns {Promise<Object>} Search results
 */
async function searchEntities(query) {
    return await apiRequest('/search', { query });
}

/**
 * Get high-level system statistics
 * @returns {Promise<Object>} System stats
 */
async function getSystemStats() {
    return await apiRequest('/stats');
}

// ============================================================================
// DATA EXTRACTION HELPERS (15 functions)
// ============================================================================

/**
 * Extract pools from API response
 * @param {Object} data - API response
 * @returns {Array} Pools array
 */
function extractPools(data) {
    return data?.pools || (Array.isArray(data) ? data : []);
}

/**
 * Extract tokens from API response
 * @param {Object} data - API response
 * @returns {Array} Tokens array
 */
function extractTokens(data) {
    return data?.tokens || (Array.isArray(data) ? data : []);
}

/**
 * Extract DEXes from API response
 * @param {Object} data - API response
 * @returns {Array} DEXes array
 */
function extractDexes(data) {
    return data?.dexes || (Array.isArray(data) ? data : []);
}

/**
 * Extract transactions from API response
 * @param {Object} data - API response
 * @returns {Array} Transactions array
 */
function extractTransactions(data) {
    return data?.transactions || (Array.isArray(data) ? data : []);
}

/**
 * Extract OHLCV data from API response
 * @param {Array} data - API response
 * @returns {Array} OHLCV array
 */
function extractOHLCV(data) {
    return Array.isArray(data) ? data : [];
}

/**
 * Extract pagination info from API response
 * @param {Object} data - API response
 * @returns {Object} Page info
 */
function extractPageInfo(data) {
    return data?.page_info || {};
}

/**
 * Extract token summary metrics
 * @param {Object} tokenData - Token data
 * @returns {Object} Token summary
 */
function extractTokenSummary(tokenData) {
    return tokenData?.summary || {};
}

/**
 * Extract time interval metrics from token data
 * @param {Object} tokenData - Token data
 * @param {string} timeframe - Timeframe (24h, 6h, 1h, etc.)
 * @returns {Object} Time metrics
 */
function extractTimeMetrics(tokenData, timeframe = '24h') {
    const summary = extractTokenSummary(tokenData);
    return summary[timeframe] || {};
}

/**
 * Extract key metrics from pool data
 * @param {Object} poolData - Pool data
 * @returns {Object} Pool metrics
 */
function extractPoolMetrics(poolData) {
    return {
        price_usd: poolData?.price_usd || 0,
        volume_usd: poolData?.volume_usd || 0,
        transactions: poolData?.transactions || 0,
        last_price_change_24h: poolData?.last_price_change_usd_24h || 0,
        fee: poolData?.fee || 0
    };
}

/**
 * Extract basic token information
 * @param {Object} tokenData - Token data
 * @returns {Object} Token info
 */
function extractTokenInfo(tokenData) {
    return {
        id: tokenData?.id || '',
        name: tokenData?.name || '',
        symbol: tokenData?.symbol || '',
        chain: tokenData?.chain || '',
        decimals: tokenData?.decimals || 0,
        price_usd: tokenData?.summary?.price_usd || 0
    };
}

/**
 * Extract key transaction information
 * @param {Object} txData - Transaction data
 * @returns {Object} Transaction info
 */
function extractTransactionInfo(txData) {
    return {
        id: txData?.id || '',
        pool_id: txData?.pool_id || '',
        token_0_symbol: txData?.token_0_symbol || '',
        token_1_symbol: txData?.token_1_symbol || '',
        amount_0: txData?.amount_0 || '',
        amount_1: txData?.amount_1 || '',
        price_0_usd: txData?.price_0_usd || 0,
        price_1_usd: txData?.price_1_usd || 0,
        created_at: txData?.created_at || ''
    };
}

/**
 * Extract key metrics from OHLCV data
 * @param {Array} ohlcvData - OHLCV data
 * @returns {Object} OHLCV metrics
 */
function extractOHLCVMetrics(ohlcvData) {
    if (!ohlcvData || ohlcvData.length === 0) return {};
    
    const latest = ohlcvData[ohlcvData.length - 1];
    return {
        open: latest?.open || 0,
        high: latest?.high || 0,
        low: latest?.low || 0,
        close: latest?.close || 0,
        volume: latest?.volume || 0,
        time_open: latest?.time_open || '',
        time_close: latest?.time_close || ''
    };
}

/**
 * Extract search results by category
 * @param {Object} searchData - Search data
 * @returns {Object} Search results by category
 */
function extractSearchResults(searchData) {
    return {
        tokens: searchData?.tokens || [],
        pools: searchData?.pools || [],
        dexes: searchData?.dexes || []
    };
}

/**
 * Extract system statistics
 * @param {Object} statsData - Stats data
 * @returns {Object} System stats
 */
function extractSystemStats(statsData) {
    return {
        chains: statsData?.chains || 0,
        factories: statsData?.factories || 0,
        pools: statsData?.pools || 0,
        tokens: statsData?.tokens || 0
    };
}

/**
 * Extract token information from pool data
 * @param {Object} poolData - Pool data
 * @returns {Array} Pool tokens
 */
function extractPoolTokens(poolData) {
    return poolData?.tokens || [];
}

// ============================================================================
// FILTERING & SORTING HELPERS (12 functions)
// ============================================================================

/**
 * Filter pools by minimum price change percentage
 * @param {Array} data - Data array
 * @param {number} minChange - Minimum change percentage
 * @returns {Array} Filtered data
 */
function filterByPriceChange(data, minChange) {
    return data.filter(item => Math.abs(item?.last_price_change_usd_24h || 0) >= minChange);
}

/**
 * Filter pools by minimum volume
 * @param {Array} data - Data array
 * @param {number} minVolume - Minimum volume
 * @returns {Array} Filtered data
 */
function filterByVolume(data, minVolume) {
    return data.filter(item => (item?.volume_usd || 0) >= minVolume);
}

/**
 * Filter data by network
 * @param {Array} data - Data array
 * @param {string} network - Network name
 * @returns {Array} Filtered data
 */
function filterByNetwork(data, network) {
    return data.filter(item => item?.chain?.toLowerCase() === network.toLowerCase());
}

/**
 * Filter pools by DEX name
 * @param {Array} data - Data array
 * @param {string} dexName - DEX name
 * @returns {Array} Filtered data
 */
function filterByDex(data, dexName) {
    return data.filter(item => item?.dex_name?.toLowerCase().includes(dexName.toLowerCase()));
}

/**
 * Filter pools by token symbol
 * @param {Array} data - Data array
 * @param {string} symbol - Token symbol
 * @returns {Array} Filtered data
 */
function filterByTokenSymbol(data, symbol) {
    const upperSymbol = symbol.toUpperCase();
    return data.filter(item => 
        item?.tokens?.some(token => token?.symbol?.toUpperCase() === upperSymbol)
    );
}

/**
 * Filter pools by token address
 * @param {Array} data - Data array
 * @param {string} address - Token address
 * @returns {Array} Filtered data
 */
function filterByTokenAddress(data, address) {
    return data.filter(item => 
        item?.tokens?.some(token => token?.id?.toLowerCase() === address.toLowerCase())
    );
}

/**
 * Get top N items by field
 * @param {Array} data - Data array
 * @param {string} field - Field to sort by
 * @param {number} n - Number of items
 * @returns {Array} Top N items
 */
function topN(data, field, n = 10) {
    return data
        .sort((a, b) => (b[field] || 0) - (a[field] || 0))
        .slice(0, n);
}

/**
 * Get bottom N items by field
 * @param {Array} data - Data array
 * @param {string} field - Field to sort by
 * @param {number} n - Number of items
 * @returns {Array} Bottom N items
 */
function bottomN(data, field, n = 10) {
    return data
        .sort((a, b) => (a[field] || 0) - (b[field] || 0))
        .slice(0, n);
}

/**
 * Sort data by field
 * @param {Array} data - Data array
 * @param {string} field - Field to sort by
 * @param {boolean} reverse - Reverse sort order
 * @returns {Array} Sorted data
 */
function sortByField(data, field, reverse = true) {
    return data.sort((a, b) => {
        const aVal = a[field] || 0;
        const bVal = b[field] || 0;
        return reverse ? bVal - aVal : aVal - bVal;
    });
}

/**
 * Filter transactions by recency
 * @param {Array} txData - Transaction data
 * @param {number} hours - Hours ago
 * @returns {Array} Filtered transactions
 */
function filterRecentTransactions(txData, hours = 24) {
    const cutoff = new Date(Date.now() - hours * 60 * 60 * 1000);
    return txData.filter(tx => new Date(tx?.created_at) > cutoff);
}

/**
 * Filter transactions by minimum USD value
 * @param {Array} txData - Transaction data
 * @param {number} minUsd - Minimum USD value
 * @returns {Array} Filtered transactions
 */
function filterLargeTransactions(txData, minUsd) {
    return txData.filter(tx => 
        Math.max(tx?.price_0_usd || 0, tx?.price_1_usd || 0) >= minUsd
    );
}

/**
 * Filter OHLCV data by time range
 * @param {Array} ohlcvData - OHLCV data
 * @param {string} startTime - Start time
 * @param {string} endTime - End time
 * @returns {Array} Filtered OHLCV data
 */
function filterByTimeframe(ohlcvData, startTime, endTime = null) {
    const startDt = new Date(startTime);
    const endDt = endTime ? new Date(endTime) : new Date();
    
    return ohlcvData.filter(candle => {
        const candleTime = new Date(candle?.time_open);
        return candleTime >= startDt && candleTime <= endDt;
    });
}

// ============================================================================
// ANALYSIS HELPERS (10 functions)
// ============================================================================

/**
 * Calculate percentage price change
 * @param {number} current - Current price
 * @param {number} previous - Previous price
 * @returns {number} Percentage change
 */
function calculatePriceChange(current, previous) {
    return previous !== 0 ? ((current - previous) / previous) * 100 : 0;
}

/**
 * Calculate volume-weighted average price
 * @param {Array} ohlcvData - OHLCV data
 * @returns {number} VWAP
 */
function calculateVolumeWeightedPrice(ohlcvData) {
    if (!ohlcvData || ohlcvData.length === 0) return 0;
    
    const totalVolume = ohlcvData.reduce((sum, candle) => sum + (candle?.volume || 0), 0);
    if (totalVolume === 0) return 0;
    
    const vwap = ohlcvData.reduce((sum, candle) => 
        sum + (candle?.close || 0) * (candle?.volume || 0), 0
    );
    
    return vwap / totalVolume;
}

/**
 * Calculate price volatility from OHLCV data
 * @param {Array} ohlcvData - OHLCV data
 * @returns {number} Volatility percentage
 */
function calculateVolatility(ohlcvData) {
    if (!ohlcvData || ohlcvData.length < 2) return 0;
    
    const returns = [];
    for (let i = 1; i < ohlcvData.length; i++) {
        const prevClose = ohlcvData[i-1]?.close || 0;
        const currClose = ohlcvData[i]?.close || 0;
        if (prevClose > 0) {
            returns.push((currClose - prevClose) / prevClose);
        }
    }
    
    if (returns.length === 0) return 0;
    
    const meanReturn = returns.reduce((sum, r) => sum + r, 0) / returns.length;
    const variance = returns.reduce((sum, r) => sum + Math.pow(r - meanReturn, 2), 0) / returns.length;
    
    return Math.sqrt(variance) * 100; // Return as percentage
}

/**
 * Analyze pool activity metrics
 * @param {Object} poolData - Pool data
 * @returns {Object} Activity metrics
 */
function analyzePoolActivity(poolData) {
    const metrics = {};
    const timeframes = ['24h', '6h', '1h', '30m', '15m', '5m'];
    
    timeframes.forEach(timeframe => {
        if (poolData[timeframe]) {
            metrics[timeframe] = {
                volume_usd: poolData[timeframe]?.volume_usd || 0,
                txns: poolData[timeframe]?.txns || 0,
                price_change: poolData[timeframe]?.last_price_usd_change || 0
            };
        }
    });
    
    return metrics;
}

/**
 * Analyze token performance across timeframes
 * @param {Object} tokenData - Token data
 * @returns {Object} Performance metrics
 */
function analyzeTokenPerformance(tokenData) {
    const summary = extractTokenSummary(tokenData);
    if (!summary || Object.keys(summary).length === 0) return {};
    
    const performance = {};
    const timeframes = ['24h', '6h', '1h', '30m', '15m', '5m'];
    
    timeframes.forEach(timeframe => {
        if (summary[timeframe]) {
            const metrics = summary[timeframe];
            performance[timeframe] = {
                volume_usd: metrics?.volume_usd || 0,
                price_change: metrics?.last_price_usd_change || 0,
                txns: metrics?.txns || 0,
                buy_ratio: metrics?.volume_usd > 0 ? (metrics?.buy_usd || 0) / metrics.volume_usd : 0
            };
        }
    });
    
    return performance;
}

/**
 * Detect anomalies using z-score method
 * @param {Array} data - Data array
 * @param {string} field - Field to analyze
 * @param {number} threshold - Z-score threshold
 * @returns {Array} Anomalies
 */
function detectAnomalies(data, field, threshold = 2.0) {
    if (!data || data.length < 3) return [];
    
    const values = data.map(item => item[field] || 0);
    const mean = values.reduce((sum, val) => sum + val, 0) / values.length;
    const variance = values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
    const stdDev = Math.sqrt(variance);
    
    if (stdDev === 0) return [];
    
    return data.filter(item => {
        const zScore = Math.abs((item[field] || 0) - mean) / stdDev;
        return zScore > threshold;
    });
}

/**
 * Calculate correlation coefficient between two datasets
 * @param {Array} data1 - First dataset
 * @param {Array} data2 - Second dataset
 * @returns {number} Correlation coefficient
 */
function calculateCorrelation(data1, data2) {
    if (!data1 || !data2 || data1.length !== data2.length || data1.length < 2) return 0;
    
    const mean1 = data1.reduce((sum, val) => sum + val, 0) / data1.length;
    const mean2 = data2.reduce((sum, val) => sum + val, 0) / data2.length;
    
    const numerator = data1.reduce((sum, val, i) => 
        sum + (val - mean1) * (data2[i] - mean2), 0
    );
    
    const denominator1 = data1.reduce((sum, val) => sum + Math.pow(val - mean1, 2), 0);
    const denominator2 = data2.reduce((sum, val) => sum + Math.pow(val - mean2, 2), 0);
    
    if (denominator1 === 0 || denominator2 === 0) return 0;
    
    return numerator / Math.sqrt(denominator1 * denominator2);
}

/**
 * Analyze liquidity distribution across pools
 * @param {Array} pools - Pools array
 * @returns {Object} Liquidity analysis
 */
function analyzeLiquidityDistribution(pools) {
    if (!pools || pools.length === 0) return {};
    
    const volumes = pools.map(pool => pool?.volume_usd || 0);
    const totalVolume = volumes.reduce((sum, vol) => sum + vol, 0);
    
    if (totalVolume === 0) return {};
    
    const sortedVolumes = [...volumes].sort((a, b) => b - a);
    const top10Pct = sortedVolumes.slice(0, Math.max(1, Math.floor(sortedVolumes.length / 10)))
        .reduce((sum, vol) => sum + vol, 0) / totalVolume;
    const top50Pct = sortedVolumes.slice(0, Math.max(1, Math.floor(sortedVolumes.length / 2)))
        .reduce((sum, vol) => sum + vol, 0) / totalVolume;
    
    return {
        total_volume: totalVolume,
        avg_volume: totalVolume / pools.length,
        top_10_percent_concentration: top10Pct,
        top_50_percent_concentration: top50Pct,
        gini_coefficient: calculateGiniCoefficient(volumes)
    };
}

/**
 * Calculate Gini coefficient for inequality measurement
 * @param {Array} values - Values array
 * @returns {number} Gini coefficient
 */
function calculateGiniCoefficient(values) {
    if (!values || values.length < 2) return 0;
    
    const sortedValues = [...values].sort((a, b) => a - b);
    const n = sortedValues.length;
    let cumsum = 0;
    
    for (let i = 0; i < n; i++) {
        cumsum += (i + 1) * sortedValues[i];
    }
    
    return (2 * cumsum) / (n * sortedValues.reduce((sum, val) => sum + val, 0)) - (n + 1) / n;
}

/**
 * Analyze transaction patterns
 * @param {Array} txData - Transaction data
 * @returns {Object} Transaction patterns
 */
function analyzeTransactionPatterns(txData) {
    if (!txData || txData.length === 0) return {};
    
    const buyTxs = txData.filter(tx => tx?.token_0_symbol && tx?.token_1_symbol);
    const sellTxs = txData.filter(tx => tx?.token_0_symbol && tx?.token_1_symbol);
    
    const totalBuyVolume = buyTxs.reduce((sum, tx) => sum + (tx?.price_0_usd || 0), 0);
    const totalSellVolume = sellTxs.reduce((sum, tx) => sum + (tx?.price_1_usd || 0), 0);
    
    return {
        total_transactions: txData.length,
        buy_transactions: buyTxs.length,
        sell_transactions: sellTxs.length,
        buy_volume_usd: totalBuyVolume,
        sell_volume_usd: totalSellVolume,
        buy_sell_ratio: totalSellVolume > 0 ? totalBuyVolume / totalSellVolume : 0
    };
}

// ============================================================================
// UTILITY HELPERS (8 functions)
// ============================================================================

/**
 * Format number with appropriate suffixes
 * @param {number} num - Number to format
 * @param {number} decimals - Decimal places
 * @returns {string} Formatted number
 */
function formatNumber(num, decimals = 2) {
    if (num >= 1e9) return `${(num / 1e9).toFixed(decimals)}B`;
    if (num >= 1e6) return `${(num / 1e6).toFixed(decimals)}M`;
    if (num >= 1e3) return `${(num / 1e3).toFixed(decimals)}K`;
    return num.toFixed(decimals);
}

/**
 * Format percentage with sign
 * @param {number} num - Number to format
 * @param {number} decimals - Decimal places
 * @returns {string} Formatted percentage
 */
function formatPercentage(num, decimals = 2) {
    const sign = num >= 0 ? '+' : '';
    return `${sign}${num.toFixed(decimals)}%`;
}

/**
 * Print data as a table
 * @param {Array} data - Data array
 * @param {Array} columns - Column names
 * @param {string} title - Table title
 */
function printTable(data, columns, title = 'Data Table') {
    console.log(`\n${title}`);
    console.log('='.repeat(50));
    
    // Print headers
    console.log(columns.map(col => col.padEnd(20)).join(' | '));
    console.log('-'.repeat(50));
    
    // Print data
    data.forEach(item => {
        const row = columns.map(col => String(item[col] || '').padEnd(20)).join(' | ');
        console.log(row);
    });
    
    console.log('='.repeat(50));
}

/**
 * Save data to CSV file
 * @param {Array} data - Data array
 * @param {string} filename - Filename
 * @param {Array} columns - Column names
 */
function saveToCSV(data, filename, columns = null) {
    if (!data || data.length === 0) return;
    
    const cols = columns || Object.keys(data[0]);
    const csvContent = [
        cols.join(','),
        ...data.map(item => cols.map(col => `"${item[col] || ''}"`).join(','))
    ].join('\n');
    
    fs.writeFileSync(filename, csvContent);
    console.log(`Data saved to ${filename}`);
}

/**
 * Load environment variable with fallback
 * @param {string} key - Environment variable key
 * @param {string} defaultValue - Default value
 * @returns {string} Environment variable value
 */
function loadEnv(key, defaultValue = '') {
    return process.env[key] || defaultValue;
}

/**
 * Validate if network is supported
 * @param {string} network - Network name
 * @returns {boolean} Is valid network
 */
async function validateNetwork(network) {
    try {
        const networks = await getNetworks();
        return networks.some(n => n.id === network);
    } catch {
        return false;
    }
}

/**
 * Basic token address validation
 * @param {string} address - Token address
 * @returns {boolean} Is valid address
 */
function validateTokenAddress(address) {
    return address && address.length >= 10 && /^[a-zA-Z0-9]+$/.test(address);
}

/**
 * Create ISO timestamp for API requests
 * @param {number} daysAgo - Days ago
 * @returns {string} ISO timestamp
 */
function createTimestamp(daysAgo = 0) {
    const date = new Date();
    date.setDate(date.getDate() - daysAgo);
    return date.toISOString().split('T')[0];
}

// ============================================================================
// ADVANCED HELPERS (5 functions)
// ============================================================================

/**
 * Get top movers with volume filter
 * @param {string} network - Network ID
 * @param {number} limit - Number of results
 * @param {number} minVolume - Minimum volume
 * @returns {Promise<Array>} Top movers
 */
async function getTopMovers(network, limit = 10, minVolume = 1000000) {
    const pools = await getNetworkPools(network, { limit: 100 });
    const poolList = extractPools(pools);
    const filtered = filterByVolume(poolList, minVolume);
    return topN(filtered, 'last_price_change_usd_24h', limit);
}

/**
 * Get highest volume pools on a network
 * @param {string} network - Network ID
 * @param {number} limit - Number of results
 * @returns {Promise<Array>} High volume pools
 */
async function getHighVolumePools(network, limit = 10) {
    const pools = await getNetworkPools(network, { limit: 100 });
    const poolList = extractPools(pools);
    return topN(poolList, 'volume_usd', limit);
}

/**
 * Get comprehensive liquidity analysis for a token
 * @param {string} tokenAddress - Token address
 * @param {string} network - Network ID
 * @returns {Promise<Object>} Liquidity analysis
 */
async function getTokenLiquidityAnalysis(tokenAddress, network = 'ethereum') {
    const tokenData = await getTokenDetails(network, tokenAddress);
    const tokenPools = await getTokenPools(network, tokenAddress);
    const poolsList = extractPools(tokenPools);
    
    return {
        token_info: extractTokenInfo(tokenData),
        total_liquidity: poolsList.reduce((sum, pool) => sum + (pool?.volume_usd || 0), 0),
        pool_count: poolsList.length,
        top_pools: topN(poolsList, 'volume_usd', 5),
        dex_distribution: analyzeDexDistribution(poolsList)
    };
}

/**
 * Analyze distribution of pools across DEXes
 * @param {Array} pools - Pools array
 * @returns {Object} DEX distribution
 */
function analyzeDexDistribution(pools) {
    const dexCounts = {};
    const dexVolumes = {};
    
    pools.forEach(pool => {
        const dexName = pool?.dex_name || 'Unknown';
        const volume = pool?.volume_usd || 0;
        
        dexCounts[dexName] = (dexCounts[dexName] || 0) + 1;
        dexVolumes[dexName] = (dexVolumes[dexName] || 0) + volume;
    });
    
    return {
        counts: dexCounts,
        volumes: dexVolumes,
        total_dexes: Object.keys(dexCounts).length
    };
}

/**
 * Get comprehensive market overview
 * @returns {Promise<Object>} Market overview
 */
async function getMarketOverview() {
    const stats = await getSystemStats();
    const statsData = extractSystemStats(stats);
    
    const networks = await getNetworks();
    const networkOverview = {};
    
    for (const network of networks.slice(0, 5)) {
        try {
            const pools = await getNetworkPools(network.id, { limit: 10 });
            const poolList = extractPools(pools);
            const totalVolume = poolList.reduce((sum, pool) => sum + (pool?.volume_usd || 0), 0);
            
            networkOverview[network.id] = {
                display_name: network.display_name,
                total_volume: totalVolume,
                pool_count: poolList.length
            };
        } catch (error) {
            // Skip failed networks
        }
    }
    
    return {
        system_stats: statsData,
        network_overview: networkOverview,
        timestamp: new Date().toISOString()
    };
}

// ============================================================================
// ASYNC BATCH PROCESSING HELPERS (5 functions)
// ============================================================================

/**
 * Async API request with promise-based approach
 * @param {string} endpoint - API endpoint
 * @param {Object} params - Query parameters
 * @returns {Promise<Object>} API response
 */
async function asyncApiRequest(endpoint, params = {}) {
    const cacheKey = crypto.createHash('md5').update(endpoint + JSON.stringify(params)).digest('hex');
    const cacheFile = path.join(CACHE_DIR, `${cacheKey}.json`);
    
    // Check cache
    if (fs.existsSync(cacheFile)) {
        const stats = fs.statSync(cacheFile);
        if (Date.now() - stats.mtime.getTime() < CACHE_DURATION) {
            try {
                const cached = JSON.parse(fs.readFileSync(cacheFile, 'utf8'));
                return cached;
            } catch (e) {
                // Cache corrupted, continue with API call
            }
        }
    }
    
    try {
        const url = new URL(endpoint, BASE_URL);
        Object.keys(params).forEach(key => url.searchParams.append(key, params[key]));
        
        let jsonData;
        
        if (fetch) {
            // Use fetch if available
            const response = await fetch(url.toString());
            jsonData = await response.json();
        } else {
            // Fallback to https module with Promise wrapper
            jsonData = await new Promise((resolve, reject) => {
                https.get(url.toString(), (res) => {
                    let data = '';
                    res.on('data', chunk => data += chunk);
                    res.on('end', () => {
                        try {
                            resolve(JSON.parse(data));
                        } catch (e) {
                            reject(new Error('Invalid JSON response'));
                        }
                    });
                }).on('error', reject);
            });
        }
        
        // Cache response
        fs.writeFileSync(cacheFile, JSON.stringify(jsonData));
        
        return jsonData;
    } catch (error) {
        console.error(`Async API request failed: ${error.message}`);
        return { error: error.message };
    }
}

/**
 * Get pools from multiple networks concurrently
 * @param {Array<string>} networks - Array of network IDs
 * @param {number} limit - Number of pools per network
 * @returns {Promise<Object>} Object with network IDs as keys and pool data as values
 */
async function asyncGetMultiplePools(networks, limit = 10) {
    const promises = networks.map(network => 
        asyncApiRequest(`/networks/${network}/pools`, { limit })
    );
    
    try {
        const results = await Promise.allSettled(promises);
        const networkResults = {};
        
        networks.forEach((network, index) => {
            const result = results[index];
            if (result.status === 'fulfilled') {
                networkResults[network] = result.value;
            } else {
                networkResults[network] = { error: result.reason };
            }
        });
        
        return networkResults;
    } catch (error) {
        console.error(`Async multiple pools request failed: ${error.message}`);
        return { error: error.message };
    }
}

/**
 * Get data for multiple tokens concurrently
 * @param {Array<string>} tokenAddresses - Array of token addresses
 * @param {string} network - Network ID (default: 'ethereum')
 * @returns {Promise<Array>} Array of token data
 */
async function asyncGetTokenDataBatch(tokenAddresses, network = 'ethereum') {
    const promises = tokenAddresses.map(address => 
        asyncApiRequest(`/networks/${network}/tokens/${address}`)
    );
    
    try {
        const results = await Promise.allSettled(promises);
        return results.map((result, index) => {
            if (result.status === 'fulfilled') {
                return result.value;
            } else {
                return { 
                    error: result.reason, 
                    token_address: tokenAddresses[index],
                    network 
                };
            }
        });
    } catch (error) {
        console.error(`Async token batch request failed: ${error.message}`);
        return [{ error: error.message }];
    }
}

/**
 * Monitor pool prices asynchronously with interval checking
 * @param {Array<string>} poolAddresses - Array of pool addresses to monitor
 * @param {string} network - Network ID (default: 'ethereum')
 * @param {number} interval - Polling interval in seconds (default: 60)
 * @param {Function} callback - Callback function to handle price updates
 * @returns {Promise<Function>} Stop function to cancel monitoring
 */
async function asyncMonitorPrices(poolAddresses, network = 'ethereum', interval = 60, callback = null) {
    let isMonitoring = true;
    
    const monitor = async () => {
        while (isMonitoring) {
            try {
                const promises = poolAddresses.map(address => 
                    asyncApiRequest(`/networks/${network}/pools/${address}`)
                );
                
                const results = await Promise.allSettled(promises);
                const priceUpdates = {};
                
                results.forEach((result, index) => {
                    const address = poolAddresses[index];
                    if (result.status === 'fulfilled' && result.value.price_usd) {
                        priceUpdates[address] = {
                            price_usd: result.value.price_usd,
                            last_price_change_usd_24h: result.value.last_price_change_usd_24h,
                            volume_usd: result.value['24h']?.volume_usd || 0,
                            timestamp: new Date().toISOString()
                        };
                        
                        if (callback) {
                            callback(address, priceUpdates[address]);
                        } else {
                            console.log(`${address}: $${result.value.price_usd.toFixed(6)} (24h: ${result.value.last_price_change_usd_24h?.toFixed(2)}%)`);
                        }
                    }
                });
                
                // Wait for the specified interval
                await new Promise(resolve => setTimeout(resolve, interval * 1000));
                
            } catch (error) {
                console.error(`Price monitoring error: ${error.message}`);
                await new Promise(resolve => setTimeout(resolve, interval * 1000));
            }
        }
    };
    
    // Start monitoring
    monitor();
    
    // Return stop function
    return () => {
        isMonitoring = false;
    };
}

/**
 * Search for multiple queries concurrently
 * @param {Array<string>} queries - Array of search queries
 * @returns {Promise<Array>} Array of search results
 */
async function asyncBatchSearch(queries) {
    const promises = queries.map(query => 
        asyncApiRequest('/search', { query })
    );
    
    try {
        const results = await Promise.allSettled(promises);
        return results.map((result, index) => {
            if (result.status === 'fulfilled') {
                return {
                    query: queries[index],
                    ...result.value
                };
            } else {
                return { 
                    query: queries[index],
                    error: result.reason 
                };
            }
        });
    } catch (error) {
        console.error(`Async batch search failed: ${error.message}`);
        return [{ error: error.message }];
    }
}

// ============================================================================
// EXPORT ALL FUNCTIONS
// ============================================================================

module.exports = {
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
    getMarketOverview,
    
    // Async Batch Processing Helpers
    asyncApiRequest,
    asyncGetMultiplePools,
    asyncGetTokenDataBatch,
    asyncMonitorPrices,
    asyncBatchSearch
}; 