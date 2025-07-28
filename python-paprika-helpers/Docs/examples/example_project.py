#!/usr/bin/env python3
"""
🔥 Multi-Network Market Scanner
A ≤50-line utility that scans multiple networks for top movers, 
high volume pools, and market anomalies. Built with the complete 
Paprika Vibe-Code Helper Library.

Features:
- Cross-network analysis
- Anomaly detection
- Market overview
- Real-time alerts
"""

from paprika_helpers import *
import time

def main():
    # Configuration
    networks = ["ethereum", "solana", "polygon"]
    min_change = 15.0  # Alert on 15%+ moves
    min_volume = 5000000  # $5M+ volume
    
    print("🚀 Multi-Network Market Scanner Starting...")
    print("=" * 50)
    
    # Get market overview
    overview = get_market_overview()
    print(f"🌐 Total Pools: {overview['system_stats']['pools']:,}")
    print(f"🏗️  Total DEXes: {overview['system_stats']['factories']:,}")
    
    while True:
        for network in networks:
            # Get top movers and high volume pools
            top_movers = get_top_movers(network, limit=5, min_volume=min_volume)
            high_volume = get_high_volume_pools(network, limit=5)
            
            # Detect anomalies
            pools = get_network_pools(network, limit=50)
            pool_list = extract_pools(pools)
            anomalies = detect_anomalies(pool_list, "volume_usd", threshold=2.5)
            
            print(f"\n🔥 {network.upper()} - Top Movers:")
            for pool in top_movers:
                change = format_percentage(pool['last_price_change_usd_24h'])
                volume = format_number(pool['volume_usd'])
                print(f"  {pool['dex_name']}: {change} | {volume} volume")
            
            if anomalies:
                print(f"🚨 {network.upper()} - Anomalies Detected:")
                for anomaly in anomalies[:3]:
                    volume = format_number(anomaly['volume_usd'])
                    print(f"  {anomaly['dex_name']}: {volume} volume (anomaly)")
        
        print("\n" + "=" * 50)
        time.sleep(300)  # Check every 5 minutes

if __name__ == "__main__":
    main() 