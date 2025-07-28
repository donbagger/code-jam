// Package paprikahelpers provides a comprehensive collection of 67 helper functions
// for DexPaprika API integration covering all endpoints and data structures
package paprikahelpers

import (
	"context"
	"crypto/md5"
	"encoding/csv"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"net/http"
	"net/url"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/olekukonko/tablewriter"
)

const (
	// BaseURL is the DexPaprika API base URL
	BaseURL = "https://api.dexpaprika.com"

	// CacheDuration is the default cache duration
	CacheDuration = 5 * time.Minute

	// DefaultTimeout is the default HTTP timeout
	DefaultTimeout = 10 * time.Second
)

var (
	// httpClient is the shared HTTP client
	httpClient = &http.Client{
		Timeout: DefaultTimeout,
	}

	// cache stores API responses
	cache    = make(map[string]CacheEntry)
	cacheMux sync.RWMutex

	// cacheDir stores the cache directory path
	cacheDir = ".cache"
)

// init initializes the cache directory
func init() {
	os.MkdirAll(cacheDir, 0755)
}

// ============================================================================
// CORE API HELPERS (12 functions)
// ============================================================================

// APIRequest makes an HTTP request to the DexPaprika API with caching and error handling
func APIRequest(endpoint string, params map[string]string) (interface{}, error) {
	// Create cache key
	cacheKey := createCacheKey(endpoint, params)

	// Check cache
	cacheMux.RLock()
	if entry, exists := cache[cacheKey]; exists && time.Since(entry.Timestamp) < CacheDuration {
		cacheMux.RUnlock()
		return entry.Data, nil
	}
	cacheMux.RUnlock()

	// Build URL
	u, err := url.Parse(BaseURL + endpoint)
	if err != nil {
		return nil, fmt.Errorf("invalid endpoint: %w", err)
	}

	// Add query parameters
	q := u.Query()
	for key, value := range params {
		q.Add(key, value)
	}
	u.RawQuery = q.Encode()

	// Make request
	resp, err := httpClient.Get(u.String())
	if err != nil {
		return nil, fmt.Errorf("API request failed: %w", err)
	}
	defer resp.Body.Close()

	// Read response
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	// Check for API errors
	if resp.StatusCode >= 400 {
		var apiErr APIError
		if json.Unmarshal(body, &apiErr) == nil {
			return nil, fmt.Errorf("API error: %s", apiErr.Error)
		}
		return nil, fmt.Errorf("API error: %s", string(body))
	}

	// Parse JSON
	var result interface{}
	if err := json.Unmarshal(body, &result); err != nil {
		return nil, fmt.Errorf("failed to parse JSON: %w", err)
	}

	// Store in cache
	cacheMux.Lock()
	cache[cacheKey] = CacheEntry{
		Data:      result,
		Timestamp: time.Now(),
	}
	cacheMux.Unlock()

	return result, nil
}

// GetNetworks retrieves all supported blockchain networks
func GetNetworks() ([]Network, error) {
	data, err := APIRequest("/networks", nil)
	if err != nil {
		return nil, err
	}

	var networks []Network
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &networks); err != nil {
		return nil, fmt.Errorf("failed to parse networks: %w", err)
	}

	return networks, nil
}

// GetNetworkPools retrieves pools for a specific network
func GetNetworkPools(network string, params map[string]string) (*PoolsResponse, error) {
	if params == nil {
		params = make(map[string]string)
	}

	data, err := APIRequest(fmt.Sprintf("/networks/%s/pools", network), params)
	if err != nil {
		return nil, err
	}

	var response PoolsResponse
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &response); err != nil {
		return nil, fmt.Errorf("failed to parse pools response: %w", err)
	}

	return &response, nil
}

// GetDexPools retrieves pools for a specific DEX on a network
func GetDexPools(network, dex string, params map[string]string) (*PoolsResponse, error) {
	if params == nil {
		params = make(map[string]string)
	}

	data, err := APIRequest(fmt.Sprintf("/networks/%s/dexes/%s/pools", network, dex), params)
	if err != nil {
		return nil, err
	}

	var response PoolsResponse
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &response); err != nil {
		return nil, fmt.Errorf("failed to parse dex pools response: %w", err)
	}

	return &response, nil
}

// GetNetworkDexes retrieves DEXes available on a network
func GetNetworkDexes(network string, params map[string]string) (*DexesResponse, error) {
	if params == nil {
		params = make(map[string]string)
	}

	data, err := APIRequest(fmt.Sprintf("/networks/%s/dexes", network), params)
	if err != nil {
		return nil, err
	}

	var response DexesResponse
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &response); err != nil {
		return nil, fmt.Errorf("failed to parse dexes response: %w", err)
	}

	return &response, nil
}

// GetPoolDetails retrieves detailed information about a specific pool
func GetPoolDetails(network, poolAddress string, inversed bool) (*Pool, error) {
	params := map[string]string{}
	if inversed {
		params["inversed"] = "true"
	}

	data, err := APIRequest(fmt.Sprintf("/networks/%s/pools/%s", network, poolAddress), params)
	if err != nil {
		return nil, err
	}

	var pool Pool
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &pool); err != nil {
		return nil, fmt.Errorf("failed to parse pool details: %w", err)
	}

	return &pool, nil
}

// GetPoolOHLCV retrieves OHLCV data for a pool
func GetPoolOHLCV(network, poolAddress, start string, params map[string]string) ([]OHLCVRecord, error) {
	if params == nil {
		params = make(map[string]string)
	}
	params["start"] = start

	data, err := APIRequest(fmt.Sprintf("/networks/%s/pools/%s/ohlcv", network, poolAddress), params)
	if err != nil {
		return nil, err
	}

	var records []OHLCVRecord
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &records); err != nil {
		return nil, fmt.Errorf("failed to parse OHLCV data: %w", err)
	}

	return records, nil
}

// GetPoolTransactions retrieves transactions for a specific pool
func GetPoolTransactions(network, poolAddress string, params map[string]string) (*TransactionsResponse, error) {
	if params == nil {
		params = make(map[string]string)
	}

	data, err := APIRequest(fmt.Sprintf("/networks/%s/pools/%s/transactions", network, poolAddress), params)
	if err != nil {
		return nil, err
	}

	var response TransactionsResponse
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &response); err != nil {
		return nil, fmt.Errorf("failed to parse transactions response: %w", err)
	}

	return &response, nil
}

