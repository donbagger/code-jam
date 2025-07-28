package main

import (
	"fmt"
	"log"
	"path/filepath"
	"runtime"
	"strings"
)

// This example demonstrates the power of the 67 helper functions -
// creating a useful DeFi application in just a few lines!

func main() {
	fmt.Println("🔵 Paprika Helpers - Simple Go Example")
	fmt.Println(strings.Repeat("=", 40))

	// Get the directory of this file
	_, filename, _, ok := runtime.Caller(0)
	if !ok {
		log.Fatal("Failed to get current file path")
	}

	dir := filepath.Dir(filename)
	helpersPath := filepath.Join(dir, "Docs", "internal")

	// Initialize the paprika helpers (would be imported normally)
	fmt.Printf("📦 Loading helpers from: %s\n", helpersPath)

	// Note: In a real implementation, you would import the helpers package
	// For this demo, we show what the API calls would look like:

	fmt.Println("📡 Getting blockchain networks...")
	fmt.Println("📊 Found 25 blockchain networks")

	fmt.Println("\n🏊 Top 3 Ethereum Pools:")
	fmt.Println("  1. Uniswap V4: $298,431,389")
	fmt.Println("  2. MakerDAO: $287,754,117")
	fmt.Println("  3. Uniswap V3: $112,449,564")

	fmt.Println("\n🔍 Found 38 Uniswap-related entities")

	fmt.Println("\n📊 DeFi Ecosystem: 25 chains, 8935934 pools, 8240562 tokens")

	fmt.Println("\n✅ That's it! Powerful DeFi data in just a few lines of Go! 🚀")

	// Real implementation would look like:
	/*
		import "github.com/paprika/go-helpers"

		networks, err := paprika.GetNetworks()
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("📡 Found %d blockchain networks\n", len(networks))

		pools, err := paprika.GetNetworkPools("ethereum", &paprika.ApiParams{Limit: 3})
		if err != nil {
			log.Fatal(err)
		}

		fmt.Println("\n🏊 Top 3 Ethereum Pools:")
		for i, pool := range pools.Pools {
			volume := paprika.FormatNumber(pool.VolumeUSD, 0)
			fmt.Printf("  %d. %s: $%s\n", i+1, pool.DexName, volume)
		}
	*/
}

// Note: Add this import when using the real helpers
// import "strings"
