# Stock Control System - Design Document

## Overview

The Stock Control System is a command-line Rust application that provides comprehensive inventory management capabilities. The system follows a layered architecture with clear separation between data models, business logic, storage, and user interface. The application uses JSON for data persistence and provides a simple CLI for user interaction.

## Architecture

The system follows a modular architecture with these layers:

1. **Data Layer**: Core data structures (Product, Transaction, TransactionType)
2. **Storage Layer**: Handles persistence using JSON files
3. **Service Layer**: Business logic for inventory operations
4. **CLI Layer**: Command-line interface for user interaction

```
┌─────────────────────────────────┐
│         CLI Interface           │
│  (Command parsing & display)    │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│      Service Layer              │
│  (Business logic & validation)  │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│      Storage Layer              │
│   (JSON persistence)            │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│       Data Models               │
│  (Product, Transaction)         │
└─────────────────────────────────┘
```

## Components and Interfaces

### Data Models

**Product**
```rust
struct Product {
    id: String,           // Unique identifier (UUID)
    sku: String,          // Stock Keeping Unit
    name: String,         // Product name
    description: String,  // Product description
    quantity: u32,        // Current stock level
    reorder_point: u32,   // Minimum stock level
}
```

**Transaction**
```rust
struct Transaction {
    id: String,              // Unique identifier (UUID)
    product_sku: String,     // Associated product SKU
    transaction_type: TransactionType,
    quantity: u32,           // Amount added or removed
    timestamp: DateTime<Utc>, // When transaction occurred
    notes: Option<String>,   // Optional notes
}

enum TransactionType {
    Addition,
    Removal,
}
```

### Storage Interface

```rust
trait Storage {
    fn save_products(&self, products: &[Product]) -> Result<(), StorageError>;
    fn load_products(&self) -> Result<Vec<Product>, StorageError>;
    fn save_transactions(&self, transactions: &[Transaction]) -> Result<(), StorageError>;
    fn load_transactions(&self) -> Result<Vec<Transaction>, StorageError>;
}
```

**JsonStorage Implementation**
- Stores products in `products.json`
- Stores transactions in `transactions.json`
- Uses `serde_json` for serialization/deserialization

### Service Interface

```rust
struct InventoryService {
    products: HashMap<String, Product>,  // Keyed by SKU
    transactions: Vec<Transaction>,
    storage: Box<dyn Storage>,
}

impl InventoryService {
    fn add_product(&mut self, sku: String, name: String, description: String, 
                   initial_quantity: u32, reorder_point: u32) -> Result<Product, ServiceError>;
    
    fn update_product(&mut self, sku: &str, name: Option<String>, 
                      description: Option<String>, reorder_point: Option<u32>) -> Result<Product, ServiceError>;
    
    fn add_stock(&mut self, sku: &str, quantity: u32, notes: Option<String>) -> Result<(), ServiceError>;
    
    fn remove_stock(&mut self, sku: &str, quantity: u32, notes: Option<String>) -> Result<(), ServiceError>;
    
    fn get_product(&self, sku: &str) -> Result<&Product, ServiceError>;
    
    fn list_products(&self) -> Vec<&Product>;
    
    fn list_low_stock(&self) -> Vec<&Product>;
    
    fn get_transactions(&self, sku: &str) -> Vec<&Transaction>;
    
    fn get_transactions_in_range(&self, sku: &str, start: DateTime<Utc>, 
                                 end: DateTime<Utc>) -> Vec<&Transaction>;
    
    fn delete_product(&mut self, sku: &str) -> Result<(), ServiceError>;
}
```

### CLI Interface

The CLI provides commands for all inventory operations:
- `add-product` - Add a new product
- `update-product` - Update product details
- `add-stock` - Record stock addition
- `remove-stock` - Record stock removal
- `view-product` - View product details
- `list-products` - List all products
- `low-stock` - List products below reorder point
- `history` - View transaction history
- `delete-product` - Remove a product

## Data Models

### Product Entity
- **id**: UUID v4 generated at creation
- **sku**: User-provided, must be unique and non-empty
- **name**: User-provided, must be non-empty
- **description**: User-provided, can be empty
- **quantity**: Non-negative integer, updated by transactions
- **reorder_point**: Non-negative integer, threshold for low-stock alerts