// GetTokenDetails retrieves detailed information about a specific token
func GetTokenDetails(network, tokenAddress string) (*Token, error) {
	data, err := APIRequest(fmt.Sprintf("/networks/%s/tokens/%s", network, tokenAddress), nil)
	if err != nil {
		return nil, err
	}

	var token Token
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &token); err != nil {
		return nil, fmt.Errorf("failed to parse token details: %w", err)
	}

	return &token, nil
}

// GetTokenPools retrieves pools containing a specific token
func GetTokenPools(network, tokenAddress string, params map[string]string) (*TokenPoolsResponse, error) {
	if params == nil {
		params = make(map[string]string)
	}

	data, err := APIRequest(fmt.Sprintf("/networks/%s/tokens/%s/pools", network, tokenAddress), params)
	if err != nil {
		return nil, err
	}

	var response TokenPoolsResponse
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &response); err != nil {
		return nil, fmt.Errorf("failed to parse token pools response: %w", err)
	}

	return &response, nil
}

// SearchEntities searches across tokens, pools, and DEXes
func SearchEntities(query string) (*SearchResponse, error) {
	params := map[string]string{"query": query}

	data, err := APIRequest("/search", params)
	if err != nil {
		return nil, err
	}

	var response SearchResponse
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &response); err != nil {
		return nil, fmt.Errorf("failed to parse search response: %w", err)
	}

	return &response, nil
}

// GetSystemStats retrieves high-level system statistics
func GetSystemStats() (*SystemStats, error) {
	data, err := APIRequest("/stats", nil)
	if err != nil {
		return nil, err
	}

	var stats SystemStats
	jsonBytes, _ := json.Marshal(data)
	if err := json.Unmarshal(jsonBytes, &stats); err != nil {
		return nil, fmt.Errorf("failed to parse system stats: %w", err)
	}

	return &stats, nil
}

// ============================================================================
// DATA EXTRACTION HELPERS (15 functions)
// ============================================================================

// ExtractPools extracts pools from API response
func ExtractPools(data interface{}) []Pool {
	if data == nil {
		return []Pool{}
	}

	// Try to parse as PoolsResponse first
	jsonBytes, _ := json.Marshal(data)
	var poolsResp PoolsResponse
	if json.Unmarshal(jsonBytes, &poolsResp) == nil && len(poolsResp.Pools) > 0 {
		return poolsResp.Pools
	}

	// Try to parse as array of pools
	var pools []Pool
	if json.Unmarshal(jsonBytes, &pools) == nil {
		return pools
	}

	return []Pool{}
}

// ExtractTokens extracts tokens from API response
func ExtractTokens(data interface{}) []Token {
	if data == nil {
		return []Token{}
	}

	jsonBytes, _ := json.Marshal(data)

	// Try search response first
	var searchResp SearchResponse
	if json.Unmarshal(jsonBytes, &searchResp) == nil && len(searchResp.Tokens) > 0 {
		return searchResp.Tokens
	}

	// Try direct array
	var tokens []Token
	if json.Unmarshal(jsonBytes, &tokens) == nil {
		return tokens
	}

	return []Token{}
}

// ExtractDexes extracts DEXes from API response
func ExtractDexes(data interface{}) []Dex {
	if data == nil {
		return []Dex{}
	}

	jsonBytes, _ := json.Marshal(data)

	// Try DexesResponse first
	var dexesResp DexesResponse
	if json.Unmarshal(jsonBytes, &dexesResp) == nil && len(dexesResp.Dexes) > 0 {
		return dexesResp.Dexes
	}

	// Try search response
	var searchResp SearchResponse
	if json.Unmarshal(jsonBytes, &searchResp) == nil && len(searchResp.Dexes) > 0 {
		return searchResp.Dexes
	}

	// Try direct array
	var dexes []Dex
	if json.Unmarshal(jsonBytes, &dexes) == nil {
		return dexes
	}

	return []Dex{}
}

// ExtractTransactions extracts transactions from API response
func ExtractTransactions(data interface{}) []Transaction {
	if data == nil {
		return []Transaction{}
	}

	jsonBytes, _ := json.Marshal(data)
	var txResp TransactionsResponse
	if json.Unmarshal(jsonBytes, &txResp) == nil {
		return txResp.Transactions
	}

	var txs []Transaction
	if json.Unmarshal(jsonBytes, &txs) == nil {
		return txs
	}

	return []Transaction{}
}

// ExtractOHLCV extracts OHLCV data from API response
func ExtractOHLCV(data interface{}) []OHLCVRecord {
	if data == nil {
		return []OHLCVRecord{}
	}

	jsonBytes, _ := json.Marshal(data)
	var records []OHLCVRecord
	if json.Unmarshal(jsonBytes, &records) == nil {
		return records
	}

	return []OHLCVRecord{}
}

// ExtractPageInfo extracts pagination info from API response
func ExtractPageInfo(data interface{}) *PageInfo {
	if data == nil {
		return nil
	}

	jsonBytes, _ := json.Marshal(data)

	// Try different response types
	var poolsResp PoolsResponse
	if json.Unmarshal(jsonBytes, &poolsResp) == nil {
		return &poolsResp.PageInfo
	}

	var dexesResp DexesResponse
	if json.Unmarshal(jsonBytes, &dexesResp) == nil {
		return &dexesResp.PageInfo
	}

	var tokenPoolsResp TokenPoolsResponse
	if json.Unmarshal(jsonBytes, &tokenPoolsResp) == nil {
		return &tokenPoolsResp.PageInfo
	}

	return nil
}

// ExtractTokenSummary extracts token summary from token data
func ExtractTokenSummary(data interface{}) *TokenSummary {
	if data == nil {
		return nil
	}

	jsonBytes, _ := json.Marshal(data)
	var token Token
	if json.Unmarshal(jsonBytes, &token) == nil {
		return token.Summary
	}

	return nil
}

