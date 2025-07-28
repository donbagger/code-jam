#!/usr/bin/env python3
"""
Paprika Vibe-Code Helper Library
A comprehensive collection of 60+ micro-helpers for DexPaprika API integration
Covers ALL endpoints and data structures from the OpenAPI specification
"""

import requests
import pandas as pd
import json
import time
import math
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Union, Any, Tuple
import os
from pathlib import Path
import matplotlib.pyplot as plt
import seaborn as sns
from rich.console import Console
from rich.table import Table
from rich import print as rprint
import asyncio
import aiohttp
from dataclasses import dataclass
import logging
from functools import wraps
import hashlib
import pickle

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Rich console for pretty output
console = Console()

# Base API URL
BASE_URL = "https://api.dexpaprika.com"

# Cache configuration
CACHE_DIR = Path(".cache")
CACHE_DIR.mkdir(exist_ok=True)
CACHE_DURATION = 300  # 5 minutes

@dataclass
class TimeIntervalMetrics:
    """Time interval metrics data structure"""
    volume: float
    volume_usd: float
    buy_usd: float
    sell_usd: float
    sells: int
    buys: int
    txns: int
    last_price_usd_change: float

@dataclass
class OHLCVRecord:
    """OHLCV data structure"""
    time_open: str
    time_close: str
    open: float
    high: float
    low: float
    close: float
    volume: int

# ============================================================================
# CORE API HELPERS (10 functions)
# ============================================================================

def api_request(endpoint: str, params: Dict = None) -> Dict:
    """Make API request with improved error handling and caching"""
    cache_key = hashlib.md5(f"{endpoint}{json.dumps(params or {}, sort_keys=True)}".encode()).hexdigest()
    cache_file = CACHE_DIR / f"{cache_key}.pkl"
    
    # Check cache
    if cache_file.exists() and time.time() - cache_file.stat().st_mtime < CACHE_DURATION:
        try:
            with open(cache_file, 'rb') as f:
                return pickle.load(f)
        except (pickle.PickleError, EOFError):
            # Cache corrupted, continue with API call
            cache_file.unlink(missing_ok=True)
    
    # Retry mechanism for failed requests
    retries = 3
    last_error = None
    
    for attempt in range(retries):
        try:
            response = requests.get(f"{BASE_URL}{endpoint}", params=params, timeout=10)
            
            # Validate response status
            if response.status_code >= 400:
                response.raise_for_status()
            
            # Validate response content
            if not response.content or len(response.content.strip()) == 0:
                raise requests.exceptions.RequestException("Empty response body")
            
            data = response.json()
            
            # Cache successful response
            with open(cache_file, 'wb') as f:
                pickle.dump(data, f)
            
            return data
            
        except (requests.exceptions.RequestException, json.JSONDecodeError, ValueError) as e:
            last_error = e
            
            if attempt < retries - 1:
                # Wait before retry (exponential backoff)
                time.sleep((attempt + 1) * 1.0)
                continue
    
    logger.error(f"API request failed after {retries} retries: {last_error}")
    return {"error": str(last_error)}

def get_networks() -> List[Dict]:
    """Get all supported blockchain networks"""
    return api_request("/networks")

def get_network_pools(network: str, **opts) -> Dict:
    """Get pools for a specific network"""
    return api_request(f"/networks/{network}/pools", opts)

def get_dex_pools(network: str, dex: str, **opts) -> Dict:
    """Get pools for a specific DEX on a network"""
    return api_request(f"/networks/{network}/dexes/{dex}/pools", opts)

def get_network_dexes(network: str, **opts) -> Dict:
    """Get DEXes available on a network"""
    return api_request(f"/networks/{network}/dexes", opts)

def get_pool_details(network: str, pool_address: str, inversed: bool = False) -> Dict:
    """Get detailed information about a specific pool"""
    return api_request(f"/networks/{network}/pools/{pool_address}", {"inversed": inversed})

