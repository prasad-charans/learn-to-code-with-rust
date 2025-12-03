const TAX_RATE: f64 = 0.03;

type Meters = i32;

mod coding_challenge;
use crate::coding_challenge::code_challenge;

#[allow(unused_variables)]
fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    // println!("This year my garden has {} fruits", fruits);
    println!("This year, my garden has {apples} Apples and {oranges} Oranges");
    println!(
        "This year, my garden has {0} Apples and {1} Oranges. I can't believe I have {0} Apples",
        apples, oranges
    );
    println!("My House");

    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps");
    gym_reps = 15;
    println!("I plan to do {gym_reps} reps");

    let grams_of_proteins = "100.345";
    // println!("grams of protein {}", grams_of_protein);
    let mut grams_of_proteins = 100.345;
    println!("grams of protein {}", grams_of_proteins);

    grams_of_proteins = 100.00;
    println!("grams of protein {}", grams_of_proteins);

    //Scopes
    let coffee_price = 5.99;

    {
        let coffee_price = 1.99;
        println!("The coffee price is {coffee_price}")
    }
    println!("The coffee price is {coffee_price}");

    let income = 100000;
    println!("My income is {income} The Tax rate is {TAX_RATE}");

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!(
        "A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} length."
    );

    code_challenge();
}