// ExtractTimeMetrics extracts time interval metrics from data
func ExtractTimeMetrics(data interface{}, timeframe string) *TimeIntervalMetrics {
	if data == nil {
		return nil
	}

	jsonBytes, _ := json.Marshal(data)

	// Try pool data
	var pool Pool
	if json.Unmarshal(jsonBytes, &pool) == nil {
		switch timeframe {
		case "24h":
			return pool.H24
		case "6h":
			return pool.H6
		case "1h":
			return pool.H1
		case "30m":
			return pool.M30
		case "15m":
			return pool.M15
		case "5m":
			return pool.M5
		}
	}

	// Try token summary
	var summary TokenSummary
	if json.Unmarshal(jsonBytes, &summary) == nil {
		switch timeframe {
		case "24h":
			return summary.H24
		case "6h":
			return summary.H6
		case "1h":
			return summary.H1
		case "30m":
			return summary.M30
		case "15m":
			return summary.M15
		case "5m":
			return summary.M5
		case "1m":
			return summary.M1
		}
	}

	return nil
}

// ExtractPoolMetrics extracts pool metrics from pool data
func ExtractPoolMetrics(data interface{}) map[string]interface{} {
	metrics := make(map[string]interface{})

	if data == nil {
		return metrics
	}

	jsonBytes, _ := json.Marshal(data)
	var pool Pool
	if json.Unmarshal(jsonBytes, &pool) == nil {
		metrics["volume_usd"] = pool.VolumeUSD
		metrics["price_usd"] = pool.PriceUSD
		metrics["transactions"] = pool.Transactions
		metrics["dex_name"] = pool.DexName
		metrics["chain"] = pool.Chain
		metrics["last_price_change_24h"] = pool.LastPriceChangeUSD24h
		metrics["created_at"] = pool.CreatedAt
		metrics["token_count"] = len(pool.Tokens)
	}

	return metrics
}

// ExtractTokenInfo extracts basic token information
func ExtractTokenInfo(data interface{}) map[string]interface{} {
	info := make(map[string]interface{})

	if data == nil {
		return info
	}

	jsonBytes, _ := json.Marshal(data)
	var token Token
	if json.Unmarshal(jsonBytes, &token) == nil {
		info["id"] = token.ID
		info["name"] = token.Name
		info["symbol"] = token.Symbol
		info["chain"] = token.Chain
		info["decimals"] = token.Decimals
		info["total_supply"] = token.TotalSupply
		info["website"] = token.Website
		info["fdv"] = token.FDV

		if token.Summary != nil {
			info["price_usd"] = token.Summary.PriceUSD
			info["liquidity_usd"] = token.Summary.LiquidityUSD
		}
	}

	return info
}

// ExtractTransactionInfo extracts transaction information
func ExtractTransactionInfo(data interface{}) map[string]interface{} {
	info := make(map[string]interface{})

	if data == nil {
		return info
	}

	jsonBytes, _ := json.Marshal(data)
	var tx Transaction
	if json.Unmarshal(jsonBytes, &tx) == nil {
		info["id"] = tx.ID
		info["pool_id"] = tx.PoolID
		info["sender"] = tx.Sender
		info["token_0"] = tx.Token0
		info["token_1"] = tx.Token1
		info["token_0_symbol"] = tx.Token0Symbol
		info["token_1_symbol"] = tx.Token1Symbol
		info["amount_0"] = tx.Amount0
		info["amount_1"] = tx.Amount1
		info["price_0_usd"] = tx.Price0USD
		info["price_1_usd"] = tx.Price1USD
		info["created_at"] = tx.CreatedAt
	}

	return info
}

// ExtractOHLCVMetrics extracts metrics from OHLCV data
func ExtractOHLCVMetrics(data interface{}) map[string]interface{} {
	metrics := make(map[string]interface{})

	records := ExtractOHLCV(data)
	if len(records) == 0 {
		return metrics
	}

	var totalVolume int64
	var prices []float64
	var high, low float64

	for i, record := range records {
		totalVolume += record.Volume
		prices = append(prices, record.Close)

		if i == 0 {
			high = record.High
			low = record.Low
		} else {
			if record.High > high {
				high = record.High
			}
			if record.Low < low {
				low = record.Low
			}
		}
	}

	metrics["total_volume"] = totalVolume
	metrics["data_points"] = len(records)
	metrics["highest_price"] = high
	metrics["lowest_price"] = low

	if len(records) > 0 {
		metrics["first_price"] = records[0].Open
		metrics["last_price"] = records[len(records)-1].Close

		// Calculate price change
		if records[0].Open > 0 {
			change := ((records[len(records)-1].Close - records[0].Open) / records[0].Open) * 100
			metrics["price_change_percent"] = change
		}
	}

	// Calculate volatility if we have enough data
	if len(prices) > 1 {
		metrics["volatility"] = CalculateVolatility(records)
	}

	return metrics
}

// ExtractSearchResults extracts and categorizes search results
func ExtractSearchResults(data interface{}) map[string]interface{} {
	results := make(map[string]interface{})

	searchResp := data.(*SearchResponse)
	if searchResp == nil {
		return results
	}

	results["tokens_count"] = len(searchResp.Tokens)
	results["pools_count"] = len(searchResp.Pools)
	results["dexes_count"] = len(searchResp.Dexes)
	results["total_results"] = len(searchResp.Tokens) + len(searchResp.Pools) + len(searchResp.Dexes)

	// Extract top results
	if len(searchResp.Tokens) > 0 {
		results["top_token"] = searchResp.Tokens[0]
	}
	if len(searchResp.Pools) > 0 {
		results["top_pool"] = searchResp.Pools[0]
	}
	if len(searchResp.Dexes) > 0 {
		results["top_dex"] = searchResp.Dexes[0]
	}

	return results
}

// ExtractSystemStats extracts system statistics
func ExtractSystemStats(data interface{}) map[string]interface{} {
	stats := make(map[string]interface{})

	if data == nil {
		return stats
	}

	jsonBytes, _ := json.Marshal(data)
	var systemStats SystemStats
	if json.Unmarshal(jsonBytes, &systemStats) == nil {
		stats["chains"] = systemStats.Chains
		stats["factories"] = systemStats.Factories
		stats["pools"] = systemStats.Pools
		stats["tokens"] = systemStats.Tokens
		stats["total_entities"] = systemStats.Chains + systemStats.Factories + systemStats.Pools + systemStats.Tokens
	}

	return stats
}

