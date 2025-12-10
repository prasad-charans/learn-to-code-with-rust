mod coding_challenge;
use crate::coding_challenge::coding_challenge;

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Called `unwrap()` on a `None` value"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Denominator cannot be zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let a = Option::Some(5);
    let b = Option::Some("Hello");
    let c = Option::Some(true);

    let a1: Option<i8> = Option::Some(5);
    let b1 = Option::<i16>::Some(5);

    let d: Option<&str> = Option::None;

    println!("{}", a.is_some());
    println!("{}", b.is_some());
    println!("{}", c.is_some());
    println!("{}", d.is_some());
    println!("{:?} {:?} {:?} {:?} {:?} {:?}", a, b, c, d, a1, b1);

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Piano"),
        String::from("Violin"),
    ];
    let bass = musical_instruments.get(1);
    println!("{:?}", bass);
    let valid_instrument = bass.unwrap();
    println!("{:?}", valid_instrument);
    let valid_instrument = bass.expect("Unable to get the instrument");
    println!("{:?}", valid_instrument);

    let invalid_instrument = musical_instruments.get(10);
    println!("{:?}", invalid_instrument);
    // let invalid_instrument = invalid_instrument.expect("Unable to get the instrument");
    // println!("{:?}", invalid_instrument);

    play_instrument(bass);
    play_instrument(invalid_instrument);

    let availability = is_item_in_stock(true, true);
    println!("{:?}", availability);
    match availability {
        Option::Some(value) => println!("Item is in stock: {}", value),
        None => println!("Item is not in stock"),
    }
    let availability = is_item_in_stock(true, false);
    println!("{:?}", availability);
    let availability = is_item_in_stock(false, true);
    println!("{:?}", availability);
    let availability = is_item_in_stock(false, false);
    println!("{:?}", availability);

    let present_value = Option::Some(13);
    let missing_value: Option<i32> = Option::None;
    println!("Present value: {:?}", present_value.unwrap_or(0));
    println!("Missing value: {:?}", missing_value.unwrap_or(100));

    let some_option = MyOption::Some(5);
    let none_option = MyOption::None;
    println!("Some option: {:?}", some_option.unwrap());
    println!("None option: {:?}", none_option.unwrap_or(10));

    let ok: Result<i8, &str> = Result::Ok(5);
    let err: Result<i32, &str> = Result::Err("Error");
    println!("Ok: {:?}", ok);
    println!("Err: {:?}", err);

    let text = "50";
    let number = text.parse::<i32>();
    println!("Number: {:?}", number);

    let text = "Chennai";
    let number = text.parse::<i32>();
    println!("Number: {:?}", number);

    let result = divide(10.0, 2.0);
    println!("Result: {:?}", result);
    println!("Result: {}", result.as_ref().unwrap());
    println!("Result: {}", result.as_ref().is_ok());

    let result = divide(10.0, 0.0);
    println!("Result: {:?}", result);
    println!(
        "Result: {:?}",
        result.expect_err("Denominator cannot be zero")
    );

    let mut sauces = vec!["Ketchup", "Mustard", "Mayonnaise", "Sriracha"];
    while let Some(sauce) = sauces.pop() {
        println!("Sauce: {}", sauce);
    }

    coding_challenge();
}

fn play_instrument(instrument: Option<&String>) {
    match instrument {
        Some(instrument) => println!("Playing {}", instrument),
        None => println!("Singing with my voice"),
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true)
    } else if item_is_in_system {
        Some(false)
    } else {
        None
    }
}
