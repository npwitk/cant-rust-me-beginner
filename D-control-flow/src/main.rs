fn main() {
    let weather = "sunny";

    if weather == "sunny" {
        println!("Crabby will cross the river by swimming! 🌞");
    } else if weather == "rainy" {
        println!("Crabby will build a bridge to stay dry. ☔");
    } else if weather == "stormy" {
        println!("Crabby will wait for better weather. ⛈️");
    }

    let monster = "dragon";

    match monster {
        "goblin" => println!("he uses his rusty sword to attack. 🦀⚔️"),
        "troll" => println!(" Crabby sets a trap! 🐾"),
        "dragon" => println!("Crabby runs for cover! 🐉🏃‍♂️"),
        _ => println!("Crabby is confused... 😵"),
    };

    let mut wood = 0;

    loop {
        wood += 1;

        println!("Crabby gathered {} pieces of wood.", wood);

        if wood == 10 {
            println!("Crabby finished the boat!🛶");
            break;
        }
    }
}
