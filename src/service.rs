// Business logic and inventory operations

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::models::{Product, Transaction, TransactionType};
use crate::storage::Storage;

/// Inventory service that manages products and transactions
pub struct InventoryService {
    /// Products indexed by SKU for fast lookup
    products: HashMap<String, Product>,
    /// All stock transactions
    transactions: Vec<Transaction>,
    /// Storage backend for persistence
    storage: Box<dyn Storage>,
}

impl InventoryService {
    /// Create a new InventoryService, loading existing data from storage
    /// 
    /// # Requirements
    /// - 8.1: Load all product and transaction data from persistent storage on startup
    pub fn new(storage: Box<dyn Storage>) -> Result<Self, ServiceError> {
        // Load products from storage and index by SKU
        let products_vec = storage.load_products()?;
        let products: HashMap<String, Product> = products_vec
            .into_iter()
            .map(|p| (p.sku.clone(), p))
            .collect();
        
        // Load transactions from storage
        let transactions = storage.load_transactions()?;
        
        Ok(InventoryService {
            products,
            transactions,
            storage,
        })
    }

    /// Add a new product to the inventory
    /// 
    /// # Requirements
    /// - 1.1: Create a new product record with provided details
    /// - 1.2: Reject duplicate SKU
    /// - 1.3: Validate SKU is non-empty and quantity is non-negative
    /// - 1.4: Persist product data to storage
    /// - 1.5: Return complete product record with unique identifier
    pub fn add_product(
        &mut self,
        sku: String,
        name: String,
        description: String,
        initial_quantity: u32,
        reorder_point: u32,
    ) -> Result<Product, ServiceError> {
        // Validate SKU is non-empty
        if sku.trim().is_empty() {
            return Err(ServiceError::InvalidInput("SKU cannot be empty".to_string()));
        }
        
        // Validate name is non-empty
        if name.trim().is_empty() {
            return Err(ServiceError::InvalidInput("Name cannot be empty".to_string()));
        }
        
        // Check for duplicate SKU
        if self.products.contains_key(&sku) {
            return Err(ServiceError::DuplicateSKU(sku));
        }
        
        // Create new product with UUID
        let product = Product {
            id: Uuid::new_v4().to_string(),
            sku: sku.clone(),
            name,
            description,
            quantity: initial_quantity,
            reorder_point,
        };
        
        // Insert into HashMap
        self.products.insert(sku, product.clone());
        
        // Persist to storage
        self.persist_products()?;
        
        Ok(product)
    }

