enum Tier {
    Platinum,
    Diamond,
    Master,
}

fn main() {
    let minu_tier = Tier::Platinum;

    let result = match minu_tier {
        Tier::Diamond => println!("Error founded"),
        Tier::Master => println!("?"),
        Tier::Platinum => println!("플딱"),
    };
}