def get_pool_ohlcv(network: str, pool_address: str, start: str, **opts) -> List[Dict]:
    """Get OHLCV data for a pool"""
    opts["start"] = start
    return api_request(f"/networks/{network}/pools/{pool_address}/ohlcv", opts)

def get_pool_transactions(network: str, pool_address: str, **opts) -> Dict:
    """Get transactions for a specific pool"""
    return api_request(f"/networks/{network}/pools/{pool_address}/transactions", opts)

def get_token_details(network: str, token_address: str) -> Dict:
    """Get detailed information about a specific token"""
    return api_request(f"/networks/{network}/tokens/{token_address}")

def get_token_pools(network: str, token_address: str, **opts) -> Dict:
    """Get pools containing a specific token"""
    return api_request(f"/networks/{network}/tokens/{token_address}/pools", opts)

def search_entities(query: str) -> Dict:
    """Search across tokens, pools, and DEXes"""
    return api_request("/search", {"query": query})

def get_system_stats() -> Dict:
    """Get high-level system statistics"""
    return api_request("/stats")

# ============================================================================
# DATA EXTRACTION HELPERS (15 functions)
# ============================================================================

def extract_pools(data: Dict) -> List[Dict]:
    """Extract pools from API response"""
    return data.get("pools", []) if isinstance(data, dict) else data

def extract_tokens(data: Dict) -> List[Dict]:
    """Extract tokens from API response"""
    return data.get("tokens", []) if isinstance(data, dict) else data

def extract_dexes(data: Dict) -> List[Dict]:
    """Extract DEXes from API response"""
    return data.get("dexes", []) if isinstance(data, dict) else data

def extract_transactions(data: Dict) -> List[Dict]:
    """Extract transactions from API response"""
    return data.get("transactions", []) if isinstance(data, dict) else data

def extract_ohlcv(data: List[Dict]) -> List[Dict]:
    """Extract OHLCV data from API response"""
    return data if isinstance(data, list) else []

def extract_page_info(data: Dict) -> Dict:
    """Extract pagination info from API response"""
    return data.get("page_info", {}) if isinstance(data, dict) else {}

def extract_token_summary(token_data: Dict) -> Dict:
    """Extract token summary metrics"""
    return token_data.get("summary", {}) if isinstance(token_data, dict) else {}

def extract_time_metrics(token_data: Dict, timeframe: str = "24h") -> Dict:
    """Extract time interval metrics from token data"""
    summary = extract_token_summary(token_data)
    return summary.get(timeframe, {}) if isinstance(summary, dict) else {}

def extract_pool_metrics(pool_data: Dict) -> Dict:
    """Extract key metrics from pool data"""
    return {
        "price_usd": pool_data.get("price_usd", 0),
        "volume_usd": pool_data.get("volume_usd", 0),
        "transactions": pool_data.get("transactions", 0),
        "last_price_change_24h": pool_data.get("last_price_change_usd_24h", 0),
        "fee": pool_data.get("fee", 0)
    }

def extract_token_info(token_data: Dict) -> Dict:
    """Extract basic token information"""
    return {
        "id": token_data.get("id", ""),
        "name": token_data.get("name", ""),
        "symbol": token_data.get("symbol", ""),
        "chain": token_data.get("chain", ""),
        "decimals": token_data.get("decimals", 0),
        "price_usd": token_data.get("summary", {}).get("price_usd", 0) if token_data.get("summary") else 0
    }