// ExtractPoolTokens extracts tokens from pool data
func ExtractPoolTokens(data interface{}) []Token {
	if data == nil {
		return []Token{}
	}

	jsonBytes, _ := json.Marshal(data)
	var pool Pool
	if json.Unmarshal(jsonBytes, &pool) == nil {
		return pool.Tokens
	}

	return []Token{}
}

// ============================================================================
// FILTERING & SORTING HELPERS (12 functions)
// ============================================================================

// FilterByPriceChange filters items by price change percentage
func FilterByPriceChange(pools []Pool, minChange float64) []Pool {
	var filtered []Pool
	for _, pool := range pools {
		if math.Abs(pool.LastPriceChangeUSD24h) >= minChange {
			filtered = append(filtered, pool)
		}
	}
	return filtered
}

// FilterByVolume filters items by minimum volume
func FilterByVolume(pools []Pool, minVolume float64) []Pool {
	var filtered []Pool
	for _, pool := range pools {
		if pool.VolumeUSD >= minVolume {
			filtered = append(filtered, pool)
		}
	}
	return filtered
}

// FilterByNetwork filters items by network
func FilterByNetwork(pools []Pool, network string) []Pool {
	var filtered []Pool
	for _, pool := range pools {
		if strings.EqualFold(pool.Chain, network) {
			filtered = append(filtered, pool)
		}
	}
	return filtered
}

// FilterByDex filters items by DEX name
func FilterByDex(pools []Pool, dexName string) []Pool {
	var filtered []Pool
	for _, pool := range pools {
		if strings.Contains(strings.ToLower(pool.DexName), strings.ToLower(dexName)) {
			filtered = append(filtered, pool)
		}
	}
	return filtered
}

// FilterByTokenSymbol filters pools by token symbol
func FilterByTokenSymbol(pools []Pool, symbol string) []Pool {
	var filtered []Pool
	for _, pool := range pools {
		for _, token := range pool.Tokens {
			if strings.EqualFold(token.Symbol, symbol) {
				filtered = append(filtered, pool)
				break
			}
		}
	}
	return filtered
}

// FilterByTokenAddress filters pools by token address
func FilterByTokenAddress(pools []Pool, address string) []Pool {
	var filtered []Pool
	for _, pool := range pools {
		for _, token := range pool.Tokens {
			if strings.EqualFold(token.ID, address) {
				filtered = append(filtered, pool)
				break
			}
		}
	}
	return filtered
}

// TopN returns the top N items by specified field
func TopN(pools []Pool, field string, n int) []Pool {
	sorted := SortByField(pools, field, true)
	if len(sorted) > n {
		return sorted[:n]
	}
	return sorted
}

// BottomN returns the bottom N items by specified field
func BottomN(pools []Pool, field string, n int) []Pool {
	sorted := SortByField(pools, field, false)
	if len(sorted) > n {
		return sorted[:n]
	}
	return sorted
}

// SortByField sorts pools by specified field
func SortByField(pools []Pool, field string, descending bool) []Pool {
	sorted := make([]Pool, len(pools))
	copy(sorted, pools)

	sort.Slice(sorted, func(i, j int) bool {
		var valI, valJ float64

		switch field {
		case "volume_usd":
			valI, valJ = sorted[i].VolumeUSD, sorted[j].VolumeUSD
		case "price_usd":
			valI, valJ = sorted[i].PriceUSD, sorted[j].PriceUSD
		case "transactions":
			valI, valJ = float64(sorted[i].Transactions), float64(sorted[j].Transactions)
		case "last_price_change_usd_24h":
			valI, valJ = sorted[i].LastPriceChangeUSD24h, sorted[j].LastPriceChangeUSD24h
		default:
			return false
		}

		if descending {
			return valI > valJ
		}
		return valI < valJ
	})

	return sorted
}

// FilterRecentTransactions filters transactions by time
func FilterRecentTransactions(transactions []Transaction, hours int) []Transaction {
	cutoff := time.Now().Add(-time.Duration(hours) * time.Hour)
	var filtered []Transaction

	for _, tx := range transactions {
		if txTime, err := time.Parse(time.RFC3339, tx.CreatedAt); err == nil {
			if txTime.After(cutoff) {
				filtered = append(filtered, tx)
			}
		}
	}

	return filtered
}

// FilterLargeTransactions filters transactions by minimum USD value
func FilterLargeTransactions(transactions []Transaction, minUSD float64) []Transaction {
	var filtered []Transaction

	for _, tx := range transactions {
		totalUSD := tx.Price0USD + tx.Price1USD
		if totalUSD >= minUSD {
			filtered = append(filtered, tx)
		}
	}

	return filtered
}

// FilterByTimeframe filters OHLCV data by timeframe
func FilterByTimeframe(records []OHLCVRecord, startTime string, endTime string) []OHLCVRecord {
	start, err := time.Parse(time.RFC3339, startTime)
	if err != nil {
		return records
	}

	var end time.Time
	if endTime != "" {
		end, err = time.Parse(time.RFC3339, endTime)
		if err != nil {
			end = time.Now()
		}
	} else {
		end = time.Now()
	}

	var filtered []OHLCVRecord
	for _, record := range records {
		if recordTime, err := time.Parse(time.RFC3339, record.TimeOpen); err == nil {
			if recordTime.After(start) && recordTime.Before(end) {
				filtered = append(filtered, record)
			}
		}
	}

	return filtered
}

// ============================================================================
// ANALYSIS HELPERS (10 functions)
// ============================================================================

// CalculatePriceChange calculates percentage price change
func CalculatePriceChange(current, previous float64) float64 {
	if previous == 0 {
		return 0
	}
	return ((current - previous) / previous) * 100
}

