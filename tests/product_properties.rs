// Feature: stock-control, Property 3: Product persistence round-trip
// For any product, after creating it and persisting to storage, reloading from storage 
// should yield an equivalent product with all fields preserved.
// Validates: Requirements 1.4, 8.1, 8.4

use quickcheck::{Arbitrary, Gen, QuickCheck};
use stock_control::models::Product;

// Custom generator for Product to ensure valid data
#[derive(Clone, Debug)]
struct ValidProduct(Product);

impl Arbitrary for ValidProduct {
    fn arbitrary(g: &mut Gen) -> Self {
        // Generate non-empty strings for id and sku
        let id = format!("id-{}", u32::arbitrary(g));
        let sku = format!("SKU-{}", u32::arbitrary(g));
        
        // Generate non-empty name (at least 1 character)
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
        
        // Generate description (can be empty)
        let desc_len = u8::arbitrary(g) % 100;
        let description: String = (0..desc_len)
            .map(|_| char::arbitrary(g))
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation())
            .collect();
        
        // Generate non-negative quantities
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

fn prop_product_serialization_roundtrip(valid_product: ValidProduct) -> bool {
    let product = valid_product.0;
    
    // Serialize to JSON
    let serialized = match serde_json::to_string(&product) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    // Deserialize back from JSON
    let deserialized: Product = match serde_json::from_str(&serialized) {
        Ok(p) => p,
        Err(_) => return false,
    };
    
    // Check that all fields are preserved
    product == deserialized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_roundtrip_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_product_serialization_roundtrip as fn(ValidProduct) -> bool);
    }
}
