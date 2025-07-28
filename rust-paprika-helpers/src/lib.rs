//! # Paprika Helpers for Rust 🦀
//!
//! A comprehensive Rust library providing **67 helper functions** for DexPaprika API integration.
//! This library covers all endpoints and data structures from the OpenAPI specification with
//! **100% parity** with Python, JavaScript, and Go versions.
//!
//! ## Features
//!
//! - **Complete API Coverage**: All 67 functions covering every DexPaprika endpoint
//! - **Type Safety**: Full Rust type system with compile-time guarantees
//! - **Async Support**: Native async/await with Tokio runtime
//! - **Error Handling**: Comprehensive Result types with detailed error information
//! - **Performance**: Zero-cost abstractions and efficient memory usage
//! - **Production Ready**: Robust error handling, timeouts, and proper resource management
//!
//! ## Quick Example
//!
//! ```rust
//! use paprika_helpers::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let networks = get_networks().await?;
//!     println!("Found {} networks", networks.len());
//!     Ok(())
//! }
//! ```

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

#[path = "../Docs/internal/types.rs"]
pub mod types;
#[path = "../Docs/internal/error.rs"]
pub mod error;

pub use types::*;
pub use error::*;

// ============================================================================
// IMPORTS
// ============================================================================

use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use url::Url;
use dashmap::DashMap;
use chrono::{DateTime, Utc, Timelike};
use regex::Regex;
use std::sync::OnceLock;

// ============================================================================
// CONSTANTS & GLOBALS
// ============================================================================

/// Base URL for the DexPaprika API
pub const BASE_URL: &str = "https://api.dexpaprika.com";

/// Default timeout for API requests
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// Cache duration for API responses
pub const CACHE_DURATION: Duration = Duration::from_secs(300); // 5 minutes

/// Global HTTP client
static CLIENT: OnceLock<Client> = OnceLock::new();

/// Global cache for API responses
static CACHE: OnceLock<DashMap<String, CacheEntry>> = OnceLock::new();

/// Initialize the global HTTP client
fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .expect("Failed to create HTTP client")
    })
}

/// Initialize the global cache
fn get_cache() -> &'static DashMap<String, CacheEntry> {
    CACHE.get_or_init(|| DashMap::new())
}

// ============================================================================
// PRIVATE HELPER FUNCTIONS
// ============================================================================

/// Create a cache key from endpoint and parameters
fn create_cache_key(endpoint: &str, params: &ApiParams) -> String {
    let params_string = serde_json::to_string(&params.to_query_params()).unwrap_or_default();
    format!("{}-{:x}", endpoint, md5::compute(params_string.as_bytes()))
}

// ============================================================================
// CORE API HELPERS (12 functions)
// ============================================================================

/// Makes an HTTP request to the DexPaprika API with caching and error handling
pub async fn api_request(endpoint: &str, params: Option<ApiParams>) -> Result<Value> {
    let params = params.unwrap_or_default();
    let cache_key = create_cache_key(endpoint, &params);
    
    // Check cache first
    let cache = get_cache();
    if let Some(entry) = cache.get(&cache_key) {
        if Utc::now().signed_duration_since(entry.timestamp).num_seconds() < CACHE_DURATION.as_secs() as i64 {
            return Ok(entry.data.clone());
        }
    }
    
    // Build URL
    let mut url = Url::parse(&format!("{}{}", BASE_URL, endpoint))?;
    
    // Add query parameters
    for (key, value) in params.to_query_params() {
        url.query_pairs_mut().append_pair(key, &value);
    }
    
    // Make request with timeout
    let client = get_client();
    let response = timeout(DEFAULT_TIMEOUT, client.get(url).send()).await??;
    
    // Check for API errors
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        if let Ok(api_error) = serde_json::from_str::<APIError>(&error_text) {
            return Err(PaprikaError::ApiError(api_error.error));
        }
        return Err(PaprikaError::HttpError(status.to_string()));
    }
    
    // Parse JSON response
    let data: Value = response.json().await?;
    
    // Store in cache
    cache.insert(cache_key, CacheEntry {
        data: data.clone(),
        timestamp: Utc::now(),
    });
    
    Ok(data)
}

/// Retrieves all supported blockchain networks
pub async fn get_networks() -> Result<Vec<Network>> {
    let data = api_request("/networks", None).await?;
    let networks: Vec<Network> = serde_json::from_value(data)?;
    Ok(networks)
}

/// Retrieves pools for a specific network
pub async fn get_network_pools(network: &str, params: Option<ApiParams>) -> Result<PoolsResponse> {
    let endpoint = format!("/networks/{}/pools", network);
    let data = api_request(&endpoint, params).await?;
    let response: PoolsResponse = serde_json::from_value(data)?;
    Ok(response)
}

/// Retrieves pools for a specific DEX on a network
pub async fn get_dex_pools(network: &str, dex: &str, params: Option<ApiParams>) -> Result<PoolsResponse> {
    let endpoint = format!("/networks/{}/dexes/{}/pools", network, dex);
    let data = api_request(&endpoint, params).await?;
    let response: PoolsResponse = serde_json::from_value(data)?;
    Ok(response)
}

/// Retrieves DEXes available on a network
pub async fn get_network_dexes(network: &str, params: Option<ApiParams>) -> Result<DexesResponse> {
    let endpoint = format!("/networks/{}/dexes", network);
    let data = api_request(&endpoint, params).await?;
    let response: DexesResponse = serde_json::from_value(data)?;
    Ok(response)
}

/// Retrieves detailed information about a specific pool
pub async fn get_pool_details(network: &str, pool_address: &str, inversed: Option<bool>) -> Result<Pool> {
    let endpoint = format!("/networks/{}/pools/{}", network, pool_address);
    let mut params = ApiParams::new();
    if let Some(inv) = inversed {
        params.inversed = Some(inv);
    }
    let data = api_request(&endpoint, Some(params)).await?;
    let pool: Pool = serde_json::from_value(data)?;
    Ok(pool)
}