// CalculateVolumeWeightedPrice calculates volume-weighted average price
func CalculateVolumeWeightedPrice(records []OHLCVRecord) float64 {
	if len(records) == 0 {
		return 0
	}

	var totalValue float64
	var totalVolume int64

	for _, record := range records {
		avgPrice := (record.High + record.Low + record.Close) / 3
		totalValue += avgPrice * float64(record.Volume)
		totalVolume += record.Volume
	}

	if totalVolume == 0 {
		return 0
	}

	return totalValue / float64(totalVolume)
}

// CalculateVolatility calculates price volatility
func CalculateVolatility(records []OHLCVRecord) float64 {
	if len(records) < 2 {
		return 0
	}

	// Calculate returns
	var returns []float64
	for i := 1; i < len(records); i++ {
		if records[i-1].Close > 0 {
			ret := (records[i].Close - records[i-1].Close) / records[i-1].Close
			returns = append(returns, ret)
		}
	}

	if len(returns) == 0 {
		return 0
	}

	// Calculate mean
	var sum float64
	for _, ret := range returns {
		sum += ret
	}
	mean := sum / float64(len(returns))

	// Calculate variance
	var variance float64
	for _, ret := range returns {
		variance += math.Pow(ret-mean, 2)
	}
	variance /= float64(len(returns))

	// Return standard deviation (volatility)
	return math.Sqrt(variance)
}

// AnalyzePoolActivity analyzes pool activity metrics
func AnalyzePoolActivity(pool Pool) map[string]interface{} {
	analysis := make(map[string]interface{})

	analysis["pool_id"] = pool.ID
	analysis["dex_name"] = pool.DexName
	analysis["chain"] = pool.Chain
	analysis["volume_usd"] = pool.VolumeUSD
	analysis["transactions"] = pool.Transactions
	analysis["price_usd"] = pool.PriceUSD

	// Calculate volume per transaction
	if pool.Transactions > 0 {
		analysis["volume_per_transaction"] = pool.VolumeUSD / float64(pool.Transactions)
	}

	// Analyze price changes
	analysis["price_change_24h"] = pool.LastPriceChangeUSD24h
	analysis["price_change_1h"] = pool.LastPriceChangeUSD1h
	analysis["price_change_5m"] = pool.LastPriceChangeUSD5m

	// Activity score (arbitrary scoring based on volume and transactions)
	activityScore := math.Log10(pool.VolumeUSD+1) * math.Log10(float64(pool.Transactions)+1)
	analysis["activity_score"] = activityScore

	// Token information
	if len(pool.Tokens) >= 2 {
		analysis["token_pair"] = fmt.Sprintf("%s/%s", pool.Tokens[0].Symbol, pool.Tokens[1].Symbol)
	}

	return analysis
}

// AnalyzeTokenPerformance analyzes token performance metrics
func AnalyzeTokenPerformance(token Token) map[string]interface{} {
	analysis := make(map[string]interface{})

	analysis["token_id"] = token.ID
	analysis["name"] = token.Name
	analysis["symbol"] = token.Symbol
	analysis["chain"] = token.Chain
	analysis["fdv"] = token.FDV

	if token.Summary != nil {
		analysis["price_usd"] = token.Summary.PriceUSD
		analysis["liquidity_usd"] = token.Summary.LiquidityUSD
		analysis["pools_count"] = token.Summary.Pools

		// Calculate liquidity per pool
		if token.Summary.Pools > 0 {
			analysis["avg_liquidity_per_pool"] = token.Summary.LiquidityUSD / float64(token.Summary.Pools)
		}

		// Market cap to liquidity ratio
		if token.Summary.LiquidityUSD > 0 {
			analysis["fdv_to_liquidity_ratio"] = token.FDV / token.Summary.LiquidityUSD
		}

		// 24h metrics
		if token.Summary.H24 != nil {
			analysis["volume_24h"] = token.Summary.H24.VolumeUSD
			analysis["price_change_24h"] = token.Summary.H24.LastPriceUSDChange
			analysis["transactions_24h"] = token.Summary.H24.Txns

			// Trading activity ratio
			if token.Summary.LiquidityUSD > 0 {
				analysis["volume_to_liquidity_ratio"] = token.Summary.H24.VolumeUSD / token.Summary.LiquidityUSD
			}
		}
	}

	return analysis
}

// DetectAnomalies detects statistical anomalies in data
func DetectAnomalies(pools []Pool, field string, threshold float64) []AnomalyResult {
	var values []float64

	// Extract values
	for _, pool := range pools {
		switch field {
		case "volume_usd":
			values = append(values, pool.VolumeUSD)
		case "price_usd":
			values = append(values, pool.PriceUSD)
		case "transactions":
			values = append(values, float64(pool.Transactions))
		case "last_price_change_usd_24h":
			values = append(values, pool.LastPriceChangeUSD24h)
		}
	}

	if len(values) < 3 {
		return []AnomalyResult{}
	}

	// Calculate mean and standard deviation
	var sum float64
	for _, val := range values {
		sum += val
	}
	mean := sum / float64(len(values))

	var sumSquares float64
	for _, val := range values {
		sumSquares += math.Pow(val-mean, 2)
	}
	stdDev := math.Sqrt(sumSquares / float64(len(values)))

	// Detect anomalies
	var anomalies []AnomalyResult
	for i, val := range values {
		zScore := math.Abs((val - mean) / stdDev)
		if zScore > threshold {
			anomalies = append(anomalies, AnomalyResult{
				Index:  i,
				Value:  val,
				ZScore: zScore,
				Item:   pools[i],
			})
		}
	}

	return anomalies
}

// CalculateCorrelation calculates correlation between two data series
func CalculateCorrelation(data1, data2 []float64) float64 {
	n := len(data1)
	if n != len(data2) || n < 2 {
		return 0
	}

	// Calculate means
	var sum1, sum2 float64
	for i := 0; i < n; i++ {
		sum1 += data1[i]
		sum2 += data2[i]
	}
	mean1 := sum1 / float64(n)
	mean2 := sum2 / float64(n)

	// Calculate correlation
	var numerator, sumSquares1, sumSquares2 float64
	for i := 0; i < n; i++ {
		diff1 := data1[i] - mean1
		diff2 := data2[i] - mean2
		numerator += diff1 * diff2
		sumSquares1 += diff1 * diff1
		sumSquares2 += diff2 * diff2
	}

	denominator := math.Sqrt(sumSquares1 * sumSquares2)
	if denominator == 0 {
		return 0
	}

	return numerator / denominator
}

