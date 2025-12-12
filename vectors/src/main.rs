mod coding_challenge;
use crate::coding_challenge::coding_challenge;

fn main() {
    let mut pizza_diameters: Vec<i32> = Vec::new();
    pizza_diameters.push(12);
    println!("Number of pizzas: {}", pizza_diameters.len());

    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.insert(0, 4);
    println!("Number of pizzas: {:?}", pizza_diameters);

    let last_pizza = pizza_diameters.pop();
    println!("Last pizza: {:?}", last_pizza);
    println!("Number of pizzas: {:?}", pizza_diameters);

    let third_pizza = pizza_diameters.remove(2);
    println!("Third pizza: {:?}", third_pizza);
    println!("Number of pizzas: {:?}", pizza_diameters);

    let pizza_diameters = vec![8, 10, 12, 14];

    let pepporini = String::from("Pepporini");
    let margherita = String::from("Margherita");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepporini, margherita, sausage];

    let value = pizza_diameters[2];
    println!("Value: {:?}", value);

    let topping = &pizza_toppings[2];
    println!("Topping: {:?}", topping);

    let pizza_slice = &pizza_toppings[1..];
    println!("Pizza slice: {:?}", pizza_slice);

    let option = pizza_toppings.get(2);
    println!("Option: {:?}", option);

    match option {
        Some(topping) => println!("Topping: {}", topping),
        None => println!("No topping"),
    }

    let mut delicious_toppings = pizza_toppings;
    let topping_reference = &delicious_toppings[1];
    println!("Topping reference: {}", topping_reference);
    delicious_toppings.push(String::from("Bacon"));

    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
    seasons.push("Spring");
    seasons.push("Summer");
    seasons.push("Autumn");
    seasons.push("Winter");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
    seasons.push("Spring");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
    coding_challenge();
}