def extract_transaction_info(tx_data: Dict) -> Dict:
    """Extract key transaction information with improved symbol extraction"""
    
    def get_token_symbol(data: Dict, token_index: int) -> str:
        """Try multiple possible field names for token symbols"""
        possible_paths = [
            f"token_{token_index}_symbol",
            f"tokens.{token_index}.symbol",
            f"token{token_index}.symbol",
            f"pool.tokens.{token_index}.symbol",
            f"pool.token_{token_index}.symbol"
        ]
        
        for path in possible_paths:
            value = get_nested_value(data, path)
            if value and isinstance(value, str) and value.strip():
                return value.strip()
        
        # Fallback to token address
        address_paths = [
            f"token_{token_index}_address",
            f"tokens.{token_index}.address",
            f"token{token_index}.address"
        ]
        
        for path in address_paths:
            address = get_nested_value(data, path)
            if address and isinstance(address, str) and len(address) > 6:
                return f"{address[:6]}...{address[-4:]}"
        
        return f"Token{token_index}"
    
    def validate_price(price: Union[float, str, None]) -> float:
        """Validate and sanitize prices"""
        try:
            num = float(price) if price is not None else 0.0
            return num if num > 0 and math.isfinite(num) else 0.0
        except (ValueError, TypeError):
            return 0.0
    
    return {
        "id": tx_data.get("id", ""),
        "pool_id": tx_data.get("pool_id", ""),
        "token_0_symbol": get_token_symbol(tx_data, 0),
        "token_1_symbol": get_token_symbol(tx_data, 1),
        "amount_0": tx_data.get("amount_0", ""),
        "amount_1": tx_data.get("amount_1", ""),
        "price_0_usd": validate_price(tx_data.get("price_0_usd")),
        "price_1_usd": validate_price(tx_data.get("price_1_usd")),
        "created_at": tx_data.get("created_at", "")
    }

def extract_ohlcv_metrics(ohlcv_data: List[Dict]) -> Dict:
    """Extract key metrics from OHLCV data"""
    if not ohlcv_data:
        return {}
    
    latest = ohlcv_data[-1]
    return {
        "open": latest.get("open", 0),
        "high": latest.get("high", 0),
        "low": latest.get("low", 0),
        "close": latest.get("close", 0),
        "volume": latest.get("volume", 0),
        "time_open": latest.get("time_open", ""),
        "time_close": latest.get("time_close", "")
    }

def extract_search_results(search_data: Dict) -> Dict:
    """Extract search results by category"""
    return {
        "tokens": search_data.get("tokens", []),
        "pools": search_data.get("pools", []),
        "dexes": search_data.get("dexes", [])
    }

def extract_system_stats(stats_data: Dict) -> Dict:
    """Extract system statistics"""
    return {
        "chains": stats_data.get("chains", 0),
        "factories": stats_data.get("factories", 0),
        "pools": stats_data.get("pools", 0),
        "tokens": stats_data.get("tokens", 0)
    }

def extract_pool_tokens(pool_data: Dict) -> List[Dict]:
    """Extract token information from pool data"""
    return pool_data.get("tokens", []) if isinstance(pool_data, dict) else []

# ============================================================================
# DATA VALIDATION & UTILITY HELPERS
# ============================================================================

def get_nested_value(obj: Dict, path: str) -> Any:
    """Get nested object value safely using dot notation"""
    if not obj or not isinstance(obj, dict):
        return None
    
    keys = path.split('.')
    current = obj
    
    for key in keys:
        if current is None or not isinstance(current, dict):
            return None
        
        # Handle array indices
        if key.isdigit():
            try:
                index = int(key)
                if isinstance(current, list) and 0 <= index < len(current):
                    current = current[index]
                else:
                    return None
            except (ValueError, IndexError):
                return None
        else:
            current = current.get(key)
    
    return current

def validate_price(price: Union[float, str, None]) -> float:
    """Validate and sanitize price data"""
    try:
        num = float(price) if price is not None else 0.0
        return num if num > 0 and math.isfinite(num) else 0.0
    except (ValueError, TypeError):
        return 0.0

def calculate_safe_percentage(current: Union[float, str], previous: Union[float, str], max_percent: float = 10000.0) -> float:
    """Calculate safe percentage change with bounds checking"""
    curr_price = validate_price(current)
    prev_price = validate_price(previous)
    
    if prev_price == 0 or curr_price == 0:
        return 0.0
    
    change = ((curr_price - prev_price) / prev_price) * 100
    
    # Cap extreme values
    if change > max_percent:
        return max_percent
    if change < -max_percent:
        return -max_percent
    
    return change

