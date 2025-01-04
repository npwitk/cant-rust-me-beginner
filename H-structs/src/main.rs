// struct Crabby {
//     name: String,
//     skill_level: u32,
//     hit_points: i32,
// }
// fn main() {
//     let crabby = Crabby {
//         name: String::from("Crabby"),
//         skill_level: 10,
//         hit_points: 100,
//     };
//     println!(
//         "{} has skill level {} and {} hit points.",
//         crabby.name, crabby.skill_level, crabby.hit_points
//     );
// }

// ---------

struct Crabby2 {
    name: String,
    health: u8, //max 100 (let's say)
}

impl Crabby2 {
    fn take_damage(&mut self, damage: u8) {
        // self.health -= damage;
        self.health = self.health.saturating_sub(damage); //prevent overflow
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
        } else {
            self.health += heal;
        }
    }
}

fn main() {
    let mut crabby = Crabby2 {
        name: "Crabby".to_string(),
        health: 100,
    };

    crabby.take_damage(100);
    crabby.take_damage(10);
    println!("Crabby health: {}", crabby.health);

    crabby.healing(60);
    println!("Crabby health: {}", crabby.health);
}
