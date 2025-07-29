# Paprika Helpers - Multi-Language DeFi SDK 🚀

[![Crypto Market Data by CoinPaprika](https://coinpaprika.com/badge.svg?text=Crypto+Market+Data+by+CoinPaprika)](https://coinpaprika.com/vibe-code?ref=vibecode-0825)
[![DexPaprika MCP Server](https://coinpaprika.com/badge.svg?text=DexPaprika+MCP+Server)](https://mcp.dexpaprika.com?ref=vibecode-0825)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/)
[![JavaScript](https://img.shields.io/badge/JavaScript-Node.js-yellow.svg)](https://nodejs.org/)
[![Go](https://img.shields.io/badge/Go-1.19+-00ADD8.svg)](https://golang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

**What it is**: ≤50‑line DeFi utilities using CoinPaprika data.  
**How it works**: Choose your language, build your project, and submit to Vibe-Code contest!

**67 powerful functions** across **4 languages** for DeFi development. Get real-time blockchain data from **25+ networks** with just a few lines of code!

## 🎯 **Choose Your Language**

| Language | Best For | Quick Start |
|----------|----------|-------------|
| 🐍 **[Python](./python-paprika-helpers/)** | Data Science & ML | `python3 example.py` |
| 🟨 **[JavaScript](./javascript-paprika-helpers/)** | Web Apps & APIs | `node example.js` |
| 🔵 **[Go](./go-paprika-helpers/)** | High Performance | `go run example.go` |
| 🦀 **[Rust](./rust-paprika-helpers/)** | Systems & Speed | `cargo run --bin example` |

## ⚡ **One-Line Install**

```bash
# Python
pip install aiohttp && cd python-paprika-helpers && python3 example.py

# JavaScript  
npm install axios && cd javascript-paprika-helpers && node example.js

# Go
cd go-paprika-helpers && go run example.go

# Rust
cd rust-paprika-helpers && cargo run --bin example
```

## 💡 **5-Minute Demo**

### Python 🐍
```python
from paprika_helpers import *

# Get live data from 25+ blockchains with built-in error handling
networks = await get_networks()
pools = await get_network_pools("ethereum", {"limit": 5})

# Safe data processing with validation
clean_pools = clean_pool_data(pools["pools"])
anomalies = detect_anomalies(clean_pools, "volume_usd", 2.0)
print(f"🚨 {len(anomalies)} volume anomalies detected")
```

### JavaScript 🟨
```javascript
const { get_networks, get_network_pools, detect_anomalies, cleanPoolData } = require('./paprika_helpers');

// Get live data from 25+ blockchains with retry logic
const networks = await get_networks();
const pools = await get_network_pools("ethereum", { limit: 5 });

// Safe data processing with validation
const cleanPools = cleanPoolData(pools.pools);
const anomalies = detect_anomalies(cleanPools, "volume_usd", 2.0);
console.log(`🚨 ${anomalies.length} volume anomalies detected`);
```

### Go 🔵
```go
import paprika "github.com/paprika/go-helpers"

// Get live data from 25+ blockchains with retry mechanism
networks, _ := paprika.GetNetworks()
pools, _ := paprika.GetNetworkPools("ethereum", &paprika.ApiParams{Limit: 5})

// Safe data processing with validation
cleanPools := paprika.CleanPoolData(pools.Pools)
anomalies := paprika.DetectAnomalies(cleanPools, "volume_usd", 2.0)
fmt.Printf("🚨 %d volume anomalies detected\n", len(anomalies))
```

### Rust 🦀
```rust
use paprika_helpers::*;

// Get live data from 25+ blockchains with robust error handling
let networks = get_networks().await?;
let pools = get_network_pools("ethereum", Some(ApiParams::new().limit(5))).await?;

// Safe data processing with validation
let clean_pools = clean_pool_data(&pools.pools);
let anomalies = detect_anomalies(&clean_pools, "volume_usd", 2.0);
println!("🚨 {} volume anomalies detected", anomalies.len());
```

## ✨ **What You Can Build**

- **🔥 DeFi Portfolio Trackers** - Real-time value monitoring across all chains
- **📊 Market Analytics Dashboards** - Volume, price, liquidity analysis  
- **🚨 Anomaly Detection Systems** - Identify unusual market activity
- **🤖 Trading Bots** - Automated strategies with live data
- **📈 Price Prediction Models** - ML with historical OHLCV data
- **💎 Yield Farming Optimizers** - Find best liquidity opportunities

## 🎯 **Key Features**

- **🌐 25+ Blockchains** - Ethereum, Solana, BSC, Polygon, Arbitrum, and more
- **⚡ Ultra Fast** - Async/concurrent operations in all languages
- **🔒 Type Safe** - Full type safety and validation
- **🛡️ Production Ready** - Robust error handling, retry logic, data validation
- **📊 Rich Analytics** - Volume, liquidity, anomaly detection built-in
- **⚙️ Easy Integration** - One dependency per language
- **📝 Complete Documentation** - Every function documented with examples
- **🎭 100% Parity** - Identical 67 functions across all languages

## 🛡️ **Production-Grade Quality**

**Enterprise-Ready Features:**
- **Retry Logic** - Automatic retries with exponential backoff for failed API calls
- **Data Validation** - Comprehensive price and volume validation with bounds checking
- **Error Recovery** - Graceful handling of invalid JSON, network timeouts, and API errors
- **Symbol Extraction** - Smart token symbol resolution with multiple fallback strategies
- **Bounds Checking** - Percentage calculations capped at realistic limits (±10,000%)
- **Cache Management** - Intelligent caching with corruption recovery

## 📚 **Function Categories (67+ Total)**

| Category | Functions | Description |
|----------|-----------|-------------|
| **🔗 Core API** | 12 | Direct blockchain data access with retry logic |
| **📊 Data Extraction** | 15 | Parse and structure responses with validation |
| **🔍 Filtering & Sorting** | 12 | Find exactly what you need with bounds checking |
| **📈 Analytics** | 10 | Advanced calculations and insights with safety |
| **🛠️ Utilities** | 8 | Formatting, validation, I/O operations |
| **🚀 Advanced** | 5 | High-level market analysis |
| **⚡ Async** | 5 | Concurrent, high-performance operations |
| **🛡️ Validation** | 5+ | **NEW** - Data safety and error prevention helpers |

## 🚀 **Quick Start for Developers**

### **1. Fork & Clone**
```bash
git clone https://github.com/your-username/Paprika-Vibe-Jam.git
cd Paprika-Vibe-Jam
```

### **2. Choose Your Language & Test**
```bash
# Python 🐍
cd python-paprika-helpers && python3 example.py

# JavaScript 🟨  
cd javascript-paprika-helpers && node example.js

# Go 🔵
cd go-paprika-helpers && go run example.go

# Rust 🦀
cd rust-paprika-helpers && cargo run --bin example
```

### **3. Build Your Project**
All examples demonstrate production-ready patterns with proper error handling!

## ⚠️ **Important: Data Quality & Best Practices**

### **🚨 Common Pitfalls (FIXED in v2.0)**

**Previous versions had data quality issues that are now resolved:**

❌ **Before**: Token symbols returned `undefined/undefined`  
✅ **Now**: Smart symbol extraction with multiple fallbacks

❌ **Before**: Unrealistic profit percentages (e.g., `+63516157.55%`)  
✅ **Now**: Percentage calculations capped at ±10,000%

❌ **Before**: API failures with "Invalid JSON response"  
✅ **Now**: Retry logic with exponential backoff

❌ **Before**: Zero prices causing calculation errors  
✅ **Now**: Comprehensive price validation

### **🛡️ Use Safe Helper Functions**

**All languages now include data validation helpers:**

```python
# Python - Use these for safe data processing
clean_pools = clean_pool_data(raw_pools)  # Validates all fields
safe_change = calculate_safe_percentage(current, previous)  # Bounds checking
valid_price = validate_price(raw_price)  # Ensures positive values
```

```javascript
// JavaScript - Use these for safe data processing
const cleanPools = cleanPoolData(rawPools);  // Validates all fields  
const safeChange = calculateSafePercentage(current, previous);  // Bounds checking
const validPrice = validatePrice(rawPrice);  // Ensures positive values
```

```go
// Go - Use these for safe data processing
cleanPools := paprika.CleanPoolData(rawPools)  // Validates all fields
safeChange := paprika.CalculateSafePercentage(current, previous, 10000.0)  // Bounds checking
validPrice := paprika.ValidatePrice(rawPrice)  // Ensures positive values
```

```rust
// Rust - Use these for safe data processing  
let clean_pools = clean_pool_data(&raw_pools);  // Validates all fields
let safe_change = calculate_safe_percentage(current, previous, 10000.0);  // Bounds checking
let valid_price = validate_price(raw_price);  // Ensures positive values
```

### **📋 Developer Checklist**

**Before deploying your DeFi application:**

- [ ] **Use cleaned data**: Always run `clean_pool_data()` before analysis
- [ ] **Validate prices**: Check for positive, finite values before calculations  
- [ ] **Handle missing symbols**: Use safe extraction functions for token symbols
- [ ] **Bound percentages**: Use safe calculation functions to prevent extreme values
- [ ] **Test error scenarios**: Verify your app handles API failures gracefully
- [ ] **Implement timeouts**: Set reasonable timeouts for production use

## 🚀 **Quick Navigation**

### **🐍 Python Developers**
→ **[Python Helpers](./python-paprika-helpers/)** - Perfect for data science, ML, and rapid prototyping

### **🟨 JavaScript Developers**  
→ **[JavaScript Helpers](./javascript-paprika-helpers/)** - Ideal for web apps, APIs, and Node.js backends

### **🔵 Go Developers**
→ **[Go Helpers](./go-paprika-helpers/)** - Optimized for high-performance services and enterprise

### **🦀 Rust Developers**
→ **[Rust Helpers](./rust-paprika-helpers/)** - Maximum performance with memory safety

## 🔗 **Resources**

- [🏗️ API Reference](https://api.dexpaprika.com)
- [💬 Community Discord](https://discord.gg/paprika)
- [📖 Full Documentation](https://docs.paprika-helpers.dev)
- [🎯 Live Examples](https://examples.paprika-helpers.dev)

## 📄 **License**

MIT License - build amazing things! 🚀

---

**⚡ Ready to build the future of DeFi? Pick your language and start coding! 🌟** 