mod coding_challenge;

use crate::coding_challenge::*;
use std::io;

fn main() {
    let pirate = "Bloodhook";
    let sailer = String::from(pirate);
    let bad_guy = pirate.to_string();
    println!("{}, {}, {}", pirate, sailer, bad_guy);

    let first_char = &sailer[0..1];
    println!("{first_char}");

    let mut full_name = String::from("Sivaprasad");
    let last_name = "Chidambaram";
    full_name.push(' ');
    full_name.push_str(last_name);
    println!("{full_name}");

    let mut f_name = String::from("Sivaprasad");
    let l_name = String::from("Chidambaram");
    f_name.push_str(&l_name);
    println!("{f_name}");

    let first_name = String::from("Sivaprasad");
    let last_name = String::from("Chidambaram");
    let full_name = first_name.clone() + &last_name;
    println!("{full_name}");
    println!("{first_name}");

    let icon = format!("{0} {1} {0}", first_name, last_name);
    println!("{icon}");

    let music_genres = "      Rock, Metal, Country, Rap     ";
    println!("\"{}\"", music_genres.trim());
    println!("\"{}\"", music_genres.trim_start());
    println!("\"{}\"", music_genres.trim_end());

    println!("\"{}\"", music_genres.trim().to_uppercase());
    println!("\"{}\"", music_genres.trim().to_lowercase());

    println!("\"{}\"", music_genres.replace("a", "@"));

    let genres = music_genres.trim().split(", ").collect::<Vec<&str>>();
    println!("{:?}", genres);

    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get input from the user!");
    println!("Hello, {}!", name);

    println!("What is your name: ");
    name.clear();
    match io::stdin().read_line(&mut name) {
        Ok(n) => println!("{} bytes read, name: {}", n, name.trim()),
        Err(e) => println!("Error: {}", e),
    }

    challenge();
}
