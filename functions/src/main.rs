mod coding_challenge;
use crate::coding_challenge::*;

fn main() {
    open_store("Neelankarai");
    bake_pizza(20, "margaritta");
    swim_in_profit();
    println!("{}", square(3));

    let result = mystery();
    println!("mystery result: {:?}", result);

    let multiplier = 3;

    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("calculation {}", calculation);

    apply_to_jobs(20, "Rust Developer");
    println!("even {}", is_even(10));
    println!("even {}", is_even(3));
    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
}

fn open_store(neighbourhood: &str) {
    println!("Opening my pizza store in {neighbourhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizza");
}

fn swim_in_profit() {
    println!("So much $$$, so little time")
}

fn square(number: i32) -> i32 {
    number * number
}

fn mystery() {}