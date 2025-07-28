#!/usr/bin/env python3
"""
🐍 Paprika Helpers - Simple Python Example

This demonstrates the power of the 67 helper functions - 
creating a useful DeFi application in just a few lines!
"""

import asyncio
import sys
import os

# Add the Docs/internal directory to the path so we can import paprika_helpers
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'Docs', 'internal'))

from paprika_helpers import *

async def main():
    print("🐍 Paprika Helpers - Simple Example")
    print("=" * 40)

    # Get all supported blockchain networks
    networks = await get_networks()
    print(f"📡 Found {len(networks)} blockchain networks")

    # Get top Ethereum pools
    pools = await get_network_pools("ethereum", {"limit": 3})
    print(f"\n🏊 Top 3 Ethereum Pools:")
    
    for i, pool in enumerate(pools["pools"], 1):
        volume = format_number(pool["volume_usd"], 0)
        print(f"  {i}. {pool['dex_name']}: ${volume}")

    # Search for protocols
    search = await search_entities("uniswap")
    total = len(search["tokens"]) + len(search["pools"]) + len(search["dexes"])
    print(f"\n🔍 Found {total} Uniswap-related entities")

    # System statistics
    stats = await get_system_stats()
    print(f"\n📊 DeFi Ecosystem: {stats['chains']} chains, {stats['pools']} pools, {stats['tokens']} tokens")

    print("\n✅ That's it! Powerful DeFi data in just a few lines of Python! 🚀")

if __name__ == "__main__":
    asyncio.run(main()) 