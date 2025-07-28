# Paprika Helpers for Python 🐍

[![PyPI version](https://badge.fury.io/py/paprika-helpers.svg)](https://badge.fury.io/py/paprika-helpers)
[![Documentation](https://readthedocs.org/projects/paprika-helpers/badge/?version=latest)](https://paprika-helpers.readthedocs.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**67 powerful functions** for DeFi development. Get real-time blockchain data with just a few lines of Python! 

## 🚀 Quick Start

Install dependencies:
```bash
pip install aiohttp asyncio
```

## 💡 Create Something Amazing in 5 Minutes

```python
import asyncio
from paprika_helpers import *

async def main():
    # Get live data from 25+ blockchains
    networks = await get_networks()
    print(f"📡 {len(networks)} networks available")

    # Analyze top Ethereum pools
    pools = await get_network_pools("ethereum", {"limit": 5})
    
    # Find volume anomalies 
    anomalies = detect_anomalies(pools["pools"], "volume_usd", 2.0)
    print(f"🚨 {len(anomalies)} volume anomalies detected")

    # Search across the entire DeFi ecosystem
    search = await search_entities("uniswap")
    total = len(search["tokens"]) + len(search["pools"]) + len(search["dexes"])
    print(f"🔍 Found {total} Uniswap entities")

if __name__ == "__main__":
    asyncio.run(main())
```

## ✨ What You Can Build

- **🔥 DeFi Portfolio Trackers** - Real-time value monitoring
- **📊 Market Analytics Dashboards** - Volume, price, liquidity analysis  
- **🚨 Anomaly Detection Systems** - Identify unusual market activity
- **🤖 Trading Bots** - Automated strategies with live data
- **📈 Price Prediction Models** - ML with historical OHLCV data
- **💎 Yield Farming Optimizers** - Find best liquidity opportunities

## 🎯 Key Features

- **🌐 25+ Blockchains** - Ethereum, Solana, BSC, Polygon, and more
- **⚡ Ultra Fast** - Async/await with aiohttp
- **🔒 Type Safe** - Full type hints and validation
- **📊 Rich Analytics** - Volume, liquidity, anomaly detection
- **⚙️ Easy Integration** - Just import and use
- **📝 Complete Documentation** - Every function documented with examples

## 🏃‍♂️ Run the Example

```bash
git clone <repo>
cd python-paprika-helpers
python3 example.py
```

## 📚 Function Categories

| Category | Functions | Description |
|----------|-----------|-------------|
| **🔗 Core API** | 12 | Direct blockchain data access |
| **📊 Data Extraction** | 15 | Parse and structure responses |
| **🔍 Filtering & Sorting** | 12 | Find exactly what you need |
| **📈 Analytics** | 10 | Advanced calculations and insights |
| **🛠️ Utilities** | 8 | Formatting, validation, I/O |
| **🚀 Advanced** | 5 | High-level market analysis |
| **⚡ Async** | 5 | Concurrent, high-performance operations |

## 🔗 More Resources

- [📖 Full Documentation](https://paprika-helpers.readthedocs.io)
- [🎯 More Examples](./example.py)
- [🏗️ API Reference](https://api.dexpaprika.com)
- [💬 Community Discord](https://discord.gg/paprika)

## 📄 License

MIT License - build amazing things! 🚀

---

**⚡ Ready to build the future of DeFi? Start coding now! 🐍** 