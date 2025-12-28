mod coding_challenge;
use coding_challenge::coding_challenge;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("Burger"), 12.99);
    menu.insert(String::from("Pizza"), 15.99);
    menu.insert(String::from("Pasta"), 10.99);

    println!("{:?}", menu);

    let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");
    country_capitals.insert("Italy", "Rome");

    println!("{:?}", country_capitals);

    let data = [("Bobby", 7), ("Alice", 8), ("Ben", 6)];
    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("Ben has been at the company for {} years", ben.unwrap());
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("Ben: {:?}", ben);

    let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairing.insert(&drink, &milk);
    coffee_pairing.insert("Espresso", "Almond Milk");
    println!("{:?}", coffee_pairing);
    println!("{}", milk);

    let value = coffee_pairing
        .get("Espresso")
        .copied()
        .unwrap_or("Unkown Milk");
    println!("{}", value);

    let value = coffee_pairing
        .get("Capuccino")
        .copied()
        .unwrap_or("Unkown Milk");
    println!("{}", value);

    coffee_pairing.insert("Latte", "Pistachio Milk");
    println!("{:?}", coffee_pairing);

    coffee_pairing.entry("Latte").or_insert("Full Milk");
    println!("{:?}", coffee_pairing);

    coffee_pairing.entry("Capuccino").or_insert("Low Fat Milk");
    println!("{:?}", coffee_pairing);

    // HashSet
    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Bobby");
    concert_queue.insert("Alice");
    concert_queue.insert("Ben");
    println!("{:?}", concert_queue);

    concert_queue.insert("Bobby");
    println!("{:?}", concert_queue);

    concert_queue.remove("Alice");
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.contains("Alice"));

    let mut movie_queue: HashSet<&str> = HashSet::new();
    movie_queue.insert("Bobby");
    movie_queue.insert("Alice");
    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_subset(&movie_queue));

    let mut attendees: HashSet<&str> = HashSet::new();
    attendees.insert("Bobby");
    println!("{:?}", attendees.is_subset(&concert_queue));
    println!("{}", concert_queue.is_superset(&attendees));

    coding_challenge();
}