def clean_pool_data(pools: List[Dict]) -> List[Dict]:
    """Clean and validate pool data for safe processing"""
    if not isinstance(pools, list):
        return []
    
    cleaned_pools = []
    for pool in pools:
        if not isinstance(pool, dict):
            continue
        
        # Clean the pool data
        cleaned_pool = pool.copy()
        cleaned_pool['price_usd'] = validate_price(pool.get('price_usd'))
        cleaned_pool['volume_usd'] = max(0.0, float(pool.get('volume_usd', 0)) if pool.get('volume_usd') is not None else 0.0)
        cleaned_pool['transactions'] = max(0, int(pool.get('transactions', 0)) if pool.get('transactions') is not None else 0)
        
        # Validate percentage change
        price_change = pool.get('last_price_change_usd_24h')
        if price_change is not None and math.isfinite(float(price_change)):
            cleaned_pool['last_price_change_usd_24h'] = float(price_change)
        else:
            cleaned_pool['last_price_change_usd_24h'] = 0.0
        
        # Only include pools with valid data
        if cleaned_pool['price_usd'] > 0 or cleaned_pool['volume_usd'] > 0:
            cleaned_pools.append(cleaned_pool)
    
    return cleaned_pools

# ============================================================================
# FILTERING & SORTING HELPERS (12 functions)
# ============================================================================

def filter_by_price_change(data: List[Dict], min_change: float) -> List[Dict]:
    """Filter pools by minimum price change percentage"""
    return [item for item in data if abs(item.get("last_price_change_usd_24h", 0)) >= min_change]

def filter_by_volume(data: List[Dict], min_volume: float) -> List[Dict]:
    """Filter pools by minimum volume with validation"""
    if not isinstance(data, list):
        return []
    
    def validate_volume(volume):
        try:
            num = float(volume) if volume is not None else 0.0
            return num if math.isfinite(num) and num >= 0 else 0.0
        except (ValueError, TypeError):
            return 0.0
    
    return [item for item in data if validate_volume(item.get("volume_usd")) >= min_volume]

def filter_by_network(data: List[Dict], network: str) -> List[Dict]:
    """Filter data by network"""
    return [item for item in data if item.get("chain", "").lower() == network.lower()]

def filter_by_dex(data: List[Dict], dex_name: str) -> List[Dict]:
    """Filter pools by DEX name"""
    return [item for item in data if dex_name.lower() in item.get("dex_name", "").lower()]

def filter_by_token_symbol(data: List[Dict], symbol: str) -> List[Dict]:
    """Filter pools by token symbol"""
    symbol = symbol.upper()
    return [item for item in data if any(
        token.get("symbol", "").upper() == symbol 
        for token in item.get("tokens", [])
    )]

def filter_by_token_address(data: List[Dict], address: str) -> List[Dict]:
    """Filter pools by token address"""
    return [item for item in data if any(
        token.get("id", "").lower() == address.lower() 
        for token in item.get("tokens", [])
    )]

def top_n(data: List[Dict], field: str, n: int = 10) -> List[Dict]:
    """Get top N items by field"""
    return sorted(data, key=lambda x: x.get(field, 0) or 0, reverse=True)[:n]

def bottom_n(data: List[Dict], field: str, n: int = 10) -> List[Dict]:
    """Get bottom N items by field"""
    return sorted(data, key=lambda x: x.get(field, 0) or 0)[:n]

def sort_by_field(data: List[Dict], field: str, reverse: bool = True) -> List[Dict]:
    """Sort data by field"""
    return sorted(data, key=lambda x: x.get(field, 0) or 0, reverse=reverse)

def filter_recent_transactions(tx_data: List[Dict], hours: int = 24) -> List[Dict]:
    """Filter transactions by recency"""
    cutoff = datetime.now() - timedelta(hours=hours)
    return [tx for tx in tx_data if datetime.fromisoformat(tx.get("created_at", "").replace("Z", "+00:00")) > cutoff]

