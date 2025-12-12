// Command-line interface for Stock Control System

use std::env;
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::errors::ServiceError;
use crate::service::InventoryService;
use crate::storage::JsonStorage;

/// CLI commands for inventory operations
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Add a new product to inventory
    AddProduct {
        sku: String,
        name: String,
        description: String,
        quantity: u32,
        reorder_point: u32,
    },
    /// Update an existing product
    UpdateProduct {
        sku: String,
        name: Option<String>,
        description: Option<String>,
        reorder_point: Option<u32>,
    },
    /// Add stock to a product
    AddStock {
        sku: String,
        quantity: u32,
        notes: Option<String>,
    },
    /// Remove stock from a product
    RemoveStock {
        sku: String,
        quantity: u32,
        notes: Option<String>,
    },
    /// View a single product by SKU
    ViewProduct {
        sku: String,
    },
    /// List all products
    ListProducts,
    /// List products with low stock
    LowStock,
    /// View transaction history for a product
    History {
        sku: String,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    },
    /// Delete a product
    DeleteProduct {
        sku: String,
    },
    /// Show help message
    Help,
}

/// Parse command-line arguments into a Command
/// 
/// # Arguments
/// * `args` - Command-line arguments (including program name)
/// 
/// # Returns
/// * `Ok(Command)` - Successfully parsed command
/// * `Err(String)` - Error message describing the parsing failure
pub fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Ok(Command::Help);
    }

    let command = args[1].as_str();
    
    match command {
        "add-product" => parse_add_product(&args[2..]),
        "update-product" => parse_update_product(&args[2..]),
        "add-stock" => parse_add_stock(&args[2..]),
        "remove-stock" => parse_remove_stock(&args[2..]),
        "view-product" => parse_view_product(&args[2..]),
        "list-products" => Ok(Command::ListProducts),
        "low-stock" => Ok(Command::LowStock),
        "history" => parse_history(&args[2..]),
        "delete-product" => parse_delete_product(&args[2..]),
        "help" | "--help" | "-h" => Ok(Command::Help),
        _ => Err(format!("Unknown command: '{}'. Use 'help' to see available commands.", command)),
    }
}


/// Parse add-product command arguments
fn parse_add_product(args: &[String]) -> Result<Command, String> {
    if args.len() < 5 {
        return Err(
            "Usage: add-product <sku> <name> <description> <quantity> <reorder_point>\n\
             Example: add-product SKU001 \"Widget\" \"A useful widget\" 100 20".to_string()
        );
    }

    let sku = args[0].clone();
    let name = args[1].clone();
    let description = args[2].clone();
    let quantity = args[3].parse::<u32>()
        .map_err(|_| format!("Invalid quantity '{}': must be a non-negative integer", args[3]))?;
    let reorder_point = args[4].parse::<u32>()
        .map_err(|_| format!("Invalid reorder point '{}': must be a non-negative integer", args[4]))?;

    Ok(Command::AddProduct {
        sku,
        name,
        description,
        quantity,
        reorder_point,
    })
}

/// Parse update-product command arguments
fn parse_update_product(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err(
            "Usage: update-product <sku> [--name <name>] [--description <desc>] [--reorder-point <n>]\n\
             Example: update-product SKU001 --name \"New Name\" --reorder-point 30".to_string()
        );
    }

    let sku = args[0].clone();
    let mut name = None;
    let mut description = None;
    let mut reorder_point = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                if i + 1 >= args.len() {
                    return Err("--name requires a value".to_string());
                }
                name = Some(args[i + 1].clone());
                i += 2;
            }
            "--description" => {
                if i + 1 >= args.len() {
                    return Err("--description requires a value".to_string());
                }
                description = Some(args[i + 1].clone());
                i += 2;
            }
            "--reorder-point" => {
                if i + 1 >= args.len() {
                    return Err("--reorder-point requires a value".to_string());
                }
                reorder_point = Some(args[i + 1].parse::<u32>()
                    .map_err(|_| format!("Invalid reorder point '{}': must be a non-negative integer", args[i + 1]))?);
                i += 2;
            }
            _ => {
                return Err(format!("Unknown option: '{}'. Valid options: --name, --description, --reorder-point", args[i]));
            }
        }
    }

    Ok(Command::UpdateProduct {
        sku,
        name,
        description,
        reorder_point,
    })
}

/// Parse add-stock command arguments
fn parse_add_stock(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err(
            "Usage: add-stock <sku> <quantity> [--notes <notes>]\n\
             Example: add-stock SKU001 50 --notes \"Received shipment\"".to_string()
        );
    }

    let sku = args[0].clone();
    let quantity = args[1].parse::<u32>()
        .map_err(|_| format!("Invalid quantity '{}': must be a positive integer", args[1]))?;
    
    let mut notes = None;
    if args.len() > 2 && args[2] == "--notes" {
        if args.len() > 3 {
            notes = Some(args[3].clone());
        } else {
            return Err("--notes requires a value".to_string());
        }
    }

    Ok(Command::AddStock { sku, quantity, notes })
}