/// Retrieves OHLCV data for a pool
pub async fn get_pool_ohlcv(network: &str, pool_address: &str, start: &str, params: Option<ApiParams>) -> Result<Vec<OHLCVRecord>> {
    let endpoint = format!("/networks/{}/pools/{}/ohlcv", network, pool_address);
    let mut params = params.unwrap_or_default();
    params.start = Some(start.to_string());
    let data = api_request(&endpoint, Some(params)).await?;
    let records: Vec<OHLCVRecord> = serde_json::from_value(data)?;
    Ok(records)
}

/// Retrieves transactions for a specific pool
pub async fn get_pool_transactions(network: &str, pool_address: &str, params: Option<ApiParams>) -> Result<TransactionsResponse> {
    let endpoint = format!("/networks/{}/pools/{}/transactions", network, pool_address);
    let data = api_request(&endpoint, params).await?;
    let response: TransactionsResponse = serde_json::from_value(data)?;
    Ok(response)
}

/// Retrieves detailed information about a specific token
pub async fn get_token_details(network: &str, token_address: &str) -> Result<Token> {
    let endpoint = format!("/networks/{}/tokens/{}", network, token_address);
    let data = api_request(&endpoint, None).await?;
    let token: Token = serde_json::from_value(data)?;
    Ok(token)
}

/// Retrieves pools containing a specific token
pub async fn get_token_pools(network: &str, token_address: &str, params: Option<ApiParams>) -> Result<TokenPoolsResponse> {
    let endpoint = format!("/networks/{}/tokens/{}/pools", network, token_address);
    let data = api_request(&endpoint, params).await?;
    let response: TokenPoolsResponse = serde_json::from_value(data)?;
    Ok(response)
}

/// Searches across tokens, pools, and DEXes
pub async fn search_entities(query: &str) -> Result<SearchResponse> {
    // Use direct endpoint with manual query string construction
    let endpoint = format!("/search?query={}", query.replace(" ", "%20"));
    let data = api_request(&endpoint, None).await?;
    let response: SearchResponse = serde_json::from_value(data)?;
    Ok(response)
}

/// Retrieves high-level system statistics
pub async fn get_system_stats() -> Result<SystemStats> {
    let data = api_request("/stats", None).await?;
    let stats: SystemStats = serde_json::from_value(data)?;
    Ok(stats)
}

// ============================================================================
// DATA EXTRACTION HELPERS (15 functions)
// ============================================================================

/// Extracts pools from API response
pub fn extract_pools(data: &Value) -> Vec<Pool> {
    // Try to parse as PoolsResponse first
    if let Ok(pools_resp) = serde_json::from_value::<PoolsResponse>(data.clone()) {
        return pools_resp.pools;
    }
    
    // Try to parse as array of pools
    if let Ok(pools) = serde_json::from_value::<Vec<Pool>>(data.clone()) {
        return pools;
    }
    
    Vec::new()
}

/// Extracts tokens from API response
pub fn extract_tokens(data: &Value) -> Vec<Token> {
    // Try search response first
    if let Ok(search_resp) = serde_json::from_value::<SearchResponse>(data.clone()) {
        return search_resp.tokens;
    }
    
    // Try direct array
    if let Ok(tokens) = serde_json::from_value::<Vec<Token>>(data.clone()) {
        return tokens;
    }
    
    Vec::new()
}

/// Extracts DEXes from API response
pub fn extract_dexes(data: &Value) -> Vec<Dex> {
    // Try DexesResponse first
    if let Ok(dexes_resp) = serde_json::from_value::<DexesResponse>(data.clone()) {
        return dexes_resp.dexes;
    }
    
    // Try search response
    if let Ok(search_resp) = serde_json::from_value::<SearchResponse>(data.clone()) {
        return search_resp.dexes;
    }
    
    // Try direct array
    if let Ok(dexes) = serde_json::from_value::<Vec<Dex>>(data.clone()) {
        return dexes;
    }
    
    Vec::new()
}

/// Extracts transactions from API response
pub fn extract_transactions(data: &Value) -> Vec<Transaction> {
    if let Ok(tx_resp) = serde_json::from_value::<TransactionsResponse>(data.clone()) {
        return tx_resp.transactions;
    }
    
    if let Ok(txs) = serde_json::from_value::<Vec<Transaction>>(data.clone()) {
        return txs;
    }
    
    Vec::new()
}

/// Extracts OHLCV data from API response
pub fn extract_ohlcv(data: &Value) -> Vec<OHLCVRecord> {
    if let Ok(records) = serde_json::from_value::<Vec<OHLCVRecord>>(data.clone()) {
        return records;
    }
    
    Vec::new()
}

/// Extracts pagination info from API response
pub fn extract_page_info(data: &Value) -> Option<PageInfo> {
    // Try different response types
    if let Ok(pools_resp) = serde_json::from_value::<PoolsResponse>(data.clone()) {
        return Some(pools_resp.page_info);
    }
    
    if let Ok(dexes_resp) = serde_json::from_value::<DexesResponse>(data.clone()) {
        return Some(dexes_resp.page_info);
    }
    
    if let Ok(token_pools_resp) = serde_json::from_value::<TokenPoolsResponse>(data.clone()) {
        return Some(token_pools_resp.page_info);
    }
    
    None
}

/// Extracts token summary from token data
pub fn extract_token_summary(data: &Value) -> Option<TokenSummary> {
    if let Ok(token) = serde_json::from_value::<Token>(data.clone()) {
        return token.summary;
    }
    
    None
}

/// Extracts time interval metrics from data
pub fn extract_time_metrics(data: &Value, timeframe: &str) -> Option<TimeIntervalMetrics> {
    // Try pool data
    if let Ok(pool) = serde_json::from_value::<Pool>(data.clone()) {
        return match timeframe {
            "24h" => pool.h24,
            "6h" => pool.h6,
            "1h" => pool.h1,
            "30m" => pool.m30,
            "15m" => pool.m15,
            "5m" => pool.m5,
            _ => None,
        };
    }
    
    // Try token summary
    if let Ok(summary) = serde_json::from_value::<TokenSummary>(data.clone()) {
        return match timeframe {
            "24h" => summary.h24,
            "6h" => summary.h6,
            "1h" => summary.h1,
            "30m" => summary.m30,
            "15m" => summary.m15,
            "5m" => summary.m5,
            "1m" => summary.m1,
            _ => None,
        };
    }
    
    None
}

