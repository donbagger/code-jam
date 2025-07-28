//! Type definitions for DexPaprika API responses
//! 
//! This module contains all the Rust types that correspond to the DexPaprika API
//! data structures, with full serde support for JSON serialization/deserialization.

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Custom deserializer that converts null to 0.0 for f64 fields
fn deserialize_f64_from_null<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<f64> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(0.0))
}

/// Network represents a blockchain network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Network {
    pub id: String,
    pub display_name: String,
}

/// Token represents a token with all its metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub chain: String,
    #[serde(rename = "type", default)]
    pub token_type: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub decimals: u32,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub total_supply: f64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub explorer: String,
    #[serde(default)]
    pub added_at: String,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub fdv: f64,
    #[serde(default)]
    pub last_updated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<TokenSummary>,
}

/// TokenSummary represents token summary metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenSummary {
    pub price_usd: f64,
    pub fdv: f64,
    pub liquidity_usd: f64,
    pub pools: u32,
    #[serde(rename = "24h", skip_serializing_if = "Option::is_none")]
    pub h24: Option<TimeIntervalMetrics>,
    #[serde(rename = "6h", skip_serializing_if = "Option::is_none")]
    pub h6: Option<TimeIntervalMetrics>,
    #[serde(rename = "1h", skip_serializing_if = "Option::is_none")]
    pub h1: Option<TimeIntervalMetrics>,
    #[serde(rename = "30m", skip_serializing_if = "Option::is_none")]
    pub m30: Option<TimeIntervalMetrics>,
    #[serde(rename = "15m", skip_serializing_if = "Option::is_none")]
    pub m15: Option<TimeIntervalMetrics>,
    #[serde(rename = "5m", skip_serializing_if = "Option::is_none")]
    pub m5: Option<TimeIntervalMetrics>,
    #[serde(rename = "1m", skip_serializing_if = "Option::is_none")]
    pub m1: Option<TimeIntervalMetrics>,
}

/// TimeIntervalMetrics represents transaction and volume metrics for a time interval
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeIntervalMetrics {
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub volume: f64,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub volume_usd: f64,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub buy_usd: f64,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub sell_usd: f64,
    #[serde(default)]
    pub sells: u32,
    #[serde(default)]
    pub buys: u32,
    #[serde(default)]
    pub txns: u32,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub last_price_usd_change: f64,
}

/// Pool represents a liquidity pool with pricing data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pool {
    pub id: String,
    pub dex_id: String,
    pub dex_name: String,
    pub chain: String,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub volume_usd: f64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub created_at_block_number: i64,
    #[serde(default)]
    pub transactions: u32,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub price_usd: f64,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub last_price_change_usd_5m: f64,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub last_price_change_usd_1h: f64,
    #[serde(default, deserialize_with = "deserialize_f64_from_null")]
    pub last_price_change_usd_24h: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    #[serde(default)]
    pub tokens: Vec<Token>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_time: Option<String>,
    #[serde(rename = "24h", skip_serializing_if = "Option::is_none")]
    pub h24: Option<TimeIntervalMetrics>,
    #[serde(rename = "6h", skip_serializing_if = "Option::is_none")]
    pub h6: Option<TimeIntervalMetrics>,
    #[serde(rename = "1h", skip_serializing_if = "Option::is_none")]
    pub h1: Option<TimeIntervalMetrics>,
    #[serde(rename = "30m", skip_serializing_if = "Option::is_none")]
    pub m30: Option<TimeIntervalMetrics>,
    #[serde(rename = "15m", skip_serializing_if = "Option::is_none")]
    pub m15: Option<TimeIntervalMetrics>,
    #[serde(rename = "5m", skip_serializing_if = "Option::is_none")]
    pub m5: Option<TimeIntervalMetrics>,
}

/// PoolsResponse represents the response from pools endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolsResponse {
    pub pools: Vec<Pool>,
    pub page_info: PageInfo,
}

/// PageInfo represents pagination information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageInfo {
    pub limit: u32,
    pub page: u32,
    pub total_items: u32,
    pub total_pages: u32,
}

/// Dex represents a decentralized exchange
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dex {
    pub id: String,
    pub dex_id: String,
    pub dex_name: String,
    pub chain: String,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usd_24h: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txns_24h: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pools_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// DexesResponse represents the response from dexes endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DexesResponse {
    pub dexes: Vec<Dex>,
    pub page_info: PageInfo,
}