    /// Update an existing product's details
    /// 
    /// # Requirements
    /// - 2.1: Modify existing product record
    /// - 2.2: Return error for non-existent product
    /// - 2.3: Validate all provided fields
    /// - 2.4: Persist changes to storage
    pub fn update_product(
        &mut self,
        sku: &str,
        name: Option<String>,
        description: Option<String>,
        reorder_point: Option<u32>,
    ) -> Result<Product, ServiceError> {
        // Check product exists
        let product = self.products.get_mut(sku)
            .ok_or_else(|| ServiceError::ProductNotFound(sku.to_string()))?;
        
        // Validate and update name if provided
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(ServiceError::InvalidInput("Name cannot be empty".to_string()));
            }
            product.name = new_name;
        }
        
        // Update description if provided
        if let Some(new_description) = description {
            product.description = new_description;
        }
        
        // Update reorder_point if provided
        if let Some(new_reorder_point) = reorder_point {
            product.reorder_point = new_reorder_point;
        }
        
        let updated_product = product.clone();
        
        // Persist to storage
        self.persist_products()?;
        
        Ok(updated_product)
    }

    /// Get a product by SKU
    /// 
    /// # Requirements
    /// - 5.1: Return current stock level and product details
    pub fn get_product(&self, sku: &str) -> Result<&Product, ServiceError> {
        self.products.get(sku)
            .ok_or_else(|| ServiceError::ProductNotFound(sku.to_string()))
    }

    /// List all products in the inventory
    /// 
    /// # Requirements
    /// - 5.2: Return list of all products with current stock levels
    pub fn list_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    /// Delete a product and all its associated transactions
    /// 
    /// # Requirements
    /// - 7.1: Remove product record from storage
    /// - 7.2: Return error for non-existent product
    /// - 7.3: Remove all associated transaction records
    /// - 7.4: Confirm deletion
    /// - 7.5: Allow deletion of products with current stock
    pub fn delete_product(&mut self, sku: &str) -> Result<(), ServiceError> {
        // Check product exists
        if !self.products.contains_key(sku) {
            return Err(ServiceError::ProductNotFound(sku.to_string()));
        }
        
        // Remove product from HashMap
        self.products.remove(sku);
        
        // Remove all associated transactions
        self.transactions.retain(|t| t.product_sku != sku);
        
        // Persist changes to storage
        self.persist_products()?;
        self.persist_transactions()?;
        
        Ok(())
    }

    /// Persist products to storage
    fn persist_products(&self) -> Result<(), ServiceError> {
        let products: Vec<Product> = self.products.values().cloned().collect();
        self.storage.save_products(&products)?;
        Ok(())
    }

    /// Persist transactions to storage
    fn persist_transactions(&self) -> Result<(), ServiceError> {
        self.storage.save_transactions(&self.transactions)?;
        Ok(())
    }

    /// Add stock to an existing product
    /// 
    /// # Requirements
    /// - 3.1: Increase product's stock level by the specified amount
    /// - 3.2: Reject negative quantity
    /// - 3.3: Create transaction record with timestamp, type, quantity, and notes
    /// - 3.4: Persist both updated stock level and transaction record
    /// - 3.5: Return error for non-existent product
    pub fn add_stock(
        &mut self,
        sku: &str,
        quantity: u32,
        notes: Option<String>,
    ) -> Result<(), ServiceError> {
        // Validate quantity is positive (non-zero)
        if quantity == 0 {
            return Err(ServiceError::InvalidInput("Quantity must be positive".to_string()));
        }

        // Validate product exists
        let product = self.products.get_mut(sku)
            .ok_or_else(|| ServiceError::ProductNotFound(sku.to_string()))?;

        // Increase product quantity
        product.quantity += quantity;

        // Create transaction record
        let transaction = Transaction {
            id: Uuid::new_v4().to_string(),
            product_sku: sku.to_string(),
            transaction_type: TransactionType::Addition,
            quantity,
            timestamp: Utc::now(),
            notes,
        };

        // Add transaction to vector
        self.transactions.push(transaction);

        // Persist both products and transactions
        self.persist_products()?;
        self.persist_transactions()?;

        Ok(())
    }

    /// Remove stock from an existing product
    /// 
    /// # Requirements
    /// - 4.1: Decrease product's stock level by the specified amount
    /// - 4.2: Reject removal exceeding available quantity
    /// - 4.3: Create transaction record with timestamp, type, quantity, and notes
    /// - 4.4: Persist both updated stock level and transaction record
    pub fn remove_stock(
        &mut self,
        sku: &str,
        quantity: u32,
        notes: Option<String>,
    ) -> Result<(), ServiceError> {
        // Validate quantity is positive (non-zero)
        if quantity == 0 {
            return Err(ServiceError::InvalidInput("Quantity must be positive".to_string()));
        }

        // Validate product exists and get current quantity
        let product = self.products.get(sku)
            .ok_or_else(|| ServiceError::ProductNotFound(sku.to_string()))?;

        // Validate sufficient stock available
        if quantity > product.quantity {
            return Err(ServiceError::InsufficientStock {
                sku: sku.to_string(),
                requested: quantity,
                available: product.quantity,
            });
        }

        // Now get mutable reference and decrease quantity
        let product = self.products.get_mut(sku).unwrap();
        product.quantity -= quantity;

        // Create transaction record
        let transaction = Transaction {
            id: Uuid::new_v4().to_string(),
            product_sku: sku.to_string(),
            transaction_type: TransactionType::Removal,
            quantity,
            timestamp: Utc::now(),
            notes,
        };

        // Add transaction to vector
        self.transactions.push(transaction);

        // Persist both products and transactions
        self.persist_products()?;
        self.persist_transactions()?;

        Ok(())
    }

    /// List all products with low stock (quantity at or below reorder point)
    /// 
    /// # Requirements
    /// - 4.5: Flag products for reorder when stock falls below reorder point
    /// - 5.3: Return all products where current stock level is at or below reorder point
    pub fn list_low_stock(&self) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| p.quantity <= p.reorder_point)
            .collect()
    }

    /// Get transaction history for a product, ordered by timestamp
    /// 
    /// # Requirements
    /// - 6.1: Return all transactions for a product ordered by timestamp
    /// - 6.2: Include transaction type, quantity, timestamp, and notes
    /// - 6.5: Preserve chronological order of all stock movements
    pub fn get_transactions(&self, sku: &str) -> Vec<&Transaction> {
        let mut transactions: Vec<&Transaction> = self.transactions
            .iter()
            .filter(|t| t.product_sku == sku)
            .collect();
        
        // Sort by timestamp ascending (earliest to latest)
        transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        transactions
    }

    /// Get transaction history for a product within a date range, ordered by timestamp
    /// 
    /// # Requirements
    /// - 6.3: Return only transactions within the specified period
    pub fn get_transactions_in_range(
        &self,
        sku: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&Transaction> {
        let mut transactions: Vec<&Transaction> = self.transactions
            .iter()
            .filter(|t| {
                t.product_sku == sku 
                    && t.timestamp >= start 
                    && t.timestamp <= end
            })
            .collect();
        
        // Sort by timestamp ascending (earliest to latest)
        transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        transactions
    }
}