/// Extracts pool metrics from pool data
pub fn extract_pool_metrics(data: &Value) -> HashMap<String, Value> {
    let mut metrics = HashMap::new();
    
    if let Ok(pool) = serde_json::from_value::<Pool>(data.clone()) {
        metrics.insert("volume_usd".to_string(), json!(pool.volume_usd));
        metrics.insert("price_usd".to_string(), json!(pool.price_usd));
        metrics.insert("transactions".to_string(), json!(pool.transactions));
        metrics.insert("dex_name".to_string(), json!(pool.dex_name));
        metrics.insert("chain".to_string(), json!(pool.chain));
        metrics.insert("last_price_change_24h".to_string(), json!(pool.last_price_change_usd_24h));
        metrics.insert("created_at".to_string(), json!(pool.created_at));
        metrics.insert("token_count".to_string(), json!(pool.tokens.len()));
    }
    
    metrics
}

/// Extracts basic token information
pub fn extract_token_info(data: &Value) -> HashMap<String, Value> {
    let mut info = HashMap::new();
    
    if let Ok(token) = serde_json::from_value::<Token>(data.clone()) {
        info.insert("id".to_string(), json!(token.id));
        info.insert("name".to_string(), json!(token.name));
        info.insert("symbol".to_string(), json!(token.symbol));
        info.insert("chain".to_string(), json!(token.chain));
        info.insert("decimals".to_string(), json!(token.decimals));
        info.insert("total_supply".to_string(), json!(token.total_supply));
        info.insert("website".to_string(), json!(token.website));
        info.insert("fdv".to_string(), json!(token.fdv));
        
        if let Some(summary) = token.summary {
            info.insert("price_usd".to_string(), json!(summary.price_usd));
            info.insert("liquidity_usd".to_string(), json!(summary.liquidity_usd));
        }
    }
    
    info
}

/// Extracts transaction information
pub fn extract_transaction_info(data: &Value) -> HashMap<String, Value> {
    let mut info = HashMap::new();
    
    if let Ok(tx) = serde_json::from_value::<Transaction>(data.clone()) {
        info.insert("id".to_string(), json!(tx.id));
        info.insert("pool_id".to_string(), json!(tx.pool_id));
        info.insert("sender".to_string(), json!(tx.sender));
        info.insert("token_0".to_string(), json!(tx.token_0));
        info.insert("token_1".to_string(), json!(tx.token_1));
        info.insert("token_0_symbol".to_string(), json!(tx.token_0_symbol));
        info.insert("token_1_symbol".to_string(), json!(tx.token_1_symbol));
        info.insert("amount_0".to_string(), json!(tx.amount_0));
        info.insert("amount_1".to_string(), json!(tx.amount_1));
        info.insert("price_0_usd".to_string(), json!(tx.price_0_usd));
        info.insert("price_1_usd".to_string(), json!(tx.price_1_usd));
        info.insert("created_at".to_string(), json!(tx.created_at));
    }
    
    info
}

/// Extracts metrics from OHLCV data
pub fn extract_ohlcv_metrics(data: &Value) -> HashMap<String, Value> {
    let mut metrics = HashMap::new();
    let records = extract_ohlcv(data);
    
    if records.is_empty() {
        return metrics;
    }
    
    let total_volume: i64 = records.iter().map(|r| r.volume).sum();
    let prices: Vec<f64> = records.iter().map(|r| r.close).collect();
    
    let (high, low) = records.iter().fold((f64::NEG_INFINITY, f64::INFINITY), |(h, l), record| {
        (h.max(record.high), l.min(record.low))
    });
    
    metrics.insert("total_volume".to_string(), json!(total_volume));
    metrics.insert("data_points".to_string(), json!(records.len()));
    metrics.insert("highest_price".to_string(), json!(high));
    metrics.insert("lowest_price".to_string(), json!(low));
    
    if !records.is_empty() {
        metrics.insert("first_price".to_string(), json!(records[0].open));
        metrics.insert("last_price".to_string(), json!(records[records.len() - 1].close));
        
        // Calculate price change
        if records[0].open > 0.0 {
            let change = ((records[records.len() - 1].close - records[0].open) / records[0].open) * 100.0;
            metrics.insert("price_change_percent".to_string(), json!(change));
        }
    }
    
    // Calculate volatility if we have enough data
    if prices.len() > 1 {
        metrics.insert("volatility".to_string(), json!(calculate_volatility(&records)));
    }
    
    metrics
}

/// Extracts and categorizes search results
pub fn extract_search_results(data: &Value) -> HashMap<String, Value> {
    let mut results = HashMap::new();
    
    if let Ok(search_resp) = serde_json::from_value::<SearchResponse>(data.clone()) {
        results.insert("tokens_count".to_string(), json!(search_resp.tokens.len()));
        results.insert("pools_count".to_string(), json!(search_resp.pools.len()));
        results.insert("dexes_count".to_string(), json!(search_resp.dexes.len()));
        results.insert("total_results".to_string(), json!(
            search_resp.tokens.len() + search_resp.pools.len() + search_resp.dexes.len()
        ));
        
        // Extract top results
        if !search_resp.tokens.is_empty() {
            results.insert("top_token".to_string(), json!(search_resp.tokens[0]));
        }
        if !search_resp.pools.is_empty() {
            results.insert("top_pool".to_string(), json!(search_resp.pools[0]));
        }
        if !search_resp.dexes.is_empty() {
            results.insert("top_dex".to_string(), json!(search_resp.dexes[0]));
        }
    }
    
    results
}

