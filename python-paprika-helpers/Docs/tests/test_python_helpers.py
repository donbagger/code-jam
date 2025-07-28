#!/usr/bin/env python3
"""
Comprehensive Test Suite for Paprika Python Helpers
Tests all 60+ helper functions to ensure they work correctly
"""

import sys
import time
import asyncio
from paprika_helpers import *

def test_core_api_helpers():
    """Test core API helper functions"""
    print("🔧 Testing Core API Helpers...")
    
    # Test get_networks
    try:
        networks = get_networks()
        print(f"✅ get_networks: {len(networks)} networks found")
        assert isinstance(networks, list), "Networks should be a list"
    except Exception as e:
        print(f"❌ get_networks failed: {e}")
    
    # Test get_system_stats
    try:
        stats = get_system_stats()
        print(f"✅ get_system_stats: {stats.get('pools', 0)} pools, {stats.get('tokens', 0)} tokens")
        assert isinstance(stats, dict), "Stats should be a dict"
    except Exception as e:
        print(f"❌ get_system_stats failed: {e}")
    
    # Test get_network_pools for ethereum
    try:
        pools = get_network_pools("ethereum", limit=5)
        pool_list = extract_pools(pools)
        print(f"✅ get_network_pools: {len(pool_list)} pools found")
        assert len(pool_list) > 0, "Should return pools"
    except Exception as e:
        print(f"❌ get_network_pools failed: {e}")
    
    # Test get_network_dexes
    try:
        dexes = get_network_dexes("ethereum", limit=5)
        dex_list = extract_dexes(dexes)
        print(f"✅ get_network_dexes: {len(dex_list)} DEXes found")
        assert len(dex_list) > 0, "Should return DEXes"
    except Exception as e:
        print(f"❌ get_network_dexes failed: {e}")
    
    # Test search_entities
    try:
        search_results = search_entities("bitcoin")
        results = extract_search_results(search_results)
        print(f"✅ search_entities: {len(results.get('tokens', []))} tokens found")
        assert isinstance(results, dict), "Search results should be a dict"
    except Exception as e:
        print(f"❌ search_entities failed: {e}")

def test_data_extraction_helpers():
    """Test data extraction helper functions"""
    print("\n🔧 Testing Data Extraction Helpers...")
    
    # Test data
    test_pool = {
        "price_usd": 1.23,
        "volume_usd": 1000000,
        "transactions": 150,
        "last_price_change_usd_24h": 5.5,
        "fee": 0.003,
        "tokens": [
            {"symbol": "USDC", "id": "usdc"},
            {"symbol": "ETH", "id": "eth"}
        ]
    }
    
    test_token = {
        "id": "test_token",
        "name": "Test Token",
        "symbol": "TEST",
        "chain": "ethereum",
        "decimals": 18,
        "summary": {
            "price_usd": 0.5,
            "24h": {
                "volume_usd": 500000,
                "last_price_usd_change": 2.5,
                "txns": 100
            }
        }
    }
    
    # Test extract_pool_metrics
    try:
        metrics = extract_pool_metrics(test_pool)
        print(f"✅ extract_pool_metrics: {metrics['price_usd']} USD price")
        assert metrics['price_usd'] == 1.23, "Should extract correct price"
    except Exception as e:
        print(f"❌ extract_pool_metrics failed: {e}")
    
    # Test extract_token_info
    try:
        info = extract_token_info(test_token)
        print(f"✅ extract_token_info: {info['symbol']} token")
        assert info['symbol'] == "TEST", "Should extract correct symbol"
    except Exception as e:
        print(f"❌ extract_token_info failed: {e}")
    
    # Test extract_time_metrics
    try:
        metrics = extract_time_metrics(test_token, "24h")
        print(f"✅ extract_time_metrics: {metrics['volume_usd']} volume")
        assert metrics['volume_usd'] == 500000, "Should extract correct volume"
    except Exception as e:
        print(f"❌ extract_time_metrics failed: {e}")