// AnalyzeLiquidityDistribution analyzes liquidity distribution across pools
func AnalyzeLiquidityDistribution(pools []Pool) LiquidityAnalysis {
	if len(pools) == 0 {
		return LiquidityAnalysis{}
	}

	// Extract volume data (using as proxy for liquidity)
	var volumes []float64
	var totalVolume float64

	for _, pool := range pools {
		volumes = append(volumes, pool.VolumeUSD)
		totalVolume += pool.VolumeUSD
	}

	// Sort volumes for percentile calculations
	sort.Float64s(volumes)

	// Calculate metrics
	analysis := LiquidityAnalysis{
		TotalLiquidity:   totalVolume,
		PoolCount:        len(pools),
		AverageLiquidity: totalVolume / float64(len(pools)),
	}

	// Median
	if len(volumes)%2 == 0 {
		analysis.MedianLiquidity = (volumes[len(volumes)/2-1] + volumes[len(volumes)/2]) / 2
	} else {
		analysis.MedianLiquidity = volumes[len(volumes)/2]
	}

	// Gini coefficient
	analysis.GiniCoefficient = CalculateGiniCoefficient(volumes)

	// Top pools share (top 10%)
	topCount := len(volumes) / 10
	if topCount < 1 {
		topCount = 1
	}

	var topVolume float64
	for i := len(volumes) - topCount; i < len(volumes); i++ {
		topVolume += volumes[i]
	}

	if totalVolume > 0 {
		analysis.TopPoolsShare = topVolume / totalVolume
	}

	// Distribution by ranges
	analysis.Distribution = make(map[string]float64)
	ranges := []struct {
		name string
		min  float64
		max  float64
	}{
		{"< 1M", 0, 1000000},
		{"1M-10M", 1000000, 10000000},
		{"10M-100M", 10000000, 100000000},
		{"> 100M", 100000000, math.Inf(1)},
	}

	for _, r := range ranges {
		var count int
		for _, vol := range volumes {
			if vol >= r.min && vol < r.max {
				count++
			}
		}
		analysis.Distribution[r.name] = float64(count) / float64(len(volumes))
	}

	return analysis
}

// CalculateGiniCoefficient calculates the Gini coefficient for inequality measurement
func CalculateGiniCoefficient(values []float64) float64 {
	n := len(values)
	if n < 2 {
		return 0
	}

	// Sort values
	sorted := make([]float64, len(values))
	copy(sorted, values)
	sort.Float64s(sorted)

	// Calculate Gini coefficient
	var sum float64
	for i, val := range sorted {
		sum += val * float64(2*i+1-n)
	}

	var mean float64
	for _, val := range sorted {
		mean += val
	}
	mean /= float64(n)

	if mean == 0 {
		return 0
	}

	return sum / (float64(n*n) * mean)
}

// AnalyzeTransactionPatterns analyzes transaction patterns
func AnalyzeTransactionPatterns(transactions []Transaction) map[string]interface{} {
	analysis := make(map[string]interface{})

	if len(transactions) == 0 {
		return analysis
	}

	analysis["total_transactions"] = len(transactions)

	// Analyze by hour
	hourCounts := make(map[int]int)
	var totalUSD float64

	for _, tx := range transactions {
		totalUSD += tx.Price0USD + tx.Price1USD

		if txTime, err := time.Parse(time.RFC3339, tx.CreatedAt); err == nil {
			hour := txTime.Hour()
			hourCounts[hour]++
		}
	}

	analysis["total_value_usd"] = totalUSD
	if len(transactions) > 0 {
		analysis["avg_value_per_tx"] = totalUSD / float64(len(transactions))
	}

	// Find peak hour
	var peakHour int
	var maxCount int
	for hour, count := range hourCounts {
		if count > maxCount {
			maxCount = count
			peakHour = hour
		}
	}

	analysis["peak_hour"] = peakHour
	analysis["peak_hour_count"] = maxCount
	analysis["hourly_distribution"] = hourCounts

	// Analyze token pairs
	pairCounts := make(map[string]int)
	for _, tx := range transactions {
		pair := fmt.Sprintf("%s/%s", tx.Token0Symbol, tx.Token1Symbol)
		pairCounts[pair]++
	}

	analysis["unique_pairs"] = len(pairCounts)
	analysis["pair_distribution"] = pairCounts

	return analysis
}

// ============================================================================
// UTILITY HELPERS (8 functions)
// ============================================================================

// FormatNumber formats a number with thousand separators
func FormatNumber(num float64, decimals int) string {
	format := fmt.Sprintf("%%.%df", decimals)
	str := fmt.Sprintf(format, num)

	// Add thousand separators
	parts := strings.Split(str, ".")
	intPart := parts[0]

	// Add commas
	var result strings.Builder
	for i, digit := range intPart {
		if i > 0 && (len(intPart)-i)%3 == 0 {
			result.WriteString(",")
		}
		result.WriteRune(digit)
	}

	if len(parts) > 1 {
		result.WriteString(".")
		result.WriteString(parts[1])
	}

	return result.String()
}

// FormatPercentage formats a number as a percentage
func FormatPercentage(num float64, decimals int) string {
	format := fmt.Sprintf("%%.%df%%%%", decimals)
	return fmt.Sprintf(format, num)
}

// PrintTable prints data in a formatted table
func PrintTable(pools []Pool, columns []string, title string) {
	if len(pools) == 0 {
		fmt.Println("No data to display")
		return
	}

	table := tablewriter.NewWriter(os.Stdout)
	table.SetHeader(columns)
	table.SetCaption(true, title)

	for _, pool := range pools {
		row := make([]string, len(columns))
		for i, col := range columns {
			switch col {
			case "dex_name":
				row[i] = pool.DexName
			case "volume_usd":
				row[i] = FormatNumber(pool.VolumeUSD, 0)
			case "price_usd":
				row[i] = FormatNumber(pool.PriceUSD, 6)
			case "transactions":
				row[i] = strconv.Itoa(pool.Transactions)
			case "chain":
				row[i] = pool.Chain
			case "last_price_change_usd_24h":
				row[i] = FormatPercentage(pool.LastPriceChangeUSD24h, 2)
			default:
				row[i] = ""
			}
		}
		table.Append(row)
	}

	table.Render()
}

