mod coding_challenge_1;
use std::mem::size_of;

use crate::coding_challenge_1::*;

fn main() {
    let age = 45;

    {
        let scoped_var = true;
        println!("Scoped var: {}", scoped_var);
    }

    // println!("Scoped var: {}", scoped_var);
    println!("Age: {}", age);

    let time = 2025;
    let year = time;

    println!("The time is {time}. It is the year {year}.");

    // Heap
    let text = String::new();
    let mut from_text = String::from("Sivaprasad");

    println!("Size of string {}", size_of::<String>());
    println!("String capacity: {}", text.capacity());
    println!(
        "from_text String capacity: {}, length: {}",
        from_text.capacity(),
        from_text.len()
    );

    from_text.push_str(" Chidambaram");
    println!(
        "from_text String capacity: {}, length: {}",
        from_text.capacity(),
        from_text.len()
    );

    //Ownership move
    let person = String::from("Sivaprasad");
    println!("Address of person variable {:p}", person.as_ptr());
    let developer = person;

    // Will not compile as value is borrowed after moved to developer
    // println!("My name is {person}");
    println!(
        "My name is {developer} and stored address is {:p}",
        developer.as_ptr()
    );

    drop(developer);

    //Cloning
    let p = String::from("Sivaprasad");
    let d = p.clone();
    println!("My name {:p}, {}", p.as_ptr(), p);
    println!("My name {:p}, {}", d.as_ptr(), d);

    //References and Borrowing
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;
    println!("{}", *my_integer_reference); //Dereference

    let my_heap_value = String::from("Toyota");
    let my_head_reference = &my_heap_value;
    println!("{}", *my_head_reference); //Dereference

    /*
        String - A dynamic piece of text stored on the heap at runtime
        &String - "Reference String" -- A reference to a heap string
        str -- A hardcoded string encoded into the binary
        &str -- "Reference str" - Reference to the text in the memory that
        has loaded the binary
    */
    let ice_cream = "Butter Scotch";
    let dessert = ice_cream;
    println!("{}, {}", ice_cream, dessert);

    let apples = 10;
    print_my_value(apples);
    println!("{apples} is still valid.");

    let oranges = String::from("Oranges");
    print_my_string(oranges);
    // println!("{oranges} is still valid.")

    let burger = String::from("Burger");
    add_fries(burger);
    // println!("Burger {burger}");

    let cake = bake_cake();
    println!("I now have a {cake} cake.");

    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    println!("Current meal : {}", current_meal);

    challenge_1();
}

fn print_my_value(value: i32) {
    println!("Your value is {value}");
}

fn print_my_string(value: String) {
    println!("Your value is {value}");
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}")
}

fn bake_cake() -> String {
    let cake = String::from("Choclate Truffle");
    return cake;
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add Flour");
    meal
}