def filter_large_transactions(tx_data: List[Dict], min_usd: float) -> List[Dict]:
    """Filter transactions by minimum USD value"""
    return [tx for tx in tx_data if max(tx.get("price_0_usd", 0), tx.get("price_1_usd", 0)) >= min_usd]

def filter_by_timeframe(ohlcv_data: List[Dict], start_time: str, end_time: str = None) -> List[Dict]:
    """Filter OHLCV data by time range"""
    start_dt = datetime.fromisoformat(start_time.replace("Z", "+00:00"))
    if end_time:
        end_dt = datetime.fromisoformat(end_time.replace("Z", "+00:00"))
    else:
        end_dt = datetime.now()
    
    return [candle for candle in ohlcv_data 
            if start_dt <= datetime.fromisoformat(candle.get("time_open", "").replace("Z", "+00:00")) <= end_dt]

# ============================================================================
# ANALYSIS HELPERS (10 functions)
# ============================================================================

def calculate_price_change(current: float, previous: float) -> float:
    """Calculate percentage price change"""
    return ((current - previous) / previous * 100) if previous != 0 else 0

def calculate_volume_weighted_price(ohlcv_data: List[Dict]) -> float:
    """Calculate volume-weighted average price"""
    if not ohlcv_data:
        return 0
    
    total_volume = sum(candle.get("volume", 0) for candle in ohlcv_data)
    if total_volume == 0:
        return 0
    
    vwap = sum(candle.get("close", 0) * candle.get("volume", 0) for candle in ohlcv_data)
    return vwap / total_volume

def calculate_volatility(ohlcv_data: List[Dict]) -> float:
    """Calculate price volatility from OHLCV data"""
    if len(ohlcv_data) < 2:
        return 0
    
    returns = []
    for i in range(1, len(ohlcv_data)):
        prev_close = ohlcv_data[i-1].get("close", 0)
        curr_close = ohlcv_data[i].get("close", 0)
        if prev_close > 0:
            returns.append((curr_close - prev_close) / prev_close)
    
    if not returns:
        return 0
    
    mean_return = sum(returns) / len(returns)
    variance = sum((r - mean_return) ** 2 for r in returns) / len(returns)
    return (variance ** 0.5) * 100  # Return as percentage

def analyze_pool_activity(pool_data: Dict) -> Dict:
    """Analyze pool activity metrics"""
    metrics = {}
    for timeframe in ["24h", "6h", "1h", "30m", "15m", "5m"]:
        if timeframe in pool_data:
            metrics[timeframe] = {
                "volume_usd": pool_data[timeframe].get("volume_usd", 0),
                "txns": pool_data[timeframe].get("txns", 0),
                "price_change": pool_data[timeframe].get("last_price_usd_change", 0)
            }
    return metrics

def analyze_token_performance(token_data: Dict) -> Dict:
    """Analyze token performance across timeframes"""
    summary = extract_token_summary(token_data)
    if not summary:
        return {}
    
    performance = {}
    for timeframe in ["24h", "6h", "1h", "30m", "15m", "5m"]:
        if timeframe in summary:
            metrics = summary[timeframe]
            performance[timeframe] = {
                "volume_usd": metrics.get("volume_usd", 0),
                "price_change": metrics.get("last_price_usd_change", 0),
                "txns": metrics.get("txns", 0),
                "buy_ratio": metrics.get("buy_usd", 0) / max(metrics.get("volume_usd", 1), 1)
            }
    return performance

def detect_anomalies(data: List[Dict], field: str, threshold: float = 2.0) -> List[Dict]:
    """Detect anomalies using z-score method"""
    if len(data) < 3:
        return []
    
    values = [item.get(field, 0) for item in data]
    mean_val = sum(values) / len(values)
    variance = sum((v - mean_val) ** 2 for v in values) / len(values)
    std_dev = variance ** 0.5
    
    if std_dev == 0:
        return []
    
    anomalies = []
    for item in data:
        z_score = abs((item.get(field, 0) - mean_val) / std_dev)
        if z_score > threshold:
            anomalies.append(item)
    
    return anomalies

def calculate_correlation(data1: List[float], data2: List[float]) -> float:
    """Calculate correlation coefficient between two datasets"""
    if len(data1) != len(data2) or len(data1) < 2:
        return 0
    
    mean1 = sum(data1) / len(data1)
    mean2 = sum(data2) / len(data2)
    
    numerator = sum((x - mean1) * (y - mean2) for x, y in zip(data1, data2))
    denominator1 = sum((x - mean1) ** 2 for x in data1)
    denominator2 = sum((y - mean2) ** 2 for y in data2)
    
    if denominator1 == 0 or denominator2 == 0:
        return 0
    
    return numerator / (denominator1 * denominator2) ** 0.5

def analyze_liquidity_distribution(pools: List[Dict]) -> Dict:
    """Analyze liquidity distribution across pools"""
    if not pools:
        return {}
    
    volumes = [pool.get("volume_usd", 0) for pool in pools]
    total_volume = sum(volumes)
    
    if total_volume == 0:
        return {}
    
    # Calculate concentration metrics
    sorted_volumes = sorted(volumes, reverse=True)
    top_10_pct = sum(sorted_volumes[:max(1, len(sorted_volumes) // 10)]) / total_volume
    top_50_pct = sum(sorted_volumes[:max(1, len(sorted_volumes) // 2)]) / total_volume
    
    return {
        "total_volume": total_volume,
        "avg_volume": total_volume / len(pools),
        "top_10_percent_concentration": top_10_pct,
        "top_50_percent_concentration": top_50_pct,
        "gini_coefficient": calculate_gini_coefficient(volumes)
    }

def calculate_gini_coefficient(values: List[float]) -> float:
    """Calculate Gini coefficient for inequality measurement"""
    if not values or len(values) < 2:
        return 0
    
    sorted_values = sorted(values)
    n = len(sorted_values)
    cumsum = 0
    
    for i, value in enumerate(sorted_values):
        cumsum += (i + 1) * value
    
    return (2 * cumsum) / (n * sum(sorted_values)) - (n + 1) / n

def analyze_transaction_patterns(tx_data: List[Dict]) -> Dict:
    """Analyze transaction patterns"""
    if not tx_data:
        return {}
    
    buy_txs = [tx for tx in tx_data if tx.get("token_0_symbol") and tx.get("token_1_symbol")]
    sell_txs = [tx for tx in tx_data if tx.get("token_0_symbol") and tx.get("token_1_symbol")]
    
    total_buy_volume = sum(tx.get("price_0_usd", 0) for tx in buy_txs)
    total_sell_volume = sum(tx.get("price_1_usd", 0) for tx in sell_txs)
    
    return {
        "total_transactions": len(tx_data),
        "buy_transactions": len(buy_txs),
        "sell_transactions": len(sell_txs),
        "buy_volume_usd": total_buy_volume,
        "sell_volume_usd": total_sell_volume,
        "buy_sell_ratio": total_buy_volume / max(total_sell_volume, 1)
    }

# ============================================================================
# UTILITY HELPERS (8 functions)
# ============================================================================

def format_number(num: float, decimals: int = 2) -> str:
    """Format number with appropriate suffixes"""
    if num >= 1e9:
        return f"{num/1e9:.{decimals}f}B"
    elif num >= 1e6:
        return f"{num/1e6:.{decimals}f}M"
    elif num >= 1e3:
        return f"{num/1e3:.{decimals}f}K"
    else:
        return f"{num:.{decimals}f}"

def format_percentage(num: float, decimals: int = 2) -> str:
    """Format percentage with sign"""
    sign = "+" if num >= 0 else ""
    return f"{sign}{num:.{decimals}f}%"

def print_table(data: List[Dict], columns: List[str], title: str = "Data Table"):
    """Print data as a rich table"""
    table = Table(title=title)
    
    for col in columns:
        table.add_column(col, style="cyan")
    
    for item in data:
        row = [str(item.get(col, "")) for col in columns]
        table.add_row(*row)
    
    console.print(table)

def save_to_csv(data: List[Dict], filename: str, columns: List[str] = None):
    """Save data to CSV file"""
    if not data:
        return
    
    if columns is None:
        columns = list(data[0].keys())
    
    df = pd.DataFrame(data)
    df = df[columns]
    df.to_csv(filename, index=False)
    print(f"Data saved to {filename}")

def load_env(key: str, default: str = "") -> str:
    """Load environment variable with fallback"""
    return os.getenv(key, default)

def validate_network(network: str) -> bool:
    """Validate if network is supported"""
    networks = get_networks()
    return any(n.get("id") == network for n in networks)

def validate_token_address(address: str) -> bool:
    """Basic token address validation"""
    return len(address) >= 10 and address.isalnum()

def create_timestamp(days_ago: int = 0) -> str:
    """Create ISO timestamp for API requests"""
    dt = datetime.now() - timedelta(days=days_ago)
    return dt.strftime("%Y-%m-%d")

# ============================================================================
# ADVANCED HELPERS (5 functions)
# ============================================================================

def get_top_movers(network: str, limit: int = 10, min_volume: float = 1000000) -> List[Dict]:
    """Get top movers (biggest price changes) on a network"""
    pools = get_network_pools(network, limit=100)
    pool_list = extract_pools(pools)
    
    # Filter by minimum volume and sort by price change
    filtered = filter_by_volume(pool_list, min_volume)
    return top_n(filtered, "last_price_change_usd_24h", limit)

def get_high_volume_pools(network: str, limit: int = 10) -> List[Dict]:
    """Get highest volume pools on a network"""
    pools = get_network_pools(network, limit=100)
    pool_list = extract_pools(pools)
    return top_n(pool_list, "volume_usd", limit)

def get_token_liquidity_analysis(token_address: str, network: str = "ethereum") -> Dict:
    """Get comprehensive liquidity analysis for a token"""
    token_data = get_token_details(network, token_address)
    token_pools = get_token_pools(network, token_address)
    
    pools_list = extract_pools(token_pools)
    
    return {
        "token_info": extract_token_info(token_data),
        "total_liquidity": sum(pool.get("volume_usd", 0) for pool in pools_list),
        "pool_count": len(pools_list),
        "top_pools": top_n(pools_list, "volume_usd", 5),
        "dex_distribution": analyze_dex_distribution(pools_list)
    }

def analyze_dex_distribution(pools: List[Dict]) -> Dict:
    """Analyze distribution of pools across DEXes"""
    dex_counts = {}
    dex_volumes = {}
    
    for pool in pools:
        dex_name = pool.get("dex_name", "Unknown")
        volume = pool.get("volume_usd", 0)
        
        dex_counts[dex_name] = dex_counts.get(dex_name, 0) + 1
        dex_volumes[dex_name] = dex_volumes.get(dex_name, 0) + volume
    
    return {
        "counts": dex_counts,
        "volumes": dex_volumes,
        "total_dexes": len(dex_counts)
    }

def get_market_overview() -> Dict:
    """Get comprehensive market overview"""
    stats = get_system_stats()
    stats_data = extract_system_stats(stats)
    
    # Get top networks by activity
    networks = get_networks()
    network_overview = {}
    
    for network in networks[:5]:  # Top 5 networks
        network_id = network.get("id")
        try:
            pools = get_network_pools(network_id, limit=10)
            pool_list = extract_pools(pools)
            total_volume = sum(pool.get("volume_usd", 0) for pool in pool_list)
            network_overview[network_id] = {
                "display_name": network.get("display_name"),
                "total_volume": total_volume,
                "pool_count": len(pool_list)
            }
        except:
            continue
    
    return {
        "system_stats": stats_data,
        "network_overview": network_overview,
        "timestamp": datetime.now().isoformat()
    }

# ============================================================================
# ASYNC HELPERS (5 functions)
# ============================================================================

async def async_api_request(endpoint: str, params: Dict = None) -> Dict:
    """Async API request"""
    async with aiohttp.ClientSession() as session:
        url = f"{BASE_URL}{endpoint}"
        async with session.get(url, params=params) as response:
            return await response.json()

async def async_get_multiple_pools(networks: List[str], limit: int = 10) -> Dict:
    """Get pools from multiple networks concurrently"""
    tasks = [async_api_request(f"/networks/{network}/pools", {"limit": limit}) for network in networks]
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    return {network: result for network, result in zip(networks, results)}

async def async_get_token_data_batch(token_addresses: List[str], network: str = "ethereum") -> List[Dict]:
    """Get data for multiple tokens concurrently"""
    tasks = [async_api_request(f"/networks/{network}/tokens/{addr}") for addr in token_addresses]
    return await asyncio.gather(*tasks, return_exceptions=True)

async def async_monitor_prices(pool_addresses: List[str], network: str = "ethereum", interval: int = 60) -> None:
    """Monitor pool prices asynchronously"""
    while True:
        tasks = [async_api_request(f"/networks/{network}/pools/{addr}") for addr in pool_addresses]
        results = await asyncio.gather(*tasks, return_exceptions=True)
        
        for addr, result in zip(pool_addresses, results):
            if isinstance(result, dict) and "price_usd" in result:
                print(f"{addr}: ${result['price_usd']:.6f}")
        
        await asyncio.sleep(interval)

async def async_batch_search(queries: List[str]) -> List[Dict]:
    """Search for multiple queries concurrently"""
    tasks = [async_api_request("/search", {"query": query}) for query in queries]
    return await asyncio.gather(*tasks, return_exceptions=True)

# ============================================================================
# EXPORT ALL FUNCTIONS
# ============================================================================

__all__ = [
    # Core API Helpers
    "api_request", "get_networks", "get_network_pools", "get_dex_pools", 
    "get_network_dexes", "get_pool_details", "get_pool_ohlcv", 
    "get_pool_transactions", "get_token_details", "get_token_pools",
    "search_entities", "get_system_stats",
    
    # Data Extraction Helpers
    "extract_pools", "extract_tokens", "extract_dexes", "extract_transactions",
    "extract_ohlcv", "extract_page_info", "extract_token_summary", 
    "extract_time_metrics", "extract_pool_metrics", "extract_token_info",
    "extract_transaction_info", "extract_ohlcv_metrics", "extract_search_results",
    "extract_system_stats", "extract_pool_tokens",
    
    # Filtering & Sorting Helpers
    "filter_by_price_change", "filter_by_volume", "filter_by_network",
    "filter_by_dex", "filter_by_token_symbol", "filter_by_token_address",
    "top_n", "bottom_n", "sort_by_field", "filter_recent_transactions",
    "filter_large_transactions", "filter_by_timeframe",
    
    # Analysis Helpers
    "calculate_price_change", "calculate_volume_weighted_price", "calculate_volatility",
    "analyze_pool_activity", "analyze_token_performance", "detect_anomalies",
    "calculate_correlation", "analyze_liquidity_distribution", "calculate_gini_coefficient",
    "analyze_transaction_patterns",
    
    # Utility Helpers
    "format_number", "format_percentage", "print_table", "save_to_csv",
    "load_env", "validate_network", "validate_token_address", "create_timestamp",
    
    # Advanced Helpers
    "get_top_movers", "get_high_volume_pools", "get_token_liquidity_analysis",
    "analyze_dex_distribution", "get_market_overview",
    
    # Async Helpers
    "async_api_request", "async_get_multiple_pools", "async_get_token_data_batch",
    "async_monitor_prices", "async_batch_search"
] 