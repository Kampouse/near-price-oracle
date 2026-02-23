use near_sdk::{env, near, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use borsh::BorshSchema;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Price data from a single source
#[derive(BorshSerialize, BorshDeserialize, BorshSchema, JsonSchema, Serialize, Deserialize, Clone)]
pub struct PriceReport {
    pub source: String,        // e.g., "coingecko", "binance", "coinmarketcap"
    pub price_usd: u128,       // Price in micro-dollars (multiply by 1e6 for actual USD)
    pub timestamp: u64,        // Unix timestamp
    pub reporter: String,      // Account that submitted the price
}

/// Main oracle state
#[near(contract_state)]
pub struct PriceOracle {
    owner: AccountId,
    prices: HashMap<String, PriceReport>,  // source -> latest price
    last_update: u64,
    min_sources: u8,          // Minimum sources required for valid price
}

impl Default for PriceOracle {
    fn default() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            prices: HashMap::new(),
            last_update: 0,
            min_sources: 3,
        }
    }
}

#[near]
impl PriceOracle {
    /// Initialize with custom min sources
    pub fn init(&mut self, min_sources: u8) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner");
        self.min_sources = min_sources;
    }

    /// Submit a price report from an external source
    /// price_usd should be in micro-dollars (e.g., $5.25 = 5250000)
    pub fn report_price(&mut self, source: String, price_usd: u128) {
        let reporter = env::predecessor_account_id();
        let timestamp = env::block_timestamp() / 1_000_000; // Convert from nanoseconds

        let report = PriceReport {
            source: source.clone(),
            price_usd,
            timestamp,
            reporter: reporter.to_string(),
        };

        let src = source.clone();
        self.prices.insert(source, report);
        self.last_update = timestamp;
        
        near_sdk::log!("Price reported: {} USD from {}", price_usd, src);
    }

    /// Get the aggregated NEAR price (average of all sources)
    /// Returns price in micro-dollars
    pub fn get_price(&self) -> u128 {
        assert!(
            self.prices.len() >= self.min_sources as usize,
            "Need at least {} price sources, have {}",
            self.min_sources,
            self.prices.len()
        );

        let total: u128 = self.prices.values().map(|p| p.price_usd).sum();
        let count = self.prices.len() as u128;
        
        total / count
    }

    /// Get detailed price info from all sources
    pub fn get_price_details(&self) -> Vec<PriceReport> {
        self.prices.values().cloned().collect()
    }

    /// Get number of price sources
    pub fn get_source_count(&self) -> u8 {
        self.prices.len() as u8
    }

    /// Check if we have enough sources for a valid price
    pub fn is_valid(&self) -> bool {
        self.prices.len() >= self.min_sources as usize
    }

    /// Get the last update timestamp
    pub fn get_last_update(&self) -> u64 {
        self.last_update
    }

    /// Get minimum sources required
    pub fn get_min_sources(&self) -> u8 {
        self.min_sources
    }

    /// Set minimum sources (owner only)
    pub fn set_min_sources(&mut self, min_sources: u8) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner");
        self.min_sources = min_sources;
    }

    /// Clear all prices (for reset)
    pub fn clear_prices(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner");
        self.prices.clear();
        self.last_update = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        VMContextBuilder::new()
    }

    #[test]
    fn test_initialization() {
        let context = get_context().build();
        testing_env!(context);
        
        let contract = PriceOracle::default();
        assert_eq!(contract.get_min_sources(), 3);
        assert_eq!(contract.get_source_count(), 0);
        assert!(!contract.is_valid());
    }

    #[test]
    fn test_report_price() {
        let context = get_context().build();
        testing_env!(context);
        
        let mut contract = PriceOracle::default();
        contract.report_price("coingecko".to_string(), 5250000); // $5.25
        
        assert_eq!(contract.get_source_count(), 1);
    }

    #[test]
    fn test_get_price_average() {
        let context = get_context().build();
        testing_env!(context);
        
        let mut contract = PriceOracle::default();
        contract.report_price("coingecko".to_string(), 5000000);  // $5.00
        contract.report_price("binance".to_string(), 5200000);    // $5.20
        contract.report_price("coinmarketcap".to_string(), 5400000); // $5.40
        
        assert_eq!(contract.get_source_count(), 3);
        assert!(contract.is_valid());
        
        // Average should be $5.20
        let price = contract.get_price();
        assert_eq!(price, 5200000);
    }
}
