// Feature: stock-control, Property 25: Storage serialization round-trip
// For any valid system state (products and transactions), serializing to JSON and 
// deserializing should produce an equivalent state with all data preserved.
// Validates: Requirements 8.4

use quickcheck::{Arbitrary, Gen, QuickCheck};
use stock_control::models::{Product, Transaction, TransactionType};
use chrono::{TimeZone, Utc};

// Custom generator for valid system state
#[derive(Clone, Debug)]
struct ValidSystemState {
    products: Vec<Product>,
    transactions: Vec<Transaction>,
}

// Custom generator for Product to ensure valid data
#[derive(Clone, Debug)]
struct ValidProduct(Product);

impl Arbitrary for ValidProduct {
    fn arbitrary(g: &mut Gen) -> Self {
        let id = format!("id-{}", u32::arbitrary(g));
        let sku = format!("SKU-{}", u32::arbitrary(g));
        
        let name_len = (u8::arbitrary(g) % 50) + 1;
        let name: String = (0..name_len)
            .map(|_| char::arbitrary(g))
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect();
        let name = if name.trim().is_empty() {
            "Product".to_string()
        } else {
            name
        };
        
        let desc_len = u8::arbitrary(g) % 100;
        let description: String = (0..desc_len)
            .map(|_| char::arbitrary(g))
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation())
            .collect();
        
        let quantity = u32::arbitrary(g);
        let reorder_point = u32::arbitrary(g);
        
        ValidProduct(Product {
            id,
            sku,
            name,
            description,
            quantity,
            reorder_point,
        })
    }
}

// Custom generator for Transaction to ensure valid data
#[derive(Clone, Debug)]
struct ValidTransaction(Transaction);

impl Arbitrary for ValidTransaction {
    fn arbitrary(g: &mut Gen) -> Self {
        let id = format!("txn-{}", u32::arbitrary(g));
        let product_sku = format!("SKU-{}", u32::arbitrary(g));
        
        let transaction_type = if bool::arbitrary(g) {
            TransactionType::Addition
        } else {
            TransactionType::Removal
        };
        
        let quantity = u32::arbitrary(g);
        
        // Generate a valid timestamp (use a reasonable range to avoid edge cases)
        let timestamp_secs = (i64::arbitrary(g) % 1_000_000_000).abs();
        let timestamp = Utc.timestamp_opt(timestamp_secs, 0).unwrap();
        
        let notes = if bool::arbitrary(g) {
            let note_len = u8::arbitrary(g) % 50;
            let note: String = (0..note_len)
                .map(|_| char::arbitrary(g))
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect();
            if note.is_empty() {
                None
            } else {
                Some(note)
            }
        } else {
            None
        };
        
        ValidTransaction(Transaction {
            id,
            product_sku,
            transaction_type,
            quantity,
            timestamp,
            notes,
        })
    }
}

impl Arbitrary for ValidSystemState {
    fn arbitrary(g: &mut Gen) -> Self {
        // Generate 0-10 products
        let product_count = (u8::arbitrary(g) % 11) as usize;
        let products: Vec<Product> = (0..product_count)
            .map(|_| ValidProduct::arbitrary(g).0)
            .collect();
        
        // Generate 0-20 transactions
        let transaction_count = (u8::arbitrary(g) % 21) as usize;
        let transactions: Vec<Transaction> = (0..transaction_count)
            .map(|_| ValidTransaction::arbitrary(g).0)
            .collect();
        
        ValidSystemState {
            products,
            transactions,
        }
    }
}

fn prop_storage_serialization_roundtrip(state: ValidSystemState) -> bool {
    // Serialize products to JSON
    let products_json = match serde_json::to_string(&state.products) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    // Serialize transactions to JSON
    let transactions_json = match serde_json::to_string(&state.transactions) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    // Deserialize products back from JSON
    let deserialized_products: Vec<Product> = match serde_json::from_str(&products_json) {
        Ok(p) => p,
        Err(_) => return false,
    };
    
    // Deserialize transactions back from JSON
    let deserialized_transactions: Vec<Transaction> = match serde_json::from_str(&transactions_json) {
        Ok(t) => t,
        Err(_) => return false,
    };
    
    // Check that all data is preserved
    state.products == deserialized_products && state.transactions == deserialized_transactions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_roundtrip_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_storage_serialization_roundtrip as fn(ValidSystemState) -> bool);
    }
}
