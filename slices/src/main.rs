mod coding_challenge;
use crate::coding_challenge::*;

fn main() {
    let action_hero = String::from("Jason Statham");
    let first_name = &action_hero[0..5];
    println!("The action hero is: {}", first_name);

    let last_name = &action_hero[6..];
    println!("The last name is: {}", last_name);

    let name = {
        let my_action_hero = "Jason Statham";
        &my_action_hero[0..5]
    };
    println!("{name}");

    let food = "🍕";
    println!("Food: {}", food.len());

    do_hero_stuff(&action_hero);
    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(&another_action_hero);

    // Array slices
    let values = [4, 8, 15, 16, 23, 42];
    let my_slice = &values[..4];
    println!("Array slice: {:?}", my_slice);

    let my_slice = &values[2..4];
    println!("Array slice: {:?}", my_slice);

    let my_slice = &values[2..];
    println!("Array slice: {:?}", my_slice);
    print!("Length of the array slice:");
    print_length(my_slice);

    let my_slice = &values[..];
    println!("Array slice: {:?}", my_slice);

    let my_slice = &values;
    println!("Array slice: {:?}", my_slice);
    print!("Length of the array slice:");
    print_length(my_slice);

    let mut my_array = [10, 15, 20, 25, 30];
    let slice = &mut my_array[2..4];
    println!("My slice: {:?}", slice);
    slice[0] = 100;
    println!("My slice: {:?}", slice);
    println!("My array: {:?}", my_array);

    coding_challenge();
}

fn do_hero_stuff(hero_name: &str) {
    println!("Doing stuff with hero: {}", hero_name);
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}
