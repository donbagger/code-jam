package paprikahelpers

import "time"

// Network represents a blockchain network
type Network struct {
	ID          string `json:"id"`
	DisplayName string `json:"display_name"`
}

// Token represents a token with all its metadata
type Token struct {
	ID          string        `json:"id"`
	Name        string        `json:"name"`
	Symbol      string        `json:"symbol"`
	Chain       string        `json:"chain"`
	Type        string        `json:"type"`
	Status      string        `json:"status"`
	Decimals    int           `json:"decimals"`
	TotalSupply float64       `json:"total_supply"`
	Description string        `json:"description"`
	Website     string        `json:"website"`
	Explorer    string        `json:"explorer"`
	AddedAt     string        `json:"added_at"`
	FDV         float64       `json:"fdv"`
	LastUpdated string        `json:"last_updated"`
	Summary     *TokenSummary `json:"summary,omitempty"`
}

// TokenSummary represents token summary metrics
type TokenSummary struct {
	PriceUSD     float64              `json:"price_usd"`
	FDV          float64              `json:"fdv"`
	LiquidityUSD float64              `json:"liquidity_usd"`
	Pools        int                  `json:"pools"`
	H24          *TimeIntervalMetrics `json:"24h,omitempty"`
	H6           *TimeIntervalMetrics `json:"6h,omitempty"`
	H1           *TimeIntervalMetrics `json:"1h,omitempty"`
	M30          *TimeIntervalMetrics `json:"30m,omitempty"`
	M15          *TimeIntervalMetrics `json:"15m,omitempty"`
	M5           *TimeIntervalMetrics `json:"5m,omitempty"`
	M1           *TimeIntervalMetrics `json:"1m,omitempty"`
}

// TimeIntervalMetrics represents transaction and volume metrics for a time interval
type TimeIntervalMetrics struct {
	Volume             float64 `json:"volume"`
	VolumeUSD          float64 `json:"volume_usd"`
	BuyUSD             float64 `json:"buy_usd"`
	SellUSD            float64 `json:"sell_usd"`
	Sells              int     `json:"sells"`
	Buys               int     `json:"buys"`
	Txns               int     `json:"txns"`
	LastPriceUSDChange float64 `json:"last_price_usd_change"`
}

// Pool represents a liquidity pool with pricing data
type Pool struct {
	ID                    string               `json:"id"`
	DexID                 string               `json:"dex_id"`
	DexName               string               `json:"dex_name"`
	Chain                 string               `json:"chain"`
	VolumeUSD             float64              `json:"volume_usd"`
	CreatedAt             string               `json:"created_at"`
	CreatedAtBlockNumber  int64                `json:"created_at_block_number"`
	Transactions          int                  `json:"transactions"`
	PriceUSD              float64              `json:"price_usd"`
	LastPriceChangeUSD5m  float64              `json:"last_price_change_usd_5m"`
	LastPriceChangeUSD1h  float64              `json:"last_price_change_usd_1h"`
	LastPriceChangeUSD24h float64              `json:"last_price_change_usd_24h"`
	Fee                   *float64             `json:"fee"`
	Tokens                []Token              `json:"tokens"`
	LastPrice             *float64             `json:"last_price,omitempty"`
	LastPriceUSD          *float64             `json:"last_price_usd,omitempty"`
	PriceTime             *string              `json:"price_time,omitempty"`
	H24                   *TimeIntervalMetrics `json:"24h,omitempty"`
	H6                    *TimeIntervalMetrics `json:"6h,omitempty"`
	H1                    *TimeIntervalMetrics `json:"1h,omitempty"`
	M30                   *TimeIntervalMetrics `json:"30m,omitempty"`
	M15                   *TimeIntervalMetrics `json:"15m,omitempty"`
	M5                    *TimeIntervalMetrics `json:"5m,omitempty"`
}

// PoolsResponse represents the response from pools endpoints
type PoolsResponse struct {
	Pools    []Pool   `json:"pools"`
	PageInfo PageInfo `json:"page_info"`
}

// PageInfo represents pagination information
type PageInfo struct {
	Limit      int `json:"limit"`
	Page       int `json:"page"`
	TotalItems int `json:"total_items"`
	TotalPages int `json:"total_pages"`
}

// Dex represents a decentralized exchange
type Dex struct {
	ID           string   `json:"id"`
	DexID        string   `json:"dex_id"`
	DexName      string   `json:"dex_name"`
	Chain        string   `json:"chain"`
	Protocol     string   `json:"protocol"`
	VolumeUSD24h *float64 `json:"volume_usd_24h,omitempty"`
	Txns24h      *int     `json:"txns_24h,omitempty"`
	PoolsCount   *int     `json:"pools_count,omitempty"`
	CreatedAt    *string  `json:"created_at,omitempty"`
}