/// Extracts system statistics
pub fn extract_system_stats(data: &Value) -> HashMap<String, Value> {
    let mut stats = HashMap::new();
    
    if let Ok(system_stats) = serde_json::from_value::<SystemStats>(data.clone()) {
        stats.insert("chains".to_string(), json!(system_stats.chains));
        stats.insert("factories".to_string(), json!(system_stats.factories));
        stats.insert("pools".to_string(), json!(system_stats.pools));
        stats.insert("tokens".to_string(), json!(system_stats.tokens));
        stats.insert("total_entities".to_string(), json!(
            system_stats.chains + system_stats.factories + system_stats.pools + system_stats.tokens
        ));
    }
    
    stats
}

/// Extracts tokens from pool data
pub fn extract_pool_tokens(data: &Value) -> Vec<Token> {
    if let Ok(pool) = serde_json::from_value::<Pool>(data.clone()) {
        return pool.tokens;
    }
    
    Vec::new()
}

// ============================================================================
// FILTERING & SORTING HELPERS (12 functions)
// ============================================================================

/// Filters items by price change percentage
pub fn filter_by_price_change(pools: &[Pool], min_change: f64) -> Vec<Pool> {
    pools.iter()
        .filter(|pool| pool.last_price_change_usd_24h.abs() >= min_change)
        .cloned()
        .collect()
}

/// Filters items by minimum volume
pub fn filter_by_volume(pools: &[Pool], min_volume: f64) -> Vec<Pool> {
    pools.iter()
        .filter(|pool| pool.volume_usd >= min_volume)
        .cloned()
        .collect()
}

/// Filters items by network
pub fn filter_by_network(pools: &[Pool], network: &str) -> Vec<Pool> {
    pools.iter()
        .filter(|pool| pool.chain.to_lowercase() == network.to_lowercase())
        .cloned()
        .collect()
}

/// Filters items by DEX name
pub fn filter_by_dex(pools: &[Pool], dex_name: &str) -> Vec<Pool> {
    pools.iter()
        .filter(|pool| pool.dex_name.to_lowercase().contains(&dex_name.to_lowercase()))
        .cloned()
        .collect()
}

/// Filters pools by token symbol
pub fn filter_by_token_symbol(pools: &[Pool], symbol: &str) -> Vec<Pool> {
    pools.iter()
        .filter(|pool| {
            pool.tokens.iter().any(|token| token.symbol.to_lowercase() == symbol.to_lowercase())
        })
        .cloned()
        .collect()
}

/// Filters pools by token address
pub fn filter_by_token_address(pools: &[Pool], address: &str) -> Vec<Pool> {
    pools.iter()
        .filter(|pool| {
            pool.tokens.iter().any(|token| token.id.to_lowercase() == address.to_lowercase())
        })
        .cloned()
        .collect()
}

/// Returns the top N items by specified field
pub fn top_n(pools: &[Pool], field: &str, n: usize) -> Vec<Pool> {
    let mut sorted = sort_by_field(pools, field, true);
    sorted.truncate(n);
    sorted
}

/// Returns the bottom N items by specified field
pub fn bottom_n(pools: &[Pool], field: &str, n: usize) -> Vec<Pool> {
    let mut sorted = sort_by_field(pools, field, false);
    sorted.truncate(n);
    sorted
}

