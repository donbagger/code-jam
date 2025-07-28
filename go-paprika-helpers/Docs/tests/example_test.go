package paprikahelpers

import (
	"context"
	"testing"
	"time"
)

func TestGetNetworks(t *testing.T) {
	networks, err := GetNetworks()
	if err != nil {
		t.Fatalf("GetNetworks failed: %v", err)
	}

	if len(networks) == 0 {
		t.Fatal("No networks returned")
	}

	t.Logf("Found %d networks", len(networks))

	// Check first network has required fields
	if networks[0].ID == "" {
		t.Error("Network ID is empty")
	}

	if networks[0].DisplayName == "" {
		t.Error("Network DisplayName is empty")
	}

	t.Logf("First network: %s (%s)", networks[0].DisplayName, networks[0].ID)
}

func TestGetNetworkPools(t *testing.T) {
	params := map[string]string{"limit": "5"}

	response, err := GetNetworkPools("ethereum", params)
	if err != nil {
		t.Fatalf("GetNetworkPools failed: %v", err)
	}

	if response == nil {
		t.Fatal("Response is nil")
	}

	if len(response.Pools) == 0 {
		t.Fatal("No pools returned")
	}

	t.Logf("Found %d pools", len(response.Pools))
	t.Logf("Page info: limit=%d, page=%d, total=%d",
		response.PageInfo.Limit, response.PageInfo.Page, response.PageInfo.TotalItems)

	// Check first pool has required fields
	pool := response.Pools[0]
	if pool.ID == "" {
		t.Error("Pool ID is empty")
	}

	if pool.DexName == "" {
		t.Error("Pool DexName is empty")
	}

	t.Logf("First pool: %s on %s (Volume: $%.2f)", pool.ID, pool.DexName, pool.VolumeUSD)
}

func TestGetSystemStats(t *testing.T) {
	stats, err := GetSystemStats()
	if err != nil {
		t.Fatalf("GetSystemStats failed: %v", err)
	}

	if stats == nil {
		t.Fatal("Stats is nil")
	}

	if stats.Chains == 0 {
		t.Error("Chains count is 0")
	}

	t.Logf("System stats: %d chains, %d factories, %d pools, %d tokens",
		stats.Chains, stats.Factories, stats.Pools, stats.Tokens)
}

func TestSearchEntities(t *testing.T) {
	response, err := SearchEntities("uniswap")
	if err != nil {
		t.Fatalf("SearchEntities failed: %v", err)
	}

	if response == nil {
		t.Fatal("Search response is nil")
	}

	totalResults := len(response.Tokens) + len(response.Pools) + len(response.Dexes)
	t.Logf("Search results: %d tokens, %d pools, %d dexes (total: %d)",
		len(response.Tokens), len(response.Pools), len(response.Dexes), totalResults)

	if totalResults == 0 {
		t.Error("No search results found")
	}
}

func TestExtractPools(t *testing.T) {
	// Get some pool data first
	response, err := GetNetworkPools("ethereum", map[string]string{"limit": "3"})
	if err != nil {
		t.Fatalf("Failed to get pools: %v", err)
	}

	// Test extraction
	pools := ExtractPools(response)
	if len(pools) == 0 {
		t.Error("ExtractPools returned no pools")
	}

	t.Logf("Extracted %d pools", len(pools))
}

func TestFilterByVolume(t *testing.T) {
	// Get some pool data first
	response, err := GetNetworkPools("ethereum", map[string]string{"limit": "10"})
	if err != nil {
		t.Fatalf("Failed to get pools: %v", err)
	}

	// Test filtering
	filtered := FilterByVolume(response.Pools, 1000000) // $1M minimum

	t.Logf("Filtered %d pools with volume >= $1M (from %d total)",
		len(filtered), len(response.Pools))

	// Verify all filtered pools meet the criteria
	for _, pool := range filtered {
		if pool.VolumeUSD < 1000000 {
			t.Errorf("Pool %s has volume $%.2f, below $1M threshold", pool.ID, pool.VolumeUSD)
		}
	}
}

func TestTopN(t *testing.T) {
	// Get some pool data first
	response, err := GetNetworkPools("ethereum", map[string]string{"limit": "10"})
	if err != nil {
		t.Fatalf("Failed to get pools: %v", err)
	}

	// Test top N
	top3 := TopN(response.Pools, "volume_usd", 3)

	if len(top3) > 3 {
		t.Errorf("TopN returned %d pools, expected max 3", len(top3))
	}

	t.Logf("Top 3 pools by volume:")
	for i, pool := range top3 {
		t.Logf("  %d. %s: $%.2f", i+1, pool.DexName, pool.VolumeUSD)
	}
}

func TestFormatNumber(t *testing.T) {
	tests := []struct {
		input    float64
		decimals int
		expected string
	}{
		{1234567.89, 2, "1,234,567.89"},
		{1000, 0, "1,000"},
		{999.5, 1, "999.5"},
	}

	for _, test := range tests {
		result := FormatNumber(test.input, test.decimals)
		if result != test.expected {
			t.Errorf("FormatNumber(%.2f, %d) = %s, expected %s",
				test.input, test.decimals, result, test.expected)
		}
	}
}

func TestFormatPercentage(t *testing.T) {
	result := FormatPercentage(15.5678, 2)
	expected := "15.57%"

	if result != expected {
		t.Errorf("FormatPercentage(15.5678, 2) = %s, expected %s", result, expected)
	}
}

func TestCalculatePriceChange(t *testing.T) {
	change := CalculatePriceChange(110, 100)
	expected := 10.0

	if change != expected {
		t.Errorf("CalculatePriceChange(110, 100) = %.2f, expected %.2f", change, expected)
	}
}

func TestValidateTokenAddress(t *testing.T) {
	tests := []struct {
		address string
		valid   bool
	}{
		{"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", true},  // Ethereum USDC
		{"0xInvalidAddress", false},                           // Invalid
		{"So11111111111111111111111111111111111111112", true}, // Solana SOL
		{"InvalidSolanaAddress", false},                       // Invalid
	}

	for _, test := range tests {
		result := ValidateTokenAddress(test.address)
		if result != test.valid {
			t.Errorf("ValidateTokenAddress(%s) = %t, expected %t",
				test.address, result, test.valid)
		}
	}
}

func TestAsyncGetMultiplePools(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	networks := []string{"ethereum", "polygon"}
	results, err := AsyncGetMultiplePools(ctx, networks, 2)

	if err != nil {
		t.Fatalf("AsyncGetMultiplePools failed: %v", err)
	}

	if len(results) != len(networks) {
		t.Errorf("Expected results for %d networks, got %d", len(networks), len(results))
	}

	for _, network := range networks {
		if _, exists := results[network]; !exists {
			t.Errorf("No results for network %s", network)
		} else {
			t.Logf("Got results for network %s", network)
		}
	}
}

func TestGetMarketOverview(t *testing.T) {
	overview, err := GetMarketOverview()
	if err != nil {
		t.Fatalf("GetMarketOverview failed: %v", err)
	}

	if overview == nil {
		t.Fatal("Overview is nil")
	}

	if overview.SystemStats.Chains == 0 {
		t.Error("No chains in overview")
	}

	if len(overview.NetworkOverview) == 0 {
		t.Error("No network overview data")
	}

	t.Logf("Market overview: %d chains, %d networks analyzed",
		overview.SystemStats.Chains, len(overview.NetworkOverview))
}

// Benchmark test
func BenchmarkGetNetworks(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_, err := GetNetworks()
		if err != nil {
			b.Fatalf("GetNetworks failed: %v", err)
		}
	}
}
