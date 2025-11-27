// Error types for the application

use std::fmt;

/// Errors that can occur in storage operations
#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    /// Failed to read from storage
    ReadError(String),
    /// Failed to write to storage
    WriteError(String),
    /// Failed to parse/deserialize data
    ParseError(String),
    /// Storage file not found
    FileNotFound(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::ReadError(msg) => write!(f, "Failed to read from storage: {}", msg),
            StorageError::WriteError(msg) => write!(f, "Failed to write to storage: {}", msg),
            StorageError::ParseError(msg) => write!(f, "Failed to parse storage data: {}", msg),
            StorageError::FileNotFound(msg) => write!(f, "Storage file not found: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

/// Errors that can occur in service operations
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceError {
    /// Product with the given SKU was not found
    ProductNotFound(String),
    /// Product with the given SKU already exists
    DuplicateSKU(String),
    /// Invalid input provided (e.g., empty SKU, negative quantity)
    InvalidInput(String),
    /// Insufficient stock for the requested operation
    InsufficientStock { sku: String, requested: u32, available: u32 },
    /// Storage operation failed
    StorageError(StorageError),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::ProductNotFound(sku) => write!(f, "Product not found: {}", sku),
            ServiceError::DuplicateSKU(sku) => write!(f, "Product with SKU '{}' already exists", sku),
            ServiceError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ServiceError::InsufficientStock { sku, requested, available } => {
                write!(f, "Insufficient stock for product '{}': requested {}, available {}", sku, requested, available)
            }
            ServiceError::StorageError(err) => write!(f, "Storage error: {}", err),
        }
    }
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceError::StorageError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(err: StorageError) -> Self {
        ServiceError::StorageError(err)
    }
}