// SaveToCSV saves data to a CSV file
func SaveToCSV(pools []Pool, filename string, columns []string) error {
	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	// Write header
	if err := writer.Write(columns); err != nil {
		return err
	}

	// Write data
	for _, pool := range pools {
		row := make([]string, len(columns))
		for i, col := range columns {
			switch col {
			case "id":
				row[i] = pool.ID
			case "dex_name":
				row[i] = pool.DexName
			case "volume_usd":
				row[i] = fmt.Sprintf("%.2f", pool.VolumeUSD)
			case "price_usd":
				row[i] = fmt.Sprintf("%.6f", pool.PriceUSD)
			case "transactions":
				row[i] = strconv.Itoa(pool.Transactions)
			case "chain":
				row[i] = pool.Chain
			case "last_price_change_usd_24h":
				row[i] = fmt.Sprintf("%.2f", pool.LastPriceChangeUSD24h)
			case "created_at":
				row[i] = pool.CreatedAt
			default:
				row[i] = ""
			}
		}
		if err := writer.Write(row); err != nil {
			return err
		}
	}

	return nil
}

// LoadEnv loads environment variable with default value
func LoadEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

// ValidateNetwork validates if a network is supported
func ValidateNetwork(network string) (bool, error) {
	networks, err := GetNetworks()
	if err != nil {
		return false, err
	}

	for _, net := range networks {
		if strings.EqualFold(net.ID, network) {
			return true, nil
		}
	}

	return false, nil
}

// ValidateTokenAddress validates if a token address format is correct
func ValidateTokenAddress(address string) bool {
	// Ethereum address pattern (0x followed by 40 hex characters)
	ethPattern := `^0x[a-fA-F0-9]{40}$`
	if matched, _ := regexp.MatchString(ethPattern, address); matched {
		return true
	}

	// Solana address pattern (base58, 32-44 characters)
	solanaPattern := `^[1-9A-HJ-NP-Za-km-z]{32,44}$`
	if matched, _ := regexp.MatchString(solanaPattern, address); matched {
		return true
	}

	return false
}

// CreateTimestamp creates an ISO timestamp for days ago
func CreateTimestamp(daysAgo int) string {
	t := time.Now().AddDate(0, 0, -daysAgo)
	return t.Format(time.RFC3339)
}

// ============================================================================
// ADVANCED HELPERS (5 functions)
// ============================================================================

// GetTopMovers retrieves top price movers with volume filter
func GetTopMovers(network string, limit int, minVolume float64) ([]Pool, error) {
	params := map[string]string{
		"limit": strconv.Itoa(limit * 3), // Get more to filter
	}

	response, err := GetNetworkPools(network, params)
	if err != nil {
		return nil, err
	}

	// Filter by volume and sort by absolute price change
	filtered := FilterByVolume(response.Pools, minVolume)

	// Sort by absolute price change
	sort.Slice(filtered, func(i, j int) bool {
		return math.Abs(filtered[i].LastPriceChangeUSD24h) > math.Abs(filtered[j].LastPriceChangeUSD24h)
	})

	if len(filtered) > limit {
		return filtered[:limit], nil
	}

	return filtered, nil
}

// GetHighVolumePools retrieves pools with highest volume
func GetHighVolumePools(network string, limit int) ([]Pool, error) {
	params := map[string]string{
		"limit":    strconv.Itoa(limit),
		"order_by": "volume_usd",
		"sort":     "desc",
	}

	response, err := GetNetworkPools(network, params)
	if err != nil {
		return nil, err
	}

	return response.Pools, nil
}

// GetTokenLiquidityAnalysis analyzes token liquidity across pools
func GetTokenLiquidityAnalysis(tokenAddress, network string) (map[string]interface{}, error) {
	// Get token details
	token, err := GetTokenDetails(network, tokenAddress)
	if err != nil {
		return nil, err
	}

	// Get token pools
	poolsResp, err := GetTokenPools(network, tokenAddress, map[string]string{"limit": "50"})
	if err != nil {
		return nil, err
	}

	analysis := make(map[string]interface{})
	analysis["token"] = token
	analysis["pools_count"] = len(poolsResp.Pools)

	if len(poolsResp.Pools) == 0 {
		return analysis, nil
	}

	// Analyze liquidity distribution
	liquidityAnalysis := AnalyzeLiquidityDistribution(poolsResp.Pools)
	analysis["liquidity_analysis"] = liquidityAnalysis

	// Find largest pool
	var largestPool Pool
	var maxVolume float64
	for _, pool := range poolsResp.Pools {
		if pool.VolumeUSD > maxVolume {
			maxVolume = pool.VolumeUSD
			largestPool = pool
		}
	}
	analysis["largest_pool"] = largestPool

	// DEX distribution
	dexDistribution := AnalyzeDexDistribution(poolsResp.Pools)
	analysis["dex_distribution"] = dexDistribution

	return analysis, nil
}

// AnalyzeDexDistribution analyzes DEX distribution across pools
func AnalyzeDexDistribution(pools []Pool) DexDistribution {
	if len(pools) == 0 {
		return DexDistribution{}
	}

	dexVolumes := make(map[string]float64)
	var totalVolume float64

	for _, pool := range pools {
		dexVolumes[pool.DexName] += pool.VolumeUSD
		totalVolume += pool.VolumeUSD
	}

	// Calculate percentages
	distribution := make(map[string]float64)
	var topDexes []string

	for dex, volume := range dexVolumes {
		if totalVolume > 0 {
			percentage := volume / totalVolume
			distribution[dex] = percentage
			topDexes = append(topDexes, dex)
		}
	}

	// Sort DEXes by volume
	sort.Slice(topDexes, func(i, j int) bool {
		return dexVolumes[topDexes[i]] > dexVolumes[topDexes[j]]
	})

	// Calculate concentration (Herfindahl-Hirschman Index)
	var concentration float64
	for _, percentage := range distribution {
		concentration += percentage * percentage
	}

	return DexDistribution{
		TotalVolume:   totalVolume,
		DexCount:      len(dexVolumes),
		Distribution:  distribution,
		TopDexes:      topDexes,
		Concentration: concentration,
	}
}

