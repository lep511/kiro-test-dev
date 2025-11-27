# Implementation Plan

- [x] 1. Set up project structure and dependencies





  - Create new Rust project with `cargo init`
  - Add dependencies to `Cargo.toml`: `serde`, `serde_json`, `chrono`, `uuid`, `quickcheck` (dev)
  - Create module structure: `models`, `storage`, `service`, `cli`, `errors`
  - _Requirements: All_

- [x] 2. Implement core data models



  - [x] 2.1 Create Product and Transaction structs with serde derives


    - Define `Product` struct with id, sku, name, description, quantity, reorder_point
    - Define `Transaction` struct with id, product_sku, transaction_type, quantity, timestamp, notes
    - Define `TransactionType` enum (Addition, Removal)
    - Add serde Serialize/Deserialize derives
    - _Requirements: 1.1, 3.3, 4.3_
  
  - [ ] 2.2 Write property test for product serialization round-trip





    - **Property 3: Product persistence round-trip**
    - **Validates: Requirements 1.4, 8.1, 8.4**
  
  - [ ]* 2.3 Write property test for storage serialization round-trip
    - **Property 25: Storage serialization round-trip**
    - **Validates: Requirements 8.4**

- [ ] 3. Implement error types
  - [ ] 3.1 Define ServiceError and StorageError enums
    - Create `ServiceError` with variants: ProductNotFound, DuplicateSKU, InvalidInput, InsufficientStock, StorageError
    - Create `StorageError` with variants: ReadError, WriteError, ParseError, FileNotFound
    - Implement Display and Error traits
    - _Requirements: 1.2, 2.2, 3.2, 3.5, 4.2, 7.2, 8.3, 8.5_

- [ ] 4. Implement storage layer
  - [ ] 4.1 Create Storage trait and JsonStorage implementation
    - Define `Storage` trait with save/load methods for products and transactions
    - Implement `JsonStorage` struct with file paths
    - Implement save_products, load_products, save_transactions, load_transactions
    - Handle missing files by returning empty vectors
    - Handle corrupted files with ParseError
    - _Requirements: 1.4, 8.1, 8.2, 8.3, 8.4, 8.5_
  
  - [ ]* 4.2 Write unit tests for storage operations
    - Test saving and loading products
    - Test saving and loading transactions
    - Test handling of missing files
    - Test handling of corrupted JSON
    - _Requirements: 8.1, 8.3, 8.5_
  
  - [ ]* 4.3 Write property test for graceful error handling
    - **Property 26: Graceful handling of invalid storage data**
    - **Validates: Requirements 8.5**

- [ ] 5. Implement inventory service - product management
  - [ ] 5.1 Create InventoryService struct with HashMap and storage
    - Define `InventoryService` with products HashMap, transactions Vec, storage Box
    - Implement `new()` constructor that loads data from storage
    - _Requirements: 8.1_
  
  - [ ] 5.2 Implement add_product method
    - Validate SKU is non-empty
    - Validate quantity and reorder_point are non-negative
    - Check for duplicate SKU
    - Generate UUID for product id
    - Insert into HashMap
    - Persist to storage
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_
  
  - [ ]* 5.3 Write property test for product creation with valid inputs
    - **Property 1: Product creation with valid inputs**
    - **Validates: Requirements 1.1, 1.3, 1.5**
  
  - [ ]* 5.4 Write property test for duplicate SKU rejection
    - **Property 2: Duplicate SKU rejection**
    - **Validates: Requirements 1.2**
  
  - [ ] 5.5 Implement update_product method
    - Validate product exists
    - Validate updated fields (non-empty name if provided)
    - Update specified fields only
    - Persist to storage
    - _Requirements: 2.1, 2.2, 2.3, 2.4_
  
  - [ ]* 5.6 Write property test for product update
    - **Property 4: Product update modifies fields**
    - **Validates: Requirements 2.1, 2.3**
  
  - [ ]* 5.7 Write property test for non-existent product update
    - **Property 5: Non-existent product update fails**
    - **Validates: Requirements 2.2**
  
  - [ ]* 5.8 Write property test for transaction history preservation
    - **Property 6: Product updates preserve transaction history**
    - **Validates: Requirements 2.5**
  
  - [ ] 5.9 Implement get_product method
    - Look up product by SKU in HashMap
    - Return reference or ProductNotFound error
    - _Requirements: 5.1_
  
  - [ ]* 5.10 Write property test for product retrieval
    - **Property 7: Product retrieval returns complete data**
    - **Validates: Requirements 5.1, 5.4**
  
  - [ ] 5.11 Implement list_products method
    - Return vector of references to all products
    - _Requirements: 5.2_
  
  - [ ]* 5.12 Write property test for listing all products
    - **Property 8: List all products returns complete set**
    - **Validates: Requirements 5.2**
  
  - [ ] 5.13 Implement delete_product method
    - Validate product exists
    - Remove product from HashMap
    - Remove all associated transactions
    - Persist changes to storage
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_
  
  - [ ]* 5.14 Write property test for product deletion
    - **Property 9: Product deletion removes data**
    - **Validates: Requirements 7.1, 7.3, 7.4**
  
  - [ ]* 5.15 Write property test for non-existent product deletion
    - **Property 10: Non-existent product deletion fails**
    - **Validates: Requirements 7.2**
  
  - [ ]* 5.16 Write property test for deleting products with stock
    - **Property 11: Products with stock can be deleted**
    - **Validates: Requirements 7.5**