/// Sorts pools by specified field
pub fn sort_by_field(pools: &[Pool], field: &str, descending: bool) -> Vec<Pool> {
    let mut sorted = pools.to_vec();
    
    sorted.sort_by(|a, b| {
        let val_a = match field {
            "volume_usd" => a.volume_usd,
            "price_usd" => a.price_usd,
            "transactions" => a.transactions as f64,
            "last_price_change_usd_24h" => a.last_price_change_usd_24h,
            _ => return std::cmp::Ordering::Equal,
        };
        
        let val_b = match field {
            "volume_usd" => b.volume_usd,
            "price_usd" => b.price_usd,
            "transactions" => b.transactions as f64,
            "last_price_change_usd_24h" => b.last_price_change_usd_24h,
            _ => return std::cmp::Ordering::Equal,
        };
        
        if descending {
            val_b.partial_cmp(&val_a).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
    
    sorted
}

/// Filters transactions by time
pub fn filter_recent_transactions(transactions: &[Transaction], hours: u32) -> Vec<Transaction> {
    let now = Utc::now();
    let cutoff = now - chrono::Duration::hours(hours as i64);
    
    transactions.iter()
        .filter(|tx| {
            if let Ok(tx_time) = DateTime::parse_from_rfc3339(&tx.created_at) {
                tx_time.with_timezone(&Utc) > cutoff
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

/// Filters transactions by minimum USD value
pub fn filter_large_transactions(transactions: &[Transaction], min_usd: f64) -> Vec<Transaction> {
    transactions.iter()
        .filter(|tx| (tx.price_0_usd + tx.price_1_usd) >= min_usd)
        .cloned()
        .collect()
}

/// Filters OHLCV data by timeframe
pub fn filter_by_timeframe(records: &[OHLCVRecord], start_time: &str, end_time: Option<&str>) -> Vec<OHLCVRecord> {
    let start = DateTime::parse_from_rfc3339(start_time);
    if start.is_err() {
        return records.to_vec();
    }
    let start = start.unwrap().with_timezone(&Utc);
    
    let end = if let Some(end_str) = end_time {
        DateTime::parse_from_rfc3339(end_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    } else {
        Utc::now()
    };
    
    records.iter()
        .filter(|record| {
            if let Ok(record_time) = DateTime::parse_from_rfc3339(&record.time_open) {
                let record_utc = record_time.with_timezone(&Utc);
                record_utc > start && record_utc < end
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

// ============================================================================
// ANALYSIS HELPERS (10 functions)
// ============================================================================

/// Calculates percentage price change
pub fn calculate_price_change(current: f64, previous: f64) -> f64 {
    if previous == 0.0 {
        return 0.0;
    }
    ((current - previous) / previous) * 100.0
}

/// Calculates volume-weighted average price
pub fn calculate_volume_weighted_price(records: &[OHLCVRecord]) -> f64 {
    if records.is_empty() {
        return 0.0;
    }
    
    let mut total_value = 0.0;
    let mut total_volume = 0i64;
    
    for record in records {
        let avg_price = (record.high + record.low + record.close) / 3.0;
        total_value += avg_price * record.volume as f64;
        total_volume += record.volume;
    }
    
    if total_volume == 0 {
        0.0
    } else {
        total_value / total_volume as f64
    }
}

/// Calculates price volatility
pub fn calculate_volatility(records: &[OHLCVRecord]) -> f64 {
    if records.len() < 2 {
        return 0.0;
    }
    
    // Calculate returns
    let mut returns = Vec::new();
    for i in 1..records.len() {
        if records[i - 1].close > 0.0 {
            let ret = (records[i].close - records[i - 1].close) / records[i - 1].close;
            returns.push(ret);
        }
    }
    
    if returns.is_empty() {
        return 0.0;
    }
    
    // Calculate mean
    let mean: f64 = returns.iter().sum::<f64>() / returns.len() as f64;
    
    // Calculate variance
    let variance: f64 = returns.iter()
        .map(|ret| (ret - mean).powi(2))
        .sum::<f64>() / returns.len() as f64;
    
    // Return standard deviation (volatility)
    variance.sqrt()
}

/// Analyzes pool activity metrics
pub fn analyze_pool_activity(pool: &Pool) -> HashMap<String, Value> {
    let mut analysis = HashMap::new();
    
    analysis.insert("pool_id".to_string(), json!(pool.id));
    analysis.insert("dex_name".to_string(), json!(pool.dex_name));
    analysis.insert("chain".to_string(), json!(pool.chain));
    analysis.insert("volume_usd".to_string(), json!(pool.volume_usd));
    analysis.insert("transactions".to_string(), json!(pool.transactions));
    analysis.insert("price_usd".to_string(), json!(pool.price_usd));
    
    // Calculate volume per transaction
    if pool.transactions > 0 {
        analysis.insert("volume_per_transaction".to_string(), 
            json!(pool.volume_usd / pool.transactions as f64));
    }
    
    // Analyze price changes
    analysis.insert("price_change_24h".to_string(), json!(pool.last_price_change_usd_24h));
    analysis.insert("price_change_1h".to_string(), json!(pool.last_price_change_usd_1h));
    analysis.insert("price_change_5m".to_string(), json!(pool.last_price_change_usd_5m));
    
    // Activity score (arbitrary scoring based on volume and transactions)
    let activity_score = (pool.volume_usd + 1.0).log10() * (pool.transactions as f64 + 1.0).log10();
    analysis.insert("activity_score".to_string(), json!(activity_score));
    
    // Token information
    if pool.tokens.len() >= 2 {
        analysis.insert("token_pair".to_string(), 
            json!(format!("{}/{}", pool.tokens[0].symbol, pool.tokens[1].symbol)));
    }
    
    analysis
}

/// Analyzes token performance metrics
pub fn analyze_token_performance(token: &Token) -> HashMap<String, Value> {
    let mut analysis = HashMap::new();
    
    analysis.insert("token_id".to_string(), json!(token.id));
    analysis.insert("name".to_string(), json!(token.name));
    analysis.insert("symbol".to_string(), json!(token.symbol));
    analysis.insert("chain".to_string(), json!(token.chain));
    analysis.insert("fdv".to_string(), json!(token.fdv));
    
    if let Some(ref summary) = token.summary {
        analysis.insert("price_usd".to_string(), json!(summary.price_usd));
        analysis.insert("liquidity_usd".to_string(), json!(summary.liquidity_usd));
        analysis.insert("pools_count".to_string(), json!(summary.pools));
        
        // Calculate liquidity per pool
        if summary.pools > 0 {
            analysis.insert("avg_liquidity_per_pool".to_string(), 
                json!(summary.liquidity_usd / summary.pools as f64));
        }
        
        // Market cap to liquidity ratio
        if summary.liquidity_usd > 0.0 {
            analysis.insert("fdv_to_liquidity_ratio".to_string(), 
                json!(token.fdv / summary.liquidity_usd));
        }
        
        // 24h metrics
        if let Some(ref h24) = summary.h24 {
            analysis.insert("volume_24h".to_string(), json!(h24.volume_usd));
            analysis.insert("price_change_24h".to_string(), json!(h24.last_price_usd_change));
            analysis.insert("transactions_24h".to_string(), json!(h24.txns));
            
            // Trading activity ratio
            if summary.liquidity_usd > 0.0 {
                analysis.insert("volume_to_liquidity_ratio".to_string(), 
                    json!(h24.volume_usd / summary.liquidity_usd));
            }
        }
    }
    
    analysis
}

/// Detects statistical anomalies in data
pub fn detect_anomalies(pools: &[Pool], field: &str, threshold: f64) -> Vec<AnomalyResult> {
    let values: Vec<f64> = pools.iter().map(|pool| {
        match field {
            "volume_usd" => pool.volume_usd,
            "price_usd" => pool.price_usd,
            "transactions" => pool.transactions as f64,
            "last_price_change_usd_24h" => pool.last_price_change_usd_24h,
            _ => 0.0,
        }
    }).collect();
    
    if values.len() < 3 {
        return Vec::new();
    }
    
    // Calculate mean and standard deviation
    let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
    let variance: f64 = values.iter()
        .map(|val| (val - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    let std_dev = variance.sqrt();
    
    // Detect anomalies
    let mut anomalies = Vec::new();
    for (i, &val) in values.iter().enumerate() {
        let z_score = if std_dev > 0.0 { (val - mean).abs() / std_dev } else { 0.0 };
        if z_score > threshold {
            anomalies.push(AnomalyResult {
                index: i,
                value: val,
                z_score,
                item: json!(pools[i]),
            });
        }
    }
    
    anomalies
}

/// Calculates correlation between two data series
pub fn calculate_correlation(data1: &[f64], data2: &[f64]) -> f64 {
    if data1.len() != data2.len() || data1.len() < 2 {
        return 0.0;
    }
    
    let n = data1.len() as f64;
    
    // Calculate means
    let mean1: f64 = data1.iter().sum::<f64>() / n;
    let mean2: f64 = data2.iter().sum::<f64>() / n;
    
    // Calculate correlation
    let numerator: f64 = data1.iter().zip(data2.iter())
        .map(|(a, b)| (a - mean1) * (b - mean2))
        .sum();
    
    let sum_squares1: f64 = data1.iter().map(|a| (a - mean1).powi(2)).sum();
    let sum_squares2: f64 = data2.iter().map(|b| (b - mean2).powi(2)).sum();
    
    let denominator = (sum_squares1 * sum_squares2).sqrt();
    
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// Analyzes liquidity distribution across pools
pub fn analyze_liquidity_distribution(pools: &[Pool]) -> LiquidityAnalysis {
    if pools.is_empty() {
        return LiquidityAnalysis {
            total_liquidity: 0.0,
            pool_count: 0,
            average_liquidity: 0.0,
            median_liquidity: 0.0,
            gini_coefficient: 0.0,
            top_pools_share: 0.0,
            distribution: HashMap::new(),
        };
    }
    
    // Extract volume data (using as proxy for liquidity)
    let mut volumes: Vec<f64> = pools.iter().map(|p| p.volume_usd).collect();
    let total_volume: f64 = volumes.iter().sum();
    
    // Sort volumes for percentile calculations
    volumes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // Calculate metrics
    let average_liquidity = total_volume / pools.len() as f64;
    
    // Median
    let median_liquidity = if volumes.len() % 2 == 0 {
        (volumes[volumes.len() / 2 - 1] + volumes[volumes.len() / 2]) / 2.0
    } else {
        volumes[volumes.len() / 2]
    };
    
    // Gini coefficient
    let gini_coefficient = calculate_gini_coefficient(&volumes);
    
    // Top pools share (top 10%)
    let top_count = (volumes.len() / 10).max(1);
    let top_volume: f64 = volumes.iter().rev().take(top_count).sum();
    let top_pools_share = if total_volume > 0.0 { top_volume / total_volume } else { 0.0 };
    
    // Distribution by ranges
    let mut distribution = HashMap::new();
    let ranges = vec![
        ("< 1M", 0.0..1_000_000.0),
        ("1M-10M", 1_000_000.0..10_000_000.0),
        ("10M-100M", 10_000_000.0..100_000_000.0),
        ("> 100M", 100_000_000.0..f64::INFINITY),
    ];
    
    for (name, range) in ranges {
        let count = volumes.iter().filter(|&&vol| range.contains(&vol)).count();
        distribution.insert(name.to_string(), count as f64 / volumes.len() as f64);
    }
    
    LiquidityAnalysis {
        total_liquidity: total_volume,
        pool_count: pools.len(),
        average_liquidity,
        median_liquidity,
        gini_coefficient,
        top_pools_share,
        distribution,
    }
}

/// Calculates the Gini coefficient for inequality measurement
pub fn calculate_gini_coefficient(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len() as f64;
    let mean: f64 = sorted.iter().sum::<f64>() / n;
    
    if mean == 0.0 {
        return 0.0;
    }
    
    let sum: f64 = sorted.iter().enumerate()
        .map(|(i, &val)| val * (2.0 * i as f64 + 1.0 - n))
        .sum();
    
    sum / (n * n * mean)
}

/// Analyzes transaction patterns
pub fn analyze_transaction_patterns(transactions: &[Transaction]) -> HashMap<String, Value> {
    let mut analysis = HashMap::new();
    
    if transactions.is_empty() {
        return analysis;
    }
    
    analysis.insert("total_transactions".to_string(), json!(transactions.len()));
    
    // Analyze by hour
    let mut hour_counts = HashMap::new();
    let mut total_usd = 0.0;
    
    for tx in transactions {
        total_usd += tx.price_0_usd + tx.price_1_usd;
        
        if let Ok(tx_time) = DateTime::parse_from_rfc3339(&tx.created_at) {
            let hour = tx_time.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }
    }
    
    analysis.insert("total_value_usd".to_string(), json!(total_usd));
    analysis.insert("avg_value_per_tx".to_string(), json!(total_usd / transactions.len() as f64));
    
    // Find peak hour
    let (peak_hour, max_count) = hour_counts.iter()
        .max_by_key(|(_, &count)| count)
        .map(|(&hour, &count)| (hour, count))
        .unwrap_or((0, 0));
    
    analysis.insert("peak_hour".to_string(), json!(peak_hour));
    analysis.insert("peak_hour_count".to_string(), json!(max_count));
    analysis.insert("hourly_distribution".to_string(), json!(hour_counts));
    
    // Analyze token pairs
    let mut pair_counts = HashMap::new();
    for tx in transactions {
        let pair = format!("{}/{}", tx.token_0_symbol, tx.token_1_symbol);
        *pair_counts.entry(pair).or_insert(0) += 1;
    }
    
    analysis.insert("unique_pairs".to_string(), json!(pair_counts.len()));
    analysis.insert("pair_distribution".to_string(), json!(pair_counts));
    
    analysis
}

// ============================================================================
// UTILITY HELPERS (8 functions)
// ============================================================================

/// Formats a number with thousand separators
pub fn format_number(num: f64, decimals: usize) -> String {
    let formatted = format!("{:.prec$}", num, prec = decimals);
    let parts: Vec<&str> = formatted.split('.').collect();
    
    let int_part = parts[0];
    let mut result = String::new();
    
    for (i, ch) in int_part.chars().enumerate() {
        if i > 0 && (int_part.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    
    if parts.len() > 1 {
        result.push('.');
        result.push_str(parts[1]);
    }
    
    result
}

/// Formats a number as a percentage
pub fn format_percentage(num: f64, decimals: usize) -> String {
    format!("{:.prec$}%", num, prec = decimals)
}

/// Prints data in a formatted table
pub fn print_table(pools: &[Pool], columns: &[&str], title: &str) {
    use comfy_table::{Table, presets::UTF8_FULL};
    
    if pools.is_empty() {
        println!("No data to display");
        return;
    }
    
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(columns);
    
    for pool in pools {
        let mut row = Vec::new();
        for &col in columns {
            let value = match col {
                "dex_name" => pool.dex_name.clone(),
                "volume_usd" => format_number(pool.volume_usd, 0),
                "price_usd" => format_number(pool.price_usd, 6),
                "transactions" => pool.transactions.to_string(),
                "chain" => pool.chain.clone(),
                "last_price_change_usd_24h" => format_percentage(pool.last_price_change_usd_24h, 2),
                _ => String::new(),
            };
            row.push(value);
        }
        table.add_row(row);
    }
    
    println!("{}", title);
    println!("{}", table);
}

/// Saves data to a CSV file
pub fn save_to_csv(pools: &[Pool], filename: &str, columns: &[&str]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(filename)?;
    
    // Write header
    wtr.write_record(columns)?;
    
    // Write data
    for pool in pools {
        let mut record = Vec::new();
        for &col in columns {
            let value = match col {
                "id" => pool.id.clone(),
                "dex_name" => pool.dex_name.clone(),
                "volume_usd" => pool.volume_usd.to_string(),
                "price_usd" => pool.price_usd.to_string(),
                "transactions" => pool.transactions.to_string(),
                "chain" => pool.chain.clone(),
                "last_price_change_usd_24h" => pool.last_price_change_usd_24h.to_string(),
                "created_at" => pool.created_at.clone(),
                _ => String::new(),
            };
            record.push(value);
        }
        wtr.write_record(&record)?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Loads environment variable with default value
pub fn load_env(key: &str, default_value: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default_value.to_string())
}

/// Validates if a network is supported
pub async fn validate_network(network: &str) -> Result<bool> {
    let networks = get_networks().await?;
    Ok(networks.iter().any(|net| net.id.to_lowercase() == network.to_lowercase()))
}

/// Validates if a token address format is correct
pub fn validate_token_address(address: &str) -> bool {
    // Ethereum address pattern (0x followed by 40 hex characters)
    if let Ok(eth_regex) = Regex::new(r"^0x[a-fA-F0-9]{40}$") {
        if eth_regex.is_match(address) {
            return true;
        }
    }
    
    // Solana address pattern (base58, 32-44 characters)
    if let Ok(sol_regex) = Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{32,44}$") {
        if sol_regex.is_match(address) {
            return true;
        }
    }
    
    false
}

/// Creates an ISO timestamp for days ago
pub fn create_timestamp(days_ago: i64) -> String {
    let timestamp = Utc::now() - chrono::Duration::days(days_ago);
    timestamp.to_rfc3339()
}

// ============================================================================
// ADVANCED HELPERS (5 functions)
// ============================================================================

/// Retrieves top price movers with volume filter
pub async fn get_top_movers(network: &str, limit: u32, min_volume: f64) -> Result<Vec<Pool>> {
    let params = ApiParams::new().limit(limit * 3); // Get more to filter
    let response = get_network_pools(network, Some(params)).await?;
    
    // Filter by volume and sort by absolute price change
    let mut filtered = filter_by_volume(&response.pools, min_volume);
    
    // Sort by absolute price change
    filtered.sort_by(|a, b| {
        b.last_price_change_usd_24h.abs()
            .partial_cmp(&a.last_price_change_usd_24h.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    filtered.truncate(limit as usize);
    Ok(filtered)
}

/// Retrieves pools with highest volume
pub async fn get_high_volume_pools(network: &str, limit: u32) -> Result<Vec<Pool>> {
    let params = ApiParams::new()
        .limit(limit)
        .order_by("volume_usd")
        .sort("desc");
    
    let response = get_network_pools(network, Some(params)).await?;
    Ok(response.pools)
}

/// Analyzes token liquidity across pools
pub async fn get_token_liquidity_analysis(token_address: &str, network: &str) -> Result<HashMap<String, Value>> {
    // Get token details
    let token = get_token_details(network, token_address).await?;
    
    // Get token pools
    let pools_resp = get_token_pools(network, token_address, 
        Some(ApiParams::new().limit(50))).await?;
    
    let mut analysis = HashMap::new();
    analysis.insert("token".to_string(), json!(token));
    analysis.insert("pools_count".to_string(), json!(pools_resp.pools.len()));
    
    if pools_resp.pools.is_empty() {
        return Ok(analysis);
    }
    
    // Analyze liquidity distribution
    let liquidity_analysis = analyze_liquidity_distribution(&pools_resp.pools);
    analysis.insert("liquidity_analysis".to_string(), json!(liquidity_analysis));
    
    // Find largest pool
    let largest_pool = pools_resp.pools.iter()
        .max_by(|a, b| a.volume_usd.partial_cmp(&b.volume_usd).unwrap_or(std::cmp::Ordering::Equal))
        .cloned();
    
    if let Some(pool) = largest_pool {
        analysis.insert("largest_pool".to_string(), json!(pool));
    }
    
    // DEX distribution
    let dex_distribution = analyze_dex_distribution(&pools_resp.pools);
    analysis.insert("dex_distribution".to_string(), json!(dex_distribution));
    
    Ok(analysis)
}

/// Analyzes DEX distribution across pools
pub fn analyze_dex_distribution(pools: &[Pool]) -> DexDistribution {
    if pools.is_empty() {
        return DexDistribution {
            total_volume: 0.0,
            dex_count: 0,
            distribution: HashMap::new(),
            top_dexes: Vec::new(),
            concentration: 0.0,
        };
    }
    
    let mut dex_volumes = HashMap::new();
    let mut total_volume = 0.0;
    
    for pool in pools {
        *dex_volumes.entry(pool.dex_name.clone()).or_insert(0.0) += pool.volume_usd;
        total_volume += pool.volume_usd;
    }
    
    // Calculate percentages
    let mut distribution = HashMap::new();
    let mut top_dexes: Vec<String> = dex_volumes.keys().cloned().collect();
    
    for (dex, volume) in &dex_volumes {
        if total_volume > 0.0 {
            let percentage = volume / total_volume;
            distribution.insert(dex.clone(), percentage);
        }
    }
    
    // Sort DEXes by volume
    top_dexes.sort_by(|a, b| {
        dex_volumes.get(b).unwrap_or(&0.0)
            .partial_cmp(dex_volumes.get(a).unwrap_or(&0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Calculate concentration (Herfindahl-Hirschman Index)
    let concentration: f64 = distribution.values().map(|&p| p * p).sum();
    
    DexDistribution {
        total_volume,
        dex_count: dex_volumes.len(),
        distribution,
        top_dexes,
        concentration,
    }
}

/// Retrieves comprehensive market overview
pub async fn get_market_overview() -> Result<MarketOverview> {
    // Get system stats
    let stats = get_system_stats().await?;
    
    // Get networks
    let networks = get_networks().await?;
    
    let mut network_overview = HashMap::new();
    
    // Analyze top 5 networks
    for (i, network) in networks.iter().enumerate() {
        if i >= 5 {
            break;
        }
        
        if let Ok(pools) = get_network_pools(&network.id, Some(ApiParams::new().limit(10))).await {
            let total_volume: f64 = pools.pools.iter().map(|p| p.volume_usd).sum();
            
            network_overview.insert(network.id.clone(), json!({
                "display_name": network.display_name,
                "total_volume": total_volume,
                "pool_count": pools.pools.len(),
            }));
        }
    }
    
    Ok(MarketOverview {
        system_stats: stats,
        network_overview,
        timestamp: Utc::now().to_rfc3339(),
    })
}

// ============================================================================
// ASYNC HELPERS (5 functions)
// ============================================================================

/// Makes an async API request with timeout
pub async fn async_api_request(endpoint: &str, params: Option<ApiParams>) -> Result<Value> {
    // Same as api_request but explicitly async
    api_request(endpoint, params).await
}

/// Gets pools from multiple networks concurrently
pub async fn async_get_multiple_pools(networks: &[String], limit: u32) -> Result<HashMap<String, Value>> {
    let mut results = HashMap::new();
    let mut handles = Vec::new();
    
    for network in networks {
        let net = network.clone();
        let handle = tokio::spawn(async move {
            let params = ApiParams::new().limit(limit);
            (net.clone(), api_request(&format!("/networks/{}/pools", net), Some(params)).await)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        if let Ok((network, result)) = handle.await {
            match result {
                Ok(data) => {
                    results.insert(network, data);
                }
                Err(e) => {
                    results.insert(network, json!({"error": e.to_string()}));
                }
            }
        }
    }
    
    Ok(results)
}

/// Gets data for multiple tokens concurrently
pub async fn async_get_token_data_batch(token_addresses: &[String], network: &str) -> Result<Vec<AsyncResult<Token>>> {
    let mut handles = Vec::new();
    
    for (index, address) in token_addresses.iter().enumerate() {
        let addr = address.clone();
        let net = network.to_string();
        let handle = tokio::spawn(async move {
            let endpoint = format!("/networks/{}/tokens/{}", net, addr);
            let result = api_request(&endpoint, None).await;
            (index, result)
        });
        handles.push(handle);
    }
    
    let mut results = vec![AsyncResult { network: None, query: None, data: None, error: None }; token_addresses.len()];
    
    for handle in handles {
        if let Ok((index, result)) = handle.await {
            match result {
                Ok(data) => {
                    if let Ok(token) = serde_json::from_value::<Token>(data) {
                        results[index] = AsyncResult {
                            network: Some(network.to_string()),
                            query: None,
                            data: Some(token),
                            error: None,
                        };
                    }
                }
                Err(e) => {
                    results[index] = AsyncResult {
                        network: Some(network.to_string()),
                        query: None,
                        data: None,
                        error: Some(e.to_string()),
                    };
                }
            }
        }
    }
    
    Ok(results)
}

/// Monitors pool prices with callback
pub async fn async_monitor_prices<F>(
    pool_addresses: &[String], 
    network: &str, 
    interval: Duration, 
    mut callback: F
) -> Result<()> 
where 
    F: FnMut(&str, HashMap<String, Value>) + Send + 'static,
{
    let mut interval_timer = tokio::time::interval(interval);
    
    loop {
        interval_timer.tick().await;
        
        for address in pool_addresses {
            let network_clone = network.to_string();
            let address_clone = address.clone();
            
            if let Ok(pool) = get_pool_details(&network_clone, &address_clone, None).await {
                let mut price_update = HashMap::new();
                price_update.insert("price_usd".to_string(), json!(pool.price_usd));
                price_update.insert("last_price_change_usd_24h".to_string(), json!(pool.last_price_change_usd_24h));
                price_update.insert("volume_usd".to_string(), json!(pool.volume_usd));
                price_update.insert("timestamp".to_string(), json!(Utc::now().to_rfc3339()));
                
                callback(&address_clone, price_update);
            }
        }
    }
}

/// Performs multiple searches concurrently
pub async fn async_batch_search(queries: &[String]) -> Result<Vec<AsyncResult<SearchResponse>>> {
    let mut handles = Vec::new();
    
    for (index, query) in queries.iter().enumerate() {
        let q = query.clone();
        let handle = tokio::spawn(async move {
            let endpoint = format!("/search?query={}", q.replace(" ", "%20"));
            let result = api_request(&endpoint, None).await;
            (index, q, result)
        });
        handles.push(handle);
    }
    
    let mut results = vec![AsyncResult { network: None, query: None, data: None, error: None }; queries.len()];
    
    for handle in handles {
        if let Ok((index, query, result)) = handle.await {
            match result {
                Ok(data) => {
                    if let Ok(search_resp) = serde_json::from_value::<SearchResponse>(data) {
                        results[index] = AsyncResult {
                            network: None,
                            query: Some(query),
                            data: Some(search_resp),
                            error: None,
                        };
                    }
                }
                Err(e) => {
                    results[index] = AsyncResult {
                        network: None,
                        query: Some(query),
                        data: None,
                        error: Some(e.to_string()),
                    };
                }
            }
        }
    }
    
    Ok(results)
}

// ============================================================================
// Re-exports for easier usage
// ============================================================================

// All functions are available at the root level 