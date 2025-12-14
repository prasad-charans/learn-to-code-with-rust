use fake::Dummy;
use std::fmt;

/// A category of product
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

// Implement Display for ProductCategory
impl fmt::Display for ProductCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductCategory::Ladder => write!(f, "Ladder"),
            ProductCategory::Hammer => write!(f, "Hammer"),
        }
    }
}

/// A concrete item
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

// Implement Display for Item to demonstrate fmt usage
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Category: {}, Qty: {})",
            self.name, self.category, self.quantity
        )
    }
}

impl Item {
    /// Create a new item
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}