// DexesResponse represents the response from dexes endpoints
type DexesResponse struct {
	Dexes    []Dex    `json:"dexes"`
	PageInfo PageInfo `json:"page_info"`
}

// OHLCVRecord represents OHLCV data point
type OHLCVRecord struct {
	TimeOpen  string  `json:"time_open"`
	TimeClose string  `json:"time_close"`
	Open      float64 `json:"open"`
	High      float64 `json:"high"`
	Low       float64 `json:"low"`
	Close     float64 `json:"close"`
	Volume    int64   `json:"volume"`
}

// Transaction represents a transaction
type Transaction struct {
	ID                   string   `json:"id"`
	LogIndex             *float64 `json:"log_index"`
	TransactionIndex     *float64 `json:"transaction_index"`
	PoolID               string   `json:"pool_id"`
	Sender               string   `json:"sender"`
	Recipient            *float64 `json:"recipient"`
	Token0               string   `json:"token_0"`
	Token0Symbol         string   `json:"token_0_symbol"`
	Token1               string   `json:"token_1"`
	Token1Symbol         string   `json:"token_1_symbol"`
	Amount0              string   `json:"amount_0"`
	Amount1              string   `json:"amount_1"`
	Price0               float64  `json:"price_0"`
	Price1               float64  `json:"price_1"`
	Price0USD            float64  `json:"price_0_usd"`
	Price1USD            float64  `json:"price_1_usd"`
	CreatedAtBlockNumber float64  `json:"created_at_block_number"`
	CreatedAt            string   `json:"created_at"`
}

// TransactionsResponse represents the response from transactions endpoints
type TransactionsResponse struct {
	Transactions []Transaction `json:"transactions"`
	PageInfo     interface{}   `json:"page_info"` // Can be PageInfo or cursor-based
}

// TokenPoolsResponse represents the response from token pools endpoints
type TokenPoolsResponse struct {
	Pools    []Pool   `json:"pools"`
	PageInfo PageInfo `json:"page_info"`
}

// SearchResponse represents the response from search endpoint
type SearchResponse struct {
	Tokens []Token `json:"tokens"`
	Pools  []Pool  `json:"pools"`
	Dexes  []Dex   `json:"dexes"`
}

// SystemStats represents system statistics
type SystemStats struct {
	Chains    int `json:"chains"`
	Factories int `json:"factories"`
	Pools     int `json:"pools"`
	Tokens    int `json:"tokens"`
}

// APIError represents an API error response
type APIError struct {
	Error   string `json:"error"`
	Message string `json:"message,omitempty"`
}

// CacheEntry represents a cached API response
type CacheEntry struct {
	Data      interface{}
	Timestamp time.Time
}

// AnalysisResult represents the result of various analysis functions
type AnalysisResult struct {
	Field     string      `json:"field"`
	Value     interface{} `json:"value"`
	Timestamp string      `json:"timestamp"`
}

// AnomalyResult represents detected anomalies
type AnomalyResult struct {
	Index  int         `json:"index"`
	Value  float64     `json:"value"`
	ZScore float64     `json:"z_score"`
	Item   interface{} `json:"item"`
}

// LiquidityAnalysis represents liquidity distribution analysis
type LiquidityAnalysis struct {
	TotalLiquidity   float64            `json:"total_liquidity"`
	PoolCount        int                `json:"pool_count"`
	AverageLiquidity float64            `json:"average_liquidity"`
	MedianLiquidity  float64            `json:"median_liquidity"`
	GiniCoefficient  float64            `json:"gini_coefficient"`
	TopPoolsShare    float64            `json:"top_pools_share"`
	Distribution     map[string]float64 `json:"distribution"`
}

// DexDistribution represents DEX distribution analysis
type DexDistribution struct {
	TotalVolume   float64            `json:"total_volume"`
	DexCount      int                `json:"dex_count"`
	Distribution  map[string]float64 `json:"distribution"`
	TopDexes      []string           `json:"top_dexes"`
	Concentration float64            `json:"concentration"`
}

// MarketOverview represents overall market statistics
type MarketOverview struct {
	SystemStats     SystemStats            `json:"system_stats"`
	NetworkOverview map[string]interface{} `json:"network_overview"`
	Timestamp       string                 `json:"timestamp"`
}

// AsyncResult represents the result of async operations
type AsyncResult struct {
	Network string      `json:"network,omitempty"`
	Query   string      `json:"query,omitempty"`
	Data    interface{} `json:"data"`
	Error   string      `json:"error,omitempty"`
}