/// Parse remove-stock command arguments
fn parse_remove_stock(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err(
            "Usage: remove-stock <sku> <quantity> [--notes <notes>]\n\
             Example: remove-stock SKU001 10 --notes \"Sold to customer\"".to_string()
        );
    }

    let sku = args[0].clone();
    let quantity = args[1].parse::<u32>()
        .map_err(|_| format!("Invalid quantity '{}': must be a positive integer", args[1]))?;
    
    let mut notes = None;
    if args.len() > 2 && args[2] == "--notes" {
        if args.len() > 3 {
            notes = Some(args[3].clone());
        } else {
            return Err("--notes requires a value".to_string());
        }
    }

    Ok(Command::RemoveStock { sku, quantity, notes })
}

/// Parse view-product command arguments
fn parse_view_product(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err(
            "Usage: view-product <sku>\n\
             Example: view-product SKU001".to_string()
        );
    }

    Ok(Command::ViewProduct { sku: args[0].clone() })
}

/// Parse history command arguments
fn parse_history(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err(
            "Usage: history <sku> [--start <datetime>] [--end <datetime>]\n\
             Example: history SKU001 --start 2025-01-01T00:00:00 --end 2025-12-31T23:59:59".to_string()
        );
    }

    let sku = args[0].clone();
    let mut start = None;
    let mut end = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--start" => {
                if i + 1 >= args.len() {
                    return Err("--start requires a datetime value (e.g., 2025-01-01T00:00:00)".to_string());
                }
                start = Some(parse_datetime(&args[i + 1])?);
                i += 2;
            }
            "--end" => {
                if i + 1 >= args.len() {
                    return Err("--end requires a datetime value (e.g., 2025-12-31T23:59:59)".to_string());
                }
                end = Some(parse_datetime(&args[i + 1])?);
                i += 2;
            }
            _ => {
                return Err(format!("Unknown option: '{}'. Valid options: --start, --end", args[i]));
            }
        }
    }

    Ok(Command::History { sku, start, end })
}

/// Parse a datetime string into DateTime<Utc>
fn parse_datetime(s: &str) -> Result<DateTime<Utc>, String> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
        .map(|dt| dt.and_utc())
        .map_err(|_| format!("Invalid datetime '{}': expected format YYYY-MM-DDTHH:MM:SS", s))
}

/// Parse delete-product command arguments
fn parse_delete_product(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err(
            "Usage: delete-product <sku>\n\
             Example: delete-product SKU001".to_string()
        );
    }

    Ok(Command::DeleteProduct { sku: args[0].clone() })
}


