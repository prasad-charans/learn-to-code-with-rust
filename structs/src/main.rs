mod coding_challenge;
use crate::coding_challenge::*;
use chrono::{Datelike, Utc};

#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    fn display_song_info(&self) {
        println!(
            "{} was released in {} and lasts for {} seconds.",
            self.title, self.release_year, self.duration_secs
        );
    }

    fn double_duration(&mut self) {
        self.duration_secs *= 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        Utc::now().year() as u32 - self.release_year
    }
}

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }
}

impl Computer {
    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, additional_memory: u32) -> &mut Self {
        self.memory += additional_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, additional_capacity: u32) -> &mut Self {
        self.hard_drive_capacity += additional_capacity;
        self
    }
}

// Tuple struct
struct ShortDuration(u32, u32);
struct LongDuration(u32, u32);

//Unit struct
#[derive(Debug)]
struct Empty;

fn main() {
    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 4.50,
        is_hot: true,
    };

    beverage.name = String::from("Cappuccino");
    beverage.price = 5.99;

    println!(
        "My {} this morning cost {}. It is {} that it is hot.",
        beverage.name, beverage.price, beverage.is_hot
    );

    let coffee = make_coffee(String::from("Latte"), 3.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it is hot.",
        coffee.name, coffee.price, coffee.is_hot
    );

    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        ..coffee
    };
    println!(
        "My {} this morning cost {}. It is {} that it is hot.",
        caramel_macchiato.name, caramel_macchiato.price, caramel_macchiato.is_hot
    );
    println!("Original coffee is still available: {}", coffee.name);
    drink_coffee(&coffee);
    drink_coffee(&caramel_macchiato);
    println!("{} is delicious!", coffee.name);
    println!("{:?}", beverage);
    println!("{:?}", coffee);
    println!("{:?}", caramel_macchiato);

    let mut song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };
    song.display_song_info();
    println!("{:?}", song);

    song.double_duration();
    println!("{:?}", song);

    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2021,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!("Blank Space is longer than All Too Well.");
    } else {
        println!("All Too Well is longer than Blank Space.");
    }
    println!(
        "Blank Space was released {} years ago.",
        blank_space.years_since_release()
    );
    println!(
        "All Too Well was released {} years ago.",
        all_too_well.years_since_release()
    );

    let mut computer = Computer::new(String::from("M3 Pro"), 64, 512);
    computer
        .upgrade_cpu(String::from("M3 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(1024);
    println!("{:?}", computer);

    let work_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);
    go_to_work(work_shift);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    let my_empty_struct = Empty;
    println!("Empty struct {:?}", my_empty_struct);

    coding_challenge();
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        price,
        name,
        is_hot,
    }
}

fn drink_coffee(coffee: &Coffee) {
    println!("Drinking my delicious {}!", coffee.name);
}

fn go_to_work(shift: ShortDuration) {
    println!(
        "I'm going to work for {} hours and {} minutes.",
        shift.0, shift.1
    );
}