- [ ] 6. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 7. Implement inventory service - stock transactions
  - [ ] 7.1 Implement add_stock method
    - Validate product exists
    - Validate quantity is positive
    - Increase product quantity
    - Generate UUID for transaction id
    - Create Transaction with type Addition and current timestamp
    - Add transaction to vector
    - Persist both products and transactions
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ]* 7.2 Write property test for stock addition
    - **Property 12: Stock addition increases quantity**
    - **Validates: Requirements 3.1**
  
  - [ ]* 7.3 Write property test for negative stock addition
    - **Property 13: Negative stock addition rejected**
    - **Validates: Requirements 3.2**
  
  - [ ]* 7.4 Write property test for addition transaction record
    - **Property 14: Stock addition creates transaction record**
    - **Validates: Requirements 3.3**
  
  - [ ]* 7.5 Write property test for addition to non-existent product
    - **Property 15: Stock addition to non-existent product fails**
    - **Validates: Requirements 3.5**
  
  - [ ] 7.6 Implement remove_stock method
    - Validate product exists
    - Validate quantity is positive
    - Validate sufficient stock available
    - Decrease product quantity
    - Generate UUID for transaction id
    - Create Transaction with type Removal and current timestamp
    - Add transaction to vector
    - Persist both products and transactions
    - _Requirements: 4.1, 4.2, 4.3, 4.4_
  
  - [ ]* 7.7 Write property test for stock removal
    - **Property 16: Stock removal decreases quantity**
    - **Validates: Requirements 4.1**
  
  - [ ]* 7.8 Write property test for insufficient stock removal
    - **Property 17: Insufficient stock removal rejected**
    - **Validates: Requirements 4.2**
  
  - [ ]* 7.9 Write property test for removal transaction record
    - **Property 18: Stock removal creates transaction record**
    - **Validates: Requirements 4.3**
  
  - [ ]* 7.10 Write property test for transaction persistence
    - **Property 19: Stock transaction persistence round-trip**
    - **Validates: Requirements 3.4, 4.4, 8.2**

- [ ] 8. Implement inventory service - queries and filtering
  - [ ] 8.1 Implement list_low_stock method
    - Filter products where quantity <= reorder_point
    - Return vector of references
    - _Requirements: 4.5, 5.3_
  
  - [ ]* 8.2 Write property test for low-stock detection
    - **Property 20: Low stock detection after removal**
    - **Validates: Requirements 4.5**
  
  - [ ]* 8.3 Write property test for low-stock filtering
    - **Property 21: Low-stock filter returns correct products**
    - **Validates: Requirements 5.3**
  
  - [ ] 8.4 Implement get_transactions method
    - Filter transactions by product_sku
    - Sort by timestamp ascending
    - Return vector of references
    - _Requirements: 6.1, 6.2, 6.5_
  
  - [ ]* 8.5 Write property test for transaction history ordering
    - **Property 22: Transaction history ordered chronologically**
    - **Validates: Requirements 6.1, 6.5**
  
  - [ ]* 8.6 Write property test for transaction data completeness
    - **Property 23: Transaction records contain complete data**
    - **Validates: Requirements 6.2**
  
  - [ ] 8.7 Implement get_transactions_in_range method
    - Filter transactions by product_sku and date range
    - Sort by timestamp ascending
    - Return vector of references
    - _Requirements: 6.3_
  
  - [ ]* 8.8 Write property test for date range filtering
    - **Property 24: Date range filtering returns correct transactions**
    - **Validates: Requirements 6.3**

- [ ] 9. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. Implement CLI interface
  - [ ] 10.1 Create CLI module with command enum
    - Define `Command` enum with variants for all operations
    - Each variant contains necessary parameters
    - _Requirements: All_
  
  - [ ] 10.2 Implement command parsing from args
    - Parse command-line arguments into Command enum
    - Handle invalid commands with helpful error messages
    - _Requirements: All_
  
  - [ ] 10.3 Implement command execution
    - Match on Command enum and call appropriate service methods
    - Format and display results
    - Display user-friendly error messages
    - _Requirements: All_
  
  - [ ] 10.4 Implement main function
    - Initialize InventoryService with JsonStorage
    - Parse command from args
    - Execute command
    - Handle errors and display messages
    - _Requirements: All_
  
  - [ ]* 10.5 Write unit tests for CLI command parsing
    - Test parsing of each command type
    - Test error handling for invalid commands
    - _Requirements: All_

- [ ] 11. Add documentation and examples
  - [ ] 11.1 Add doc comments to all public types and methods
    - Document all structs, enums, traits, and public functions
    - Include usage examples in doc comments
    - _Requirements: All_
  
  - [ ] 11.2 Create README with usage instructions
    - Document all CLI commands with examples
    - Include build and run instructions
    - Provide example workflows
    - _Requirements: All_

- [ ] 12. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
