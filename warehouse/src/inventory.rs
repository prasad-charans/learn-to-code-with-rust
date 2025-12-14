pub mod products;
pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Prasad Inventory";

fn talk_to_manager() {
    println!(
        "Hey Manager: {}, How's your coffee? What do you think of {:?}",
        MANAGER,
        ProductCategory::Ladder
    );
}
