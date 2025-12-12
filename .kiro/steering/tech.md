# Tech Stack

## Language & Build
- Rust 2021 edition
- Cargo build system

## Dependencies
| Crate | Purpose |
|-------|---------|
| serde + serde_json | JSON serialization/deserialization |
| chrono | DateTime handling with UTC timestamps |
| uuid | UUID v4 generation for IDs |

## Dev Dependencies
| Crate | Purpose |
|-------|---------|
| quickcheck | Property-based testing |
| tempfile | Temporary directories for tests |

## Common Commands
```bash
cargo build          # Compile the project
cargo run            # Run the CLI application
cargo test           # Run all tests
cargo test --lib     # Run unit tests only
cargo clippy         # Lint the code
cargo fmt            # Format code
```
