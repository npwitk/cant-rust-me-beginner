// fn main() {
//     let treasures = vec!["Gold", "Ruby", "Emerald"];
//     let treasure = treasures.get(3);

//     match treasure {
//         Some(t) => println!("Found treasure: {}", t),
//         None => println!("No treasure found!"),
//     }

//     if let Some(treasure) = treasures.get(3) {
//         //อีกแบบคล้าย Swift 555 love!
//         println!("Found treasure: {}", treasure)
//     } else {
//         println!("No treasure found!")
//     }

//     let gem = treasures.get(3).unwrap_or(&"None");
//     let gem2 = treasures.get(3).unwrap_or_else(|| &"None"); //อันนี้ต้องเขียนเป็น closure
//     let gem3 = treasures.get(3).unwrap; //ไม่สน

// }

fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a treasure!".to_string())
    }
}

fn open_door(is_danger: bool) -> Result<String, String> {
    if is_danger {
        Err("You found a monster!".to_string())
    } else {
        Ok("The door is safe".to_string())
    }
}

fn main() {
    let chest_result = match open_chest(true) {
        Some(treasure) => treasure,
        None => "The chest is empty".to_string(),
    };

    println!("{}", chest_result);

    let door_result = match open_door(false) {
        Ok(safe) => safe,
        Err(mimic) => panic!("{}", mimic),
    };

    println!("{}", door_result);
}
