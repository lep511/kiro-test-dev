# Stock Control System

A Rust-based inventory management CLI application for tracking products and stock transactions.

## Core Functionality
- Product management (CRUD operations with SKU-based identification)
- Stock tracking (additions and removals with transaction history)
- Low stock alerts via reorder points
- JSON file-based persistence

## Domain Concepts
- **Product**: Inventory item with SKU, name, description, quantity, and reorder point
- **Transaction**: Stock movement record (Addition/Removal) with timestamp and optional notes
- **SKU**: Unique product identifier used as the primary lookup key
