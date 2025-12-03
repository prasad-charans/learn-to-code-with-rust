mod coding_challenge;
use crate::coding_challenge::code_challenge;

fn main() {
    //Arrays
    let numbers = [4,8,15,16,23,42];

    let apples = ["Shimla", "American", "Royal Gala"];

    println!("Apples Length: {}", apples.len());

    let currency_rates : [f64; 0] = [];

    let seasons = ["Spring", "Summer", "Winter", "Autumn"];
    let first = seasons[0];
    let second = seasons[1];
    println!("The first season is {first} and second season is {second}");

    println!("Seasons {:?}", seasons);
    println!("{seasons:#?}");

    dbg!(2 + 2);
    dbg!(seasons);

    let employee = ("John Doe", 32, "Development");
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    let (name, age, department) = employee;
    println!("Name: {name}, Age: {age}, Department: {department}");

    println!("Employee: {:?}", employee);
    println!("Employee: {:#?}", employee);

    //Ranges
    let month_days = 1..31;
    println!("{month_days:?}");

    let month_days = 1..=31;
    println!("{month_days:?}");

    for n in month_days {
        println!("{n}");
    }

    let letters = 'a'..'j';
    for l in letters {
        println!("{l}");
    }

    code_challenge();
}