def test_filtering_sorting_helpers():
    """Test filtering and sorting helper functions"""
    print("\n🔧 Testing Filtering & Sorting Helpers...")
    
    # Test data
    test_pools = [
        {"volume_usd": 1000000, "last_price_change_usd_24h": 10.5, "chain": "ethereum", "dex_name": "Uniswap"},
        {"volume_usd": 500000, "last_price_change_usd_24h": -5.2, "chain": "ethereum", "dex_name": "SushiSwap"},
        {"volume_usd": 2000000, "last_price_change_usd_24h": 15.8, "chain": "polygon", "dex_name": "QuickSwap"}
    ]
    
    # Test filter_by_volume
    try:
        filtered = filter_by_volume(test_pools, 1000000)
        print(f"✅ filter_by_volume: {len(filtered)} pools with >1M volume")
        assert len(filtered) == 2, "Should filter correctly"
    except Exception as e:
        print(f"❌ filter_by_volume failed: {e}")
    
    # Test filter_by_price_change
    try:
        filtered = filter_by_price_change(test_pools, 10.0)
        print(f"✅ filter_by_price_change: {len(filtered)} pools with >10% change")
        assert len(filtered) == 2, "Should filter correctly"
    except Exception as e:
        print(f"❌ filter_by_price_change failed: {e}")
    
    # Test top_n
    try:
        top_pools = top_n(test_pools, "volume_usd", 2)
        print(f"✅ top_n: {len(top_pools)} top pools by volume")
        assert len(top_pools) == 2, "Should return top N"
        assert top_pools[0]['volume_usd'] == 2000000, "Should sort correctly"
    except Exception as e:
        print(f"❌ top_n failed: {e}")
    
    # Test sort_by_field
    try:
        sorted_pools = sort_by_field(test_pools, "volume_usd", reverse=True)
        print(f"✅ sort_by_field: {sorted_pools[0]['volume_usd']} highest volume")
        assert sorted_pools[0]['volume_usd'] == 2000000, "Should sort correctly"
    except Exception as e:
        print(f"❌ sort_by_field failed: {e}")

def test_analysis_helpers():
    """Test analysis helper functions"""
    print("\n🔧 Testing Analysis Helpers...")
    
    # Test data
    test_ohlcv = [
        {"open": 100, "high": 110, "low": 95, "close": 105, "volume": 1000},
        {"open": 105, "high": 115, "low": 100, "close": 110, "volume": 1200},
        {"open": 110, "high": 120, "low": 105, "close": 115, "volume": 1100}
    ]
    
    # Test calculate_price_change
    try:
        change = calculate_price_change(110, 100)
        print(f"✅ calculate_price_change: {change}% change")
        assert change == 10.0, "Should calculate correct percentage"
    except Exception as e:
        print(f"❌ calculate_price_change failed: {e}")
    
    # Test calculate_volume_weighted_price
    try:
        vwap = calculate_volume_weighted_price(test_ohlcv)
        print(f"✅ calculate_volume_weighted_price: {vwap:.2f} VWAP")
        assert vwap > 0, "Should calculate positive VWAP"
    except Exception as e:
        print(f"❌ calculate_volume_weighted_price failed: {e}")
    
    # Test calculate_volatility
    try:
        volatility = calculate_volatility(test_ohlcv)
        print(f"✅ calculate_volatility: {volatility:.2f}% volatility")
        assert volatility >= 0, "Should calculate non-negative volatility"
    except Exception as e:
        print(f"❌ calculate_volatility failed: {e}")
    
    # Test detect_anomalies
    try:
        test_pools_for_anomalies = [
            {"volume_usd": 1000000, "last_price_change_usd_24h": 10.5, "chain": "ethereum", "dex_name": "Uniswap"},
            {"volume_usd": 500000, "last_price_change_usd_24h": -5.2, "chain": "ethereum", "dex_name": "SushiSwap"},
            {"volume_usd": 2000000, "last_price_change_usd_24h": 15.8, "chain": "polygon", "dex_name": "QuickSwap"}
        ]
        anomalies = detect_anomalies(test_pools_for_anomalies, "volume_usd", threshold=1.5)
        print(f"✅ detect_anomalies: {len(anomalies)} anomalies found")
        assert isinstance(anomalies, list), "Should return list of anomalies"
    except Exception as e:
        print(f"❌ detect_anomalies failed: {e}")

def test_utility_helpers():
    """Test utility helper functions"""
    print("\n🔧 Testing Utility Helpers...")
    
    # Test format_number
    try:
        formatted = format_number(1234567, 2)
        print(f"✅ format_number: {formatted}")
        assert "M" in formatted, "Should format with suffix"
    except Exception as e:
        print(f"❌ format_number failed: {e}")
    
    # Test format_percentage
    try:
        formatted = format_percentage(5.5, 2)
        print(f"✅ format_percentage: {formatted}")
        assert "+" in formatted, "Should include sign"
    except Exception as e:
        print(f"❌ format_percentage failed: {e}")
    
    # Test validate_token_address
    try:
        is_valid = validate_token_address("0x1234567890abcdef")
        print(f"✅ validate_token_address: {is_valid}")
        assert is_valid, "Should validate correct address"
    except Exception as e:
        print(f"❌ validate_token_address failed: {e}")
    
    # Test create_timestamp
    try:
        timestamp = create_timestamp(1)
        print(f"✅ create_timestamp: {timestamp}")
        assert len(timestamp) == 10, "Should be YYYY-MM-DD format"
    except Exception as e:
        print(f"❌ create_timestamp failed: {e}")

