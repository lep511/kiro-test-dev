// Data models for the Stock Control System

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a product in the inventory system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    /// Unique identifier (UUID)
    pub id: String,
    /// Stock Keeping Unit - unique identifier for the product
    pub sku: String,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Current stock level
    pub quantity: u32,
    /// Minimum stock level that triggers reorder alert
    pub reorder_point: u32,
}

/// Type of stock transaction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    /// Stock addition
    Addition,
    /// Stock removal
    Removal,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Addition => write!(f, "Addition"),
            TransactionType::Removal => write!(f, "Removal"),
        }
    }
}

/// Represents a stock transaction (addition or removal)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Unique identifier (UUID)
    pub id: String,
    /// SKU of the product this transaction affects
    pub product_sku: String,
    /// Type of transaction (Addition or Removal)
    pub transaction_type: TransactionType,
    /// Amount of stock added or removed
    pub quantity: u32,
    /// When the transaction occurred
    pub timestamp: DateTime<Utc>,
    /// Optional notes about the transaction
    pub notes: Option<String>,
}