// GetMarketOverview retrieves comprehensive market overview
func GetMarketOverview() (*MarketOverview, error) {
	// Get system stats
	stats, err := GetSystemStats()
	if err != nil {
		return nil, err
	}

	// Get networks
	networks, err := GetNetworks()
	if err != nil {
		return nil, err
	}

	networkOverview := make(map[string]interface{})

	// Analyze top 5 networks
	for i, network := range networks {
		if i >= 5 {
			break
		}

		pools, err := GetNetworkPools(network.ID, map[string]string{"limit": "10"})
		if err != nil {
			continue
		}

		var totalVolume float64
		for _, pool := range pools.Pools {
			totalVolume += pool.VolumeUSD
		}

		networkOverview[network.ID] = map[string]interface{}{
			"display_name": network.DisplayName,
			"total_volume": totalVolume,
			"pool_count":   len(pools.Pools),
		}
	}

	return &MarketOverview{
		SystemStats:     *stats,
		NetworkOverview: networkOverview,
		Timestamp:       time.Now().Format(time.RFC3339),
	}, nil
}

// ============================================================================
// ASYNC HELPERS (5 functions)
// ============================================================================

// AsyncAPIRequest makes an async API request with context
func AsyncAPIRequest(ctx context.Context, endpoint string, params map[string]string) (interface{}, error) {
	// Use the same logic as APIRequest but with context support
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	default:
		return APIRequest(endpoint, params)
	}
}

// AsyncGetMultiplePools gets pools from multiple networks concurrently
func AsyncGetMultiplePools(ctx context.Context, networks []string, limit int) (map[string]interface{}, error) {
	results := make(map[string]interface{})
	resultsChan := make(chan struct {
		network string
		data    interface{}
		err     error
	}, len(networks))

	// Start goroutines for each network
	for _, network := range networks {
		go func(net string) {
			params := map[string]string{"limit": strconv.Itoa(limit)}
			data, err := AsyncAPIRequest(ctx, fmt.Sprintf("/networks/%s/pools", net), params)
			resultsChan <- struct {
				network string
				data    interface{}
				err     error
			}{net, data, err}
		}(network)
	}

	// Collect results
	for i := 0; i < len(networks); i++ {
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		case result := <-resultsChan:
			if result.err != nil {
				results[result.network] = map[string]string{"error": result.err.Error()}
			} else {
				results[result.network] = result.data
			}
		}
	}

	return results, nil
}

// AsyncGetTokenDataBatch gets data for multiple tokens concurrently
func AsyncGetTokenDataBatch(ctx context.Context, tokenAddresses []string, network string) ([]AsyncResult, error) {
	results := make([]AsyncResult, len(tokenAddresses))
	resultsChan := make(chan struct {
		index int
		data  interface{}
		err   error
	}, len(tokenAddresses))

	// Start goroutines for each token
	for i, address := range tokenAddresses {
		go func(idx int, addr string) {
			data, err := AsyncAPIRequest(ctx, fmt.Sprintf("/networks/%s/tokens/%s", network, addr), nil)
			resultsChan <- struct {
				index int
				data  interface{}
				err   error
			}{idx, data, err}
		}(i, address)
	}

	// Collect results
	for i := 0; i < len(tokenAddresses); i++ {
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		case result := <-resultsChan:
			if result.err != nil {
				results[result.index] = AsyncResult{
					Data:  nil,
					Error: result.err.Error(),
				}
			} else {
				results[result.index] = AsyncResult{
					Data:  result.data,
					Error: "",
				}
			}
		}
	}

	return results, nil
}

// AsyncMonitorPrices monitors pool prices with callback
func AsyncMonitorPrices(ctx context.Context, poolAddresses []string, network string, interval time.Duration, callback func(string, map[string]interface{})) error {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-ticker.C:
			// Get current prices for all pools
			for _, address := range poolAddresses {
				go func(addr string) {
					pool, err := GetPoolDetails(network, addr, false)
					if err == nil && callback != nil {
						priceUpdate := map[string]interface{}{
							"price_usd":                 pool.PriceUSD,
							"last_price_change_usd_24h": pool.LastPriceChangeUSD24h,
							"volume_usd":                pool.VolumeUSD,
							"timestamp":                 time.Now().Format(time.RFC3339),
						}
						callback(addr, priceUpdate)
					}
				}(address)
			}
		}
	}
}

// AsyncBatchSearch performs multiple searches concurrently
func AsyncBatchSearch(ctx context.Context, queries []string) ([]AsyncResult, error) {
	results := make([]AsyncResult, len(queries))
	resultsChan := make(chan struct {
		index int
		query string
		data  interface{}
		err   error
	}, len(queries))

	// Start goroutines for each query
	for i, query := range queries {
		go func(idx int, q string) {
			params := map[string]string{"query": q}
			data, err := AsyncAPIRequest(ctx, "/search", params)
			resultsChan <- struct {
				index int
				query string
				data  interface{}
				err   error
			}{idx, q, data, err}
		}(i, query)
	}

	// Collect results
	for i := 0; i < len(queries); i++ {
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		case result := <-resultsChan:
			if result.err != nil {
				results[result.index] = AsyncResult{
					Query: result.query,
					Data:  nil,
					Error: result.err.Error(),
				}
			} else {
				results[result.index] = AsyncResult{
					Query: result.query,
					Data:  result.data,
					Error: "",
				}
			}
		}
	}

	return results, nil
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

// createCacheKey creates a cache key from endpoint and parameters
func createCacheKey(endpoint string, params map[string]string) string {
	key := endpoint
	if params != nil {
		var keys []string
		for k, v := range params {
			keys = append(keys, fmt.Sprintf("%s=%s", k, v))
		}
		sort.Strings(keys)
		key += "?" + strings.Join(keys, "&")
	}

	hash := md5.Sum([]byte(key))
	return fmt.Sprintf("%x", hash)
}