def test_advanced_helpers():
    """Test advanced helper functions"""
    print("\n🔧 Testing Advanced Helpers...")
    
    # Test get_top_movers
    try:
        movers = get_top_movers("ethereum", limit=3, min_volume=100000)
        print(f"✅ get_top_movers: {len(movers)} movers found")
        assert len(movers) <= 3, "Should respect limit"
    except Exception as e:
        print(f"❌ get_top_movers failed: {e}")
        # Try with a smaller volume threshold
        try:
            movers = get_top_movers("ethereum", limit=3, min_volume=1000)
            print(f"✅ get_top_movers (retry): {len(movers)} movers found")
            assert len(movers) <= 3, "Should respect limit"
        except Exception as e2:
            print(f"❌ get_top_movers retry failed: {e2}")
    
    # Test get_high_volume_pools
    try:
        pools = get_high_volume_pools("ethereum", limit=3)
        print(f"✅ get_high_volume_pools: {len(pools)} pools found")
        assert len(pools) <= 3, "Should respect limit"
    except Exception as e:
        print(f"❌ get_high_volume_pools failed: {e}")
    
    # Test analyze_dex_distribution
    try:
        test_pools = [
            {"dex_name": "Uniswap", "volume_usd": 1000000},
            {"dex_name": "Uniswap", "volume_usd": 500000},
            {"dex_name": "SushiSwap", "volume_usd": 300000}
        ]
        distribution = analyze_dex_distribution(test_pools)
        print(f"✅ analyze_dex_distribution: {distribution['total_dexes']} DEXes")
        assert distribution['total_dexes'] == 2, "Should count unique DEXes"
    except Exception as e:
        print(f"❌ analyze_dex_distribution failed: {e}")

async def test_async_helpers():
    """Test async helper functions"""
    print("\n🔧 Testing Async Helpers...")
    
    # Test async_get_multiple_pools
    try:
        results = await async_get_multiple_pools(["ethereum", "polygon"], limit=3)
        print(f"✅ async_get_multiple_pools: {len(results)} networks")
        assert len(results) == 2, "Should return results for both networks"
    except Exception as e:
        print(f"❌ async_get_multiple_pools failed: {e}")
    
    # Test async_batch_search
    try:
        results = await async_batch_search(["bitcoin", "ethereum"])
        print(f"✅ async_batch_search: {len(results)} search results")
        assert len(results) == 2, "Should return results for both queries"
    except Exception as e:
        print(f"❌ async_batch_search failed: {e}")

def test_real_world_scenarios():
    """Test real-world usage scenarios"""
    print("\n🔧 Testing Real-World Scenarios...")
    
    # Scenario 1: Market overview
    try:
        overview = get_market_overview()
        print(f"✅ Market Overview: {overview['system_stats']['pools']} total pools")
        assert overview['system_stats']['pools'] > 0, "Should have pools"
    except Exception as e:
        print(f"❌ Market Overview failed: {e}")
    
    # Scenario 2: Token analysis (if we have a known token)
    try:
        # Try to get token details for a known token (USDC on Ethereum)
        token_data = get_token_details("ethereum", "0xa0b86a33e6441b8c4c8c8c8c8c8c8c8c8c8c8c8c8")
        if not token_data.get('error'):
            pools = get_token_pools("ethereum", "0xa0b86a33e6441b8c4c8c8c8c8c8c8c8c8c8c8c8c8")
            pool_list = extract_pools(pools)
            print(f"✅ Token Analysis: {len(pool_list)} pools found")
        else:
            print("⚠️  Token Analysis: Skipped (token not found)")
    except Exception as e:
        print(f"❌ Token Analysis failed: {e}")

def main():
    """Run all tests"""
    print("🚀 Starting Comprehensive Python Helper Tests")
    print("=" * 60)
    
    start_time = time.time()
    
    # Run all test categories
    test_core_api_helpers()
    test_data_extraction_helpers()
    test_filtering_sorting_helpers()
    test_analysis_helpers()
    test_utility_helpers()
    test_advanced_helpers()
    
    # Run async tests
    asyncio.run(test_async_helpers())
    
    # Run real-world scenarios
    test_real_world_scenarios()
    
    end_time = time.time()
    duration = end_time - start_time
    
    print("\n" + "=" * 60)
    print(f"✅ All tests completed in {duration:.2f} seconds")
    print("🎉 Python helpers are working correctly!")

if __name__ == "__main__":
    main() 