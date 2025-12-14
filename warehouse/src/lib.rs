/// Tools for managing inventory
pub mod inventory;
/// Tools for managing orders
pub mod orders;

pub use inventory::{FLOOR_SPACE, Item, MANAGER as INVENTORY_MANAGER, ProductCategory};
pub use orders::MANAGER as ORDERS_MANAGER;
