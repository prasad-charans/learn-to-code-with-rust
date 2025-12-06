mod coding_challenge;
use crate::coding_challenge::*;
#[derive(Debug)]
struct DeliSandwitch {}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capitalize_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

#[derive(Debug)]
enum CheeseSteak<T> {
    Plain,
    Toppings(T),
}

fn main() {
    println!("{}", identity::<i32>(128));
    println!("{}", identity::<u32>(35546));
    println!("{}", identity::<i8>(24));
    println!("{}", identity::<&str>("Hello"));
    println!("{}", identity::<String>(String::from("Hello")));
    println!("{}", identity::<char>('a'));
    println!("{:?}", identity::<DeliSandwitch>(DeliSandwitch {}));
    let t = make_tuple(5, "hello");
    println!("{:?}", t);

    let gold_chest = TreasureChest {
        captain: String::from("Firebread"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);
    println!("{}", gold_chest.capitalize_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("BloodSail"),
        treasure: String::from("       Silver       "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);
    println!("{}", silver_chest.capitalize_captain());

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest);
    println!("{}", special_chest.capitalize_captain());

    let mushroom = CheeseSteak::Toppings("Mushroom");
    println!("{:?}", mushroom);
    let onions = CheeseSteak::Toppings("Onions".to_string());
    println!("{:?}", onions);
    let toppings = "bacon".to_string();
    let bacon = CheeseSteak::Toppings(&toppings);
    println!("{:?}", bacon);
    let mut plain: CheeseSteak<String> = CheeseSteak::Plain;
    println!("{:?}", plain);
    plain = CheeseSteak::Toppings("Sausage".to_string());
    println!("{:?}", plain);

    coding_challenge();
}

fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
