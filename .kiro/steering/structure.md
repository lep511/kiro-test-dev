# Project Structure

```
stock-control/
├── src/
│   ├── main.rs      # Entry point
│   ├── lib.rs       # Library exports
│   ├── models.rs    # Data structures (Product, Transaction)
│   ├── errors.rs    # Error types (StorageError, ServiceError)
│   ├── storage.rs   # Persistence layer (Storage trait, JsonStorage)
│   ├── service.rs   # Business logic (InventoryService)
│   └── cli.rs       # Command-line interface (placeholder)
└── tests/
    ├── product_properties.rs   # Product serialization tests
    ├── storage_properties.rs   # Storage round-trip tests
    └── property/               # Additional property tests
```

## Architecture Layers
1. **Models** (`models.rs`): Pure data structures with serde derives
2. **Errors** (`errors.rs`): Domain-specific error enums with Display/Error traits
3. **Storage** (`storage.rs`): Trait-based persistence abstraction with JSON implementation
4. **Service** (`service.rs`): Business logic operating on in-memory HashMap, delegates persistence to Storage

## Conventions
- SKU is the primary key for product lookups (stored in HashMap<String, Product>)
- All IDs are UUID v4 strings
- Timestamps use `chrono::DateTime<Utc>`
- Storage trait enables swappable backends
- Methods document requirements they implement via doc comments
