// Storage layer for persistence

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::errors::StorageError;
use crate::models::{Product, Transaction};

/// Trait defining storage operations for products and transactions
pub trait Storage {
    /// Save products to persistent storage
    fn save_products(&self, products: &[Product]) -> Result<(), StorageError>;
    
    /// Load products from persistent storage
    fn load_products(&self) -> Result<Vec<Product>, StorageError>;
    
    /// Save transactions to persistent storage
    fn save_transactions(&self, transactions: &[Transaction]) -> Result<(), StorageError>;
    
    /// Load transactions from persistent storage
    fn load_transactions(&self) -> Result<Vec<Transaction>, StorageError>;
}

/// JSON file-based storage implementation
pub struct JsonStorage {
    /// Path to the products JSON file
    products_path: PathBuf,
    /// Path to the transactions JSON file
    transactions_path: PathBuf,
}

impl JsonStorage {
    /// Create a new JsonStorage with the specified directory
    /// 
    /// Products will be stored in `{dir}/products.json`
    /// Transactions will be stored in `{dir}/transactions.json`
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        let dir = dir.as_ref();
        JsonStorage {
            products_path: dir.join("products.json"),
            transactions_path: dir.join("transactions.json"),
        }
    }
    
    /// Create a new JsonStorage with explicit file paths
    pub fn with_paths<P1: AsRef<Path>, P2: AsRef<Path>>(
        products_path: P1,
        transactions_path: P2,
    ) -> Self {
        JsonStorage {
            products_path: products_path.as_ref().to_path_buf(),
            transactions_path: transactions_path.as_ref().to_path_buf(),
        }
    }

    
    /// Read JSON data from a file, returning empty vector if file doesn't exist
    fn read_json_file<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Result<Vec<T>, StorageError> {
        match fs::read_to_string(path) {
            Ok(contents) => {
                // Handle empty files as empty vectors
                if contents.trim().is_empty() {
                    return Ok(Vec::new());
                }
                
                serde_json::from_str(&contents).map_err(|e| {
                    StorageError::ParseError(format!("Failed to parse {}: {}", path.display(), e))
                })
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // Missing files return empty vectors (Requirements 8.3)
                Ok(Vec::new())
            }
            Err(e) => Err(StorageError::ReadError(format!(
                "Failed to read {}: {}",
                path.display(),
                e
            ))),
        }
    }
    
    /// Write JSON data to a file
    fn write_json_file<T: serde::Serialize>(&self, path: &Path, data: &[T]) -> Result<(), StorageError> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    StorageError::WriteError(format!(
                        "Failed to create directory {}: {}",
                        parent.display(),
                        e
                    ))
                })?;
            }
        }
        
        let json = serde_json::to_string_pretty(data).map_err(|e| {
            StorageError::WriteError(format!("Failed to serialize data: {}", e))
        })?;
        
        fs::write(path, json).map_err(|e| {
            StorageError::WriteError(format!("Failed to write {}: {}", path.display(), e))
        })
    }
}

impl Storage for JsonStorage {
    fn save_products(&self, products: &[Product]) -> Result<(), StorageError> {
        self.write_json_file(&self.products_path, products)
    }
    
    fn load_products(&self) -> Result<Vec<Product>, StorageError> {
        self.read_json_file(&self.products_path)
    }
    
    fn save_transactions(&self, transactions: &[Transaction]) -> Result<(), StorageError> {
        self.write_json_file(&self.transactions_path, transactions)
    }
    
    fn load_transactions(&self) -> Result<Vec<Transaction>, StorageError> {
        self.read_json_file(&self.transactions_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::TempDir;
    use crate::models::TransactionType;
    
    fn create_test_product() -> Product {
        Product {
            id: "test-id-123".to_string(),
            sku: "SKU001".to_string(),
            name: "Test Product".to_string(),
            description: "A test product".to_string(),
            quantity: 100,
            reorder_point: 20,
        }
    }
    
    fn create_test_transaction() -> Transaction {
        Transaction {
            id: "txn-id-123".to_string(),
            product_sku: "SKU001".to_string(),
            transaction_type: TransactionType::Addition,
            quantity: 50,
            timestamp: Utc::now(),
            notes: Some("Test transaction".to_string()),
        }
    }
    
    #[test]
    fn test_save_and_load_products() {
        let temp_dir = TempDir::new().unwrap();
        let storage = JsonStorage::new(temp_dir.path());
        
        let products = vec![create_test_product()];
        storage.save_products(&products).unwrap();
        
        let loaded = storage.load_products().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].sku, "SKU001");
    }
    
    #[test]
    fn test_save_and_load_transactions() {
        let temp_dir = TempDir::new().unwrap();
        let storage = JsonStorage::new(temp_dir.path());
        
        let transactions = vec![create_test_transaction()];
        storage.save_transactions(&transactions).unwrap();
        
        let loaded = storage.load_transactions().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].product_sku, "SKU001");
    }
    
    #[test]
    fn test_load_missing_files_returns_empty() {
        let temp_dir = TempDir::new().unwrap();
        let storage = JsonStorage::new(temp_dir.path());
        
        // Files don't exist yet
        let products = storage.load_products().unwrap();
        assert!(products.is_empty());
        
        let transactions = storage.load_transactions().unwrap();
        assert!(transactions.is_empty());
    }
    
    #[test]
    fn test_corrupted_json_returns_parse_error() {
        let temp_dir = TempDir::new().unwrap();
        let storage = JsonStorage::new(temp_dir.path());
        
        // Write invalid JSON
        fs::write(temp_dir.path().join("products.json"), "not valid json {{{").unwrap();
        
        let result = storage.load_products();
        assert!(matches!(result, Err(StorageError::ParseError(_))));
    }
}