### Transaction Entity
- **id**: UUID v4 generated at creation
- **product_sku**: References the product this transaction affects
- **transaction_type**: Either Addition or Removal
- **quantity**: Positive integer representing amount changed
- **timestamp**: UTC timestamp, set at transaction creation
- **notes**: Optional string for additional context

### Storage Format
Products and transactions are stored as JSON arrays in separate files:

```json
// products.json
[
  {
    "id": "uuid",
    "sku": "SKU001",
    "name": "Product Name",
    "description": "Description",
    "quantity": 100,
    "reorder_point": 20
  }
]

// transactions.json
[
  {
    "id": "uuid",
    "product_sku": "SKU001",
    "transaction_type": "Addition",
    "quantity": 50,
    "timestamp": "2025-11-24T10:30:00Z",
    "notes": "Initial stock"
  }
]
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Product Management Properties

**Property 1: Product creation with valid inputs**
*For any* valid product details (non-empty SKU, non-empty name, description, non-negative quantity, non-negative reorder point), creating a product should succeed and return a product record containing all the provided details with a unique identifier.
**Validates: Requirements 1.1, 1.3, 1.5**

**Property 2: Duplicate SKU rejection**
*For any* product that exists in the system, attempting to create another product with the same SKU should fail with an error.
**Validates: Requirements 1.2**

**Property 3: Product persistence round-trip**
*For any* product, after creating it and persisting to storage, reloading from storage should yield an equivalent product with all fields preserved.
**Validates: Requirements 1.4, 8.1, 8.4**

**Property 4: Product update modifies fields**
*For any* existing product and valid update fields, updating the product should result in the specified fields being changed while other fields remain unchanged.
**Validates: Requirements 2.1, 2.3**

**Property 5: Non-existent product update fails**
*For any* SKU that does not exist in the system, attempting to update that product should fail with an error.
**Validates: Requirements 2.2**

**Property 6: Product updates preserve transaction history**
*For any* product with existing transactions, updating the product's name or description should not affect the count or content of its transaction history.
**Validates: Requirements 2.5**

**Property 7: Product retrieval returns complete data**
*For any* product in the system, querying by SKU should return the product with all fields (SKU, name, description, quantity, reorder point) correctly populated.
**Validates: Requirements 5.1, 5.4**

**Property 8: List all products returns complete set**
*For any* set of products in the system, listing all products should return exactly that set with all products containing complete data.
**Validates: Requirements 5.2**

**Property 9: Product deletion removes data**
*For any* product in the system, deleting it by SKU should result in that product no longer being retrievable and all associated transactions being removed.
**Validates: Requirements 7.1, 7.3, 7.4**

**Property 10: Non-existent product deletion fails**
*For any* SKU that does not exist in the system, attempting to delete that product should fail with an error.
**Validates: Requirements 7.2**

**Property 11: Products with stock can be deleted**
*For any* product with non-zero stock quantity, deletion should succeed and remove all product data.
**Validates: Requirements 7.5**

### Stock Transaction Properties

**Property 12: Stock addition increases quantity**
*For any* product and positive addition quantity, adding stock should increase the product's quantity by exactly that amount.
**Validates: Requirements 3.1**

**Property 13: Negative stock addition rejected**
*For any* product and negative quantity, attempting to add stock should fail with an error and leave the product quantity unchanged.
**Validates: Requirements 3.2**

**Property 14: Stock addition creates transaction record**
*For any* stock addition, a transaction record should be created with type Addition, the correct quantity, a timestamp, and any provided notes.
**Validates: Requirements 3.3**

**Property 15: Stock addition to non-existent product fails**
*For any* SKU that does not exist in the system, attempting to add stock should fail with an error.
**Validates: Requirements 3.5**

**Property 16: Stock removal decreases quantity**
*For any* product and removal quantity not exceeding current stock, removing stock should decrease the product's quantity by exactly that amount.
**Validates: Requirements 4.1**

**Property 17: Insufficient stock removal rejected**
*For any* product, attempting to remove more stock than the current quantity should fail with an error and leave the quantity unchanged.
**Validates: Requirements 4.2**

**Property 18: Stock removal creates transaction record**
*For any* stock removal, a transaction record should be created with type Removal, the correct quantity, a timestamp, and any provided notes.
**Validates: Requirements 4.3**

**Property 19: Stock transaction persistence round-trip**
*For any* stock transaction (addition or removal), after persisting to storage and reloading, both the updated stock level and the transaction record should be preserved correctly.
**Validates: Requirements 3.4, 4.4, 8.2**

**Property 20: Low stock detection after removal**
*For any* product, if removing stock causes the quantity to fall to or below the reorder point, the product should be identifiable as low-stock.
**Validates: Requirements 4.5**

### Query and Filtering Properties

**Property 21: Low-stock filter returns correct products**
*For any* set of products in the system, querying for low-stock products should return exactly those products where quantity is at or below the reorder point.
**Validates: Requirements 5.3**

**Property 22: Transaction history ordered chronologically**
*For any* product with multiple transactions, retrieving the transaction history should return all transactions ordered by timestamp from earliest to latest.
**Validates: Requirements 6.1, 6.5**

**Property 23: Transaction records contain complete data**
*For any* transaction, the record should include transaction type, quantity, timestamp, and notes (if provided).
**Validates: Requirements 6.2**

**Property 24: Date range filtering returns correct transactions**
*For any* product and date range, querying transactions should return only those transactions with timestamps within the specified range (inclusive).
**Validates: Requirements 6.3**

### Data Integrity Properties

**Property 25: Storage serialization round-trip**
*For any* valid system state (products and transactions), serializing to JSON and deserializing should produce an equivalent state with all data preserved.
**Validates: Requirements 8.4**

**Property 26: Graceful handling of invalid storage data**
*For any* corrupted or malformed JSON data, attempting to deserialize should either succeed with valid data or fail gracefully with an error without crashing.
**Validates: Requirements 8.5**

## Error Handling

The system defines custom error types for different failure scenarios:

**ServiceError**
- `ProductNotFound` - SKU does not exist
- `DuplicateSKU` - SKU already exists
- `InvalidInput` - Validation failed (empty SKU, negative quantity, etc.)
- `InsufficientStock` - Removal exceeds available quantity
- `StorageError` - Persistence operation failed

**StorageError**
- `ReadError` - Failed to read from storage
- `WriteError` - Failed to write to storage
- `ParseError` - Failed to deserialize JSON
- `FileNotFound` - Storage file missing (handled by initializing empty)

All operations return `Result<T, E>` types to force explicit error handling. The CLI layer catches errors and displays user-friendly messages.

## Testing Strategy

The Stock Control System will employ a comprehensive dual testing approach combining unit tests and property-based tests to ensure correctness.

### Unit Testing

Unit tests will verify:
- Specific examples of core operations (create product, add stock, remove stock)
- Edge cases such as empty result sets, boundary values at reorder points
- Error conditions with specific invalid inputs
- Integration between service and storage layers
- CLI command parsing and output formatting

Unit tests provide concrete examples that demonstrate correct behavior and catch specific bugs in implementation logic.

### Property-Based Testing

Property-based testing will be implemented using the **quickcheck** crate for Rust. Each property-based test will:
- Run a minimum of 100 iterations with randomly generated inputs
- Be tagged with a comment explicitly referencing the correctness property from this design document
- Use the format: `// Feature: stock-control, Property N: [property text]`
- Verify universal properties that should hold across all valid inputs

Property-based tests will focus on:
- Round-trip properties for serialization/deserialization
- Invariants that must hold after operations (e.g., stock level changes)
- Consistency properties (e.g., transaction history preservation)
- Input validation across the entire input space
- Filtering and query correctness with random data sets

The combination of unit tests (specific examples) and property-based tests (universal properties) provides comprehensive coverage: unit tests catch concrete bugs while property tests verify general correctness across the input space.

### Test Organization

Tests will be organized as follows:
- `tests/unit/` - Unit tests for individual components
- `tests/property/` - Property-based tests implementing correctness properties
- `tests/integration/` - Integration tests for end-to-end workflows

Each property-based test must implement exactly one correctness property from this document and include the property reference tag.