/// Execute a command against the inventory service
/// 
/// # Arguments
/// * `command` - The command to execute
/// * `service` - The inventory service instance
/// 
/// # Returns
/// * `Ok(String)` - Success message to display
/// * `Err(String)` - Error message to display
pub fn execute_command(command: Command, service: &mut InventoryService) -> Result<String, String> {
    match command {
        Command::AddProduct { sku, name, description, quantity, reorder_point } => {
            let product = service.add_product(sku, name, description, quantity, reorder_point)
                .map_err(format_error)?;
            Ok(format!(
                "Product added successfully:\n  ID: {}\n  SKU: {}\n  Name: {}\n  Description: {}\n  Quantity: {}\n  Reorder Point: {}",
                product.id, product.sku, product.name, product.description, product.quantity, product.reorder_point
            ))
        }
        
        Command::UpdateProduct { sku, name, description, reorder_point } => {
            let product = service.update_product(&sku, name, description, reorder_point)
                .map_err(format_error)?;
            Ok(format!(
                "Product updated successfully:\n  SKU: {}\n  Name: {}\n  Description: {}\n  Quantity: {}\n  Reorder Point: {}",
                product.sku, product.name, product.description, product.quantity, product.reorder_point
            ))
        }
        
        Command::AddStock { sku, quantity, notes } => {
            service.add_stock(&sku, quantity, notes)
                .map_err(format_error)?;
            let product = service.get_product(&sku).map_err(format_error)?;
            Ok(format!(
                "Stock added successfully:\n  SKU: {}\n  Added: {}\n  New Quantity: {}",
                sku, quantity, product.quantity
            ))
        }
        
        Command::RemoveStock { sku, quantity, notes } => {
            service.remove_stock(&sku, quantity, notes)
                .map_err(format_error)?;
            let product = service.get_product(&sku).map_err(format_error)?;
            Ok(format!(
                "Stock removed successfully:\n  SKU: {}\n  Removed: {}\n  New Quantity: {}",
                sku, quantity, product.quantity
            ))
        }
        
        Command::ViewProduct { sku } => {
            let product = service.get_product(&sku).map_err(format_error)?;
            let low_stock_warning = if product.quantity <= product.reorder_point {
                " [LOW STOCK]"
            } else {
                ""
            };
            Ok(format!(
                "Product Details:\n  ID: {}\n  SKU: {}\n  Name: {}\n  Description: {}\n  Quantity: {}{}\n  Reorder Point: {}",
                product.id, product.sku, product.name, product.description, product.quantity, low_stock_warning, product.reorder_point
            ))
        }
        
        Command::ListProducts => {
            let products = service.list_products();
            if products.is_empty() {
                return Ok("No products in inventory.".to_string());
            }
            
            let mut output = format!("Products ({} total):\n", products.len());
            for product in products {
                let low_stock = if product.quantity <= product.reorder_point { " [LOW]" } else { "" };
                output.push_str(&format!(
                    "  {} - {} (Qty: {}{})\n",
                    product.sku, product.name, product.quantity, low_stock
                ));
            }
            Ok(output.trim_end().to_string())
        }
        
        Command::LowStock => {
            let products = service.list_low_stock();
            if products.is_empty() {
                return Ok("No products with low stock.".to_string());
            }
            
            let mut output = format!("Low Stock Products ({} total):\n", products.len());
            for product in products {
                output.push_str(&format!(
                    "  {} - {} (Qty: {}, Reorder at: {})\n",
                    product.sku, product.name, product.quantity, product.reorder_point
                ));
            }
            Ok(output.trim_end().to_string())
        }
        
        Command::History { sku, start, end } => {
            // Verify product exists first
            service.get_product(&sku).map_err(format_error)?;
            
            let transactions = match (start, end) {
                (Some(s), Some(e)) => service.get_transactions_in_range(&sku, s, e),
                _ => service.get_transactions(&sku),
            };
            
            if transactions.is_empty() {
                return Ok(format!("No transactions found for product '{}'.", sku));
            }
            
            let mut output = format!("Transaction History for '{}' ({} transactions):\n", sku, transactions.len());
            for txn in transactions {
                let type_str = match txn.transaction_type {
                    crate::models::TransactionType::Addition => "+",
                    crate::models::TransactionType::Removal => "-",
                };
                let notes_str = txn.notes.as_ref().map(|n| format!(" - {}", n)).unwrap_or_default();
                output.push_str(&format!(
                    "  {} {} {} {}{}\n",
                    txn.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    type_str,
                    txn.quantity,
                    txn.transaction_type.to_string().to_lowercase(),
                    notes_str
                ));
            }
            Ok(output.trim_end().to_string())
        }
        
        Command::DeleteProduct { sku } => {
            service.delete_product(&sku).map_err(format_error)?;
            Ok(format!("Product '{}' deleted successfully.", sku))
        }
        
        Command::Help => {
            Ok(get_help_text())
        }
    }
}

/// Format a ServiceError into a user-friendly message
fn format_error(err: ServiceError) -> String {
    match err {
        ServiceError::ProductNotFound(sku) => format!("Error: Product '{}' not found.", sku),
        ServiceError::DuplicateSKU(sku) => format!("Error: Product with SKU '{}' already exists.", sku),
        ServiceError::InvalidInput(msg) => format!("Error: {}", msg),
        ServiceError::InsufficientStock { sku, requested, available } => {
            format!("Error: Insufficient stock for '{}'. Requested: {}, Available: {}", sku, requested, available)
        }
        ServiceError::StorageError(e) => format!("Error: Storage operation failed - {}", e),
    }
}

/// Get the help text for the CLI
fn get_help_text() -> String {
    r#"Stock Control System - Inventory Management CLI

USAGE:
    stock-control <COMMAND> [OPTIONS]

COMMANDS:
    add-product <sku> <name> <description> <quantity> <reorder_point>
        Add a new product to inventory
        Example: add-product SKU001 "Widget" "A useful widget" 100 20

    update-product <sku> [--name <name>] [--description <desc>] [--reorder-point <n>]
        Update an existing product's details
        Example: update-product SKU001 --name "New Widget" --reorder-point 30

    add-stock <sku> <quantity> [--notes <notes>]
        Add stock to a product
        Example: add-stock SKU001 50 --notes "Received shipment"

    remove-stock <sku> <quantity> [--notes <notes>]
        Remove stock from a product
        Example: remove-stock SKU001 10 --notes "Sold to customer"

    view-product <sku>
        View details of a specific product
        Example: view-product SKU001

    list-products
        List all products in inventory

    low-stock
        List products with stock at or below reorder point

    history <sku> [--start <datetime>] [--end <datetime>]
        View transaction history for a product
        Datetime format: YYYY-MM-DDTHH:MM:SS
        Example: history SKU001 --start 2025-01-01T00:00:00 --end 2025-12-31T23:59:59

    delete-product <sku>
        Delete a product and all its transactions
        Example: delete-product SKU001

    help
        Show this help message"#.to_string()
}


