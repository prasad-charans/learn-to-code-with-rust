use fake::{Fake, Faker};

use warehouse::ProductCategory;

fn main() {
    let random_category = Faker.fake::<ProductCategory>();
    println!("Random category: {:?}", random_category);
}
