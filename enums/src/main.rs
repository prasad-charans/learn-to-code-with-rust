#[derive(Debug)]
enum CardSuite {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

struct Card {
    rank: String,
    suite: CardSuite,
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Upi(String),
    Paypal(Credentials),
    Wallet { username: String, password: String },
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
    Red,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Mutton,
    Fish,
    Egg,
    Veg,
}

#[derive(Debug)]
enum RestaurantItem {
    Kichidi,
    Rice { meat: Meat, beans: Beans },
    Dhal,
    Naan,
    Bowl { meat: Meat, beans: Beans },
}

enum OperatingSystem {
    Windows,
    Linux,
    MacOS,
}

enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => println!("Washing laundry in cold water"),
            LaundryCycle::Hot { temperature } => println!(
                "Washing laundry in hot water with temperature {}",
                temperature
            ),
            LaundryCycle::Delicate(fabric_type) => {
                println!("Washing delicate laundry of type {}", fabric_type)
            }
        }
    }
}

enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered => println!("Order is ordered"),
            OnlineOrderStatus::Packed => println!("Order is packed"),
            OnlineOrderStatus::Shipped => println!("Order is shipped"),
            OnlineOrderStatus::Delivered => println!("Order is delivered"),
        }
    }
}

enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

impl Milk {
    fn drink(&self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious 2% milk.")
            }
            Milk::Lowfat(percent) => {
                println!("Lowfat {percent}% milk")
            }
            Milk::Whole => {
                println!("Delicious whole milk.")
            }
            Milk::NonDiary { kind } => {
                println!("Non-dairy {kind}")
            }
        }
    }
}

fn main() {
    let first_card = CardSuite::Hearts;
    println!("{:?}", first_card);
    let mut second_card = CardSuite::Diamonds;
    println!("{:?}", second_card);
    second_card = CardSuite::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuite::Hearts, CardSuite::Clubs];
    println!("{:?}", card_suits);

    let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let mastercard = PaymentMethodType::CreditCard(String::from("2532-5678-9012-3456"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);

    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    my_payment_method = PaymentMethodType::Upi(String::from("prasad@ybl"));
    println!("{:?}", my_payment_method);

    let paypal_cred = Credentials {
        username: String::from("prasad@ybl"),
        password: String::from("password"),
    };
    let paypal = PaymentMethodType::Paypal(paypal_cred);
    println!("{:?}", paypal);

    let wallet = PaymentMethodType::Wallet {
        username: String::from("prasad@ybl"),
        password: String::from("password"),
    };
    println!("{:?}", wallet);

    let lunch = RestaurantItem::Rice {
        meat: Meat::Chicken,
        beans: Beans::Pinto,
    };
    println!("{:?}", lunch);

    let dinner = RestaurantItem::Bowl {
        meat: Meat::Egg,
        beans: Beans::Black,
    };
    println!("{:?}", dinner);

    let my_os = OperatingSystem::Windows;
    let years = years_since_release(my_os);
    println!("Years since release: {}", years);

    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 40 });
    wash_laundry(LaundryCycle::Delicate(String::from("Cotton")));

    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 40 }.wash_laundry();
    LaundryCycle::Delicate(String::from("Cotton")).wash_laundry();

    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivered.check();

    Milk::Lowfat(2).drink();
    Milk::Lowfat(1).drink();
    Milk::Whole.drink();

    let my_milk = Milk::Lowfat(2);
    if let Milk::Lowfat(percent) = my_milk {
        println!("Your milk is {percent}% fat.")
    }

    let my_another_milk = Milk::NonDiary {
        kind: String::from("Almond"),
    };
    if let Milk::NonDiary { kind } = my_another_milk {
        println!("Your milk is {kind}")
    }

    let whole_beverage = Milk::Whole;
    let Milk::Lowfat(percent) = whole_beverage else {
        println!("You do not have a lowfat milk.");
        return;
    };
    println!("Your milk is {percent}% fat.");
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::Linux => 34,
        OperatingSystem::MacOS => 23,
    }
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running Laundry in Cold temperature");
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running Laundry in Hot temperature: {temperature}");
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running Laundry in Delicate mode {fabric_type}");
        }
    }
}