/// Run the CLI application
/// 
/// # Arguments
/// * `data_dir` - Directory for storing data files
/// 
/// # Returns
/// * `Ok(())` - Application completed successfully
/// * `Err(String)` - Error message
pub fn run(data_dir: &str) -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    run_with_args(&args, data_dir)
}

/// Run the CLI application with provided arguments
/// 
/// # Arguments
/// * `args` - Command-line arguments
/// * `data_dir` - Directory for storing data files
/// 
/// # Returns
/// * `Ok(())` - Application completed successfully
/// * `Err(String)` - Error message
pub fn run_with_args(args: &[String], data_dir: &str) -> Result<(), String> {
    let command = parse_args(args)?;
    
    // Help command doesn't need the service
    if matches!(command, Command::Help) {
        println!("{}", get_help_text());
        return Ok(());
    }
    
    // Initialize storage and service
    let storage = JsonStorage::new(data_dir);
    let mut service = InventoryService::new(Box::new(storage))
        .map_err(|e| format!("Failed to initialize inventory service: {}", e))?;
    
    // Execute command and print result
    match execute_command(command, &mut service) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(err) => {
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn test_parse_help() {
        assert_eq!(parse_args(&args("prog help")).unwrap(), Command::Help);
        assert_eq!(parse_args(&args("prog --help")).unwrap(), Command::Help);
        assert_eq!(parse_args(&args("prog -h")).unwrap(), Command::Help);
        assert_eq!(parse_args(&args("prog")).unwrap(), Command::Help);
    }

    #[test]
    fn test_parse_add_product() {
        let result = parse_args(&args("prog add-product SKU001 Widget Description 100 20")).unwrap();
        assert_eq!(result, Command::AddProduct {
            sku: "SKU001".to_string(),
            name: "Widget".to_string(),
            description: "Description".to_string(),
            quantity: 100,
            reorder_point: 20,
        });
    }

    #[test]
    fn test_parse_add_product_missing_args() {
        let result = parse_args(&args("prog add-product SKU001 Widget"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_update_product() {
        let result = parse_args(&args("prog update-product SKU001 --name NewName --reorder-point 30")).unwrap();
        assert_eq!(result, Command::UpdateProduct {
            sku: "SKU001".to_string(),
            name: Some("NewName".to_string()),
            description: None,
            reorder_point: Some(30),
        });
    }

    #[test]
    fn test_parse_add_stock() {
        let result = parse_args(&args("prog add-stock SKU001 50")).unwrap();
        assert_eq!(result, Command::AddStock {
            sku: "SKU001".to_string(),
            quantity: 50,
            notes: None,
        });
    }

    #[test]
    fn test_parse_add_stock_with_notes() {
        let result = parse_args(&args("prog add-stock SKU001 50 --notes Shipment")).unwrap();
        assert_eq!(result, Command::AddStock {
            sku: "SKU001".to_string(),
            quantity: 50,
            notes: Some("Shipment".to_string()),
        });
    }

    #[test]
    fn test_parse_remove_stock() {
        let result = parse_args(&args("prog remove-stock SKU001 10")).unwrap();
        assert_eq!(result, Command::RemoveStock {
            sku: "SKU001".to_string(),
            quantity: 10,
            notes: None,
        });
    }

    #[test]
    fn test_parse_view_product() {
        let result = parse_args(&args("prog view-product SKU001")).unwrap();
        assert_eq!(result, Command::ViewProduct { sku: "SKU001".to_string() });
    }

    #[test]
    fn test_parse_list_products() {
        let result = parse_args(&args("prog list-products")).unwrap();
        assert_eq!(result, Command::ListProducts);
    }

    #[test]
    fn test_parse_low_stock() {
        let result = parse_args(&args("prog low-stock")).unwrap();
        assert_eq!(result, Command::LowStock);
    }

    #[test]
    fn test_parse_history() {
        let result = parse_args(&args("prog history SKU001")).unwrap();
        assert_eq!(result, Command::History {
            sku: "SKU001".to_string(),
            start: None,
            end: None,
        });
    }

    #[test]
    fn test_parse_delete_product() {
        let result = parse_args(&args("prog delete-product SKU001")).unwrap();
        assert_eq!(result, Command::DeleteProduct { sku: "SKU001".to_string() });
    }

    #[test]
    fn test_parse_unknown_command() {
        let result = parse_args(&args("prog unknown-cmd"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown command"));
    }

    #[test]
    fn test_parse_invalid_quantity() {
        let result = parse_args(&args("prog add-product SKU001 Widget Desc abc 20"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid quantity"));
    }
}
