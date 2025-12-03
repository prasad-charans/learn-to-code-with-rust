mod coding_challenge;
use crate::coding_challenge::*;

fn even_or_odd(n: i32) {
    let result = if n % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("[countdown] Taking off ...");
    } else {
        println!("[countdown] {seconds} seconds to take off ...");
        countdown(seconds - 1);
    }
}

fn main() {
    let season = "Spring";

    if season == "Summer" {
        println!("School's out");
    } else if season == "Winter" {
        println!("It's cold");
    } else if season == "Fall" {
        println!("Leaves falling");
    } else if season == "Spring" {
        println!("It's raining");
    } else {
        println!("Some season");
    }

    even_or_odd(8);
    even_or_odd(3);

    match season {
        "Summer" => println!("School's out"),
        "Winter" => println!("It's cold"),
        "Spring" => println!("It's raining"),
        "Fall" => println!("Leaves falling"),
        _ => println!("Some season"),
    }

    let n = 11;
    match n {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }

    let mut seconds = 21;

    loop {
        if seconds == 0 {
            println!("Breaking the loop");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds is even, skiiping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to break..");
        seconds -= 1;
    }

    seconds = 21;
    while seconds > 0 {
        println!("while seconds {seconds}");
        if seconds % 2 == 0 {
            println!("while {seconds} seconds is even, skiiping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("while {seconds} seconds to break..");
        seconds -= 1;
    }

    countdown(21);

    println!("color number {} for color 'red'", color_to_number("red"));
    println!(
        "color number {} for color 'green'",
        color_to_number("green")
    );
    println!("color number {} for color 'blue'", color_to_number("blue"));
    println!(
        "color number {} for color 'yellow'",
        color_to_number("yellow")
    );

    println!(
        "color number {} for color 'red'",
        color_to_number_match("red")
    );
    println!(
        "color number {} for color 'green'",
        color_to_number_match("green")
    );
    println!(
        "color number {} for color 'blue'",
        color_to_number_match("blue")
    );
    println!(
        "color number {} for color 'yellow'",
        color_to_number_match("yellow")
    );

    println!("Factorial of 10 is {}", factorial_iterative(10));
    println!("Factorial of 10 is {}", factorial_recursive(10));
}
