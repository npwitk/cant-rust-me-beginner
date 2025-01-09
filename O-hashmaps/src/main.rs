use std::collections::HashMap;

fn main() {
    let mut treasures = HashMap::new();

    treasures.insert("Gold coins", 10); // key-value
    treasures.insert("Gems", 1000);

    if let Some(gem) = treasures.get("Gem") {
        println!("Gem: {}", gem);
    }

    if let Some(gem2) = treasures.get_mut("Gem") {
        *gem2 += 100;
        println!("Gem: {}", gem2);
    }

    let ruby_count = treasures.get("Ruby").unwrap_or(&0); //give default value
    treasures.remove("Gem");

    let mut jewels: HashMap<&str, i32> = HashMap::new();

    jewels.insert("Silver Coins", 100);
    jewels.insert("Ruby", 3);

    if let Some(ruby) = jewels.get_mut("Ruby") {
        *ruby += 5;
    }

    println!("Jewels: {:?}", jewels);
}