/// OHLCVRecord represents OHLCV data point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OHLCVRecord {
    pub time_open: String,
    pub time_close: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

/// Transaction represents a transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<f64>,
    pub pool_id: String,
    pub sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<f64>,
    pub token_0: String,
    pub token_0_symbol: String,
    pub token_1: String,
    pub token_1_symbol: String,
    pub amount_0: String,
    pub amount_1: String,
    pub price_0: f64,
    pub price_1: f64,
    pub price_0_usd: f64,
    pub price_1_usd: f64,
    pub created_at_block_number: f64,
    pub created_at: String,
}

/// TransactionsResponse represents the response from transactions endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionsResponse {
    pub transactions: Vec<Transaction>,
    pub page_info: serde_json::Value, // Can be PageInfo or cursor-based
}

/// TokenPoolsResponse represents the response from token pools endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenPoolsResponse {
    pub pools: Vec<Pool>,
    pub page_info: PageInfo,
}

/// SearchResponse represents the response from search endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    pub tokens: Vec<Token>,
    pub pools: Vec<Pool>,
    pub dexes: Vec<Dex>,
}

/// SystemStats represents system statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemStats {
    pub chains: u32,
    pub factories: u32,
    pub pools: u32,
    pub tokens: u32,
}

/// APIError represents an API error response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct APIError {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// CacheEntry represents a cached API response
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

/// AnalysisResult represents the result of various analysis functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisResult {
    pub field: String,
    pub value: serde_json::Value,
    pub timestamp: String,
}

/// AnomalyResult represents detected anomalies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnomalyResult {
    pub index: usize,
    pub value: f64,
    pub z_score: f64,
    pub item: serde_json::Value,
}

/// LiquidityAnalysis represents liquidity distribution analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiquidityAnalysis {
    pub total_liquidity: f64,
    pub pool_count: usize,
    pub average_liquidity: f64,
    pub median_liquidity: f64,
    pub gini_coefficient: f64,
    pub top_pools_share: f64,
    pub distribution: HashMap<String, f64>,
}

/// DexDistribution represents DEX distribution analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DexDistribution {
    pub total_volume: f64,
    pub dex_count: usize,
    pub distribution: HashMap<String, f64>,
    pub top_dexes: Vec<String>,
    pub concentration: f64,
}

/// MarketOverview represents overall market statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketOverview {
    pub system_stats: SystemStats,
    pub network_overview: HashMap<String, serde_json::Value>,
    pub timestamp: String,
}

/// AsyncResult represents the result of async operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AsyncResult<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Common parameters for API requests
#[derive(Debug, Clone, Default)]
pub struct ApiParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
    pub order_by: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub interval: Option<String>,
    pub inversed: Option<bool>,
    pub cursor: Option<String>,
    pub reorder: Option<bool>,
    pub address: Option<String>,
}

impl ApiParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_string());
        self
    }

    pub fn order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    pub fn start(mut self, start: &str) -> Self {
        self.start = Some(start.to_string());
        self
    }

    pub fn interval(mut self, interval: &str) -> Self {
        self.interval = Some(interval.to_string());
        self
    }

    pub fn inversed(mut self, inversed: bool) -> Self {
        self.inversed = Some(inversed);
        self
    }

    /// Convert to URL query parameters
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(ref sort) = self.sort {
            params.push(("sort", sort.clone()));
        }
        if let Some(ref order_by) = self.order_by {
            params.push(("order_by", order_by.clone()));
        }
        if let Some(ref start) = self.start {
            params.push(("start", start.clone()));
        }
        if let Some(ref end) = self.end {
            params.push(("end", end.clone()));
        }
        if let Some(ref interval) = self.interval {
            params.push(("interval", interval.clone()));
        }
        if let Some(inversed) = self.inversed {
            params.push(("inversed", inversed.to_string()));
        }
        if let Some(ref cursor) = self.cursor {
            params.push(("cursor", cursor.clone()));
        }
        if let Some(reorder) = self.reorder {
            params.push(("reorder", reorder.to_string()));
        }
        if let Some(ref address) = self.address {
            params.push(("address", address.clone()));
        }

        params
    }
} 