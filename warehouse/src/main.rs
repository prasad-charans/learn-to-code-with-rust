use fake::{Fake, Faker};

use std::io::{Write, stdin, stdout};

use warehouse::*;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let fav_category = ProductCategory::Hammer;
    println!("My favourite category is {:?}", fav_category);

    let tall_ladder = Item::new(String::from("Tall Ladder"), fav_category, 100);
    println!("Tall ladder: {:?}", tall_ladder);

    // Example: Using fmt::Display trait (custom formatting)
    println!("\n--- fmt::Display Example ---");
    println!("Using Display trait: {}", tall_ladder);

    let fake_item = Faker.fake::<Item>();
    println!("Fake item: {:?}", fake_item);
    println!("Fake item with Display: {}", fake_item);

    let random_category = Faker.fake::<ProductCategory>();
    println!("Random category: {:?}", random_category);
    println!("Random category with Display: {}", random_category);

    // Example: Using stdout directly with Write trait
    println!("\n--- stdout Example ---");
    let mut stdout_handle = stdout();
    stdout_handle
        .write_all(b"Writing directly to stdout using write_all()\n")
        .unwrap();
    write!(
        stdout_handle,
        "Using write! macro with stdout: {}\n",
        "Hello!"
    )
    .unwrap();
    writeln!(
        stdout_handle,
        "Using writeln! macro with stdout: {}",
        "World!"
    )
    .unwrap();
    stdout_handle.flush().unwrap(); // Ensure all output is written

    let mut input = String::new();
    println!("\nEnter your name: ");
    stdin().read_line(&mut input).unwrap();
    println!("Hello, {}!", input.trim());
}
