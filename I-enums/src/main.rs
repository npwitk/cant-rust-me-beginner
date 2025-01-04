// enum CrabbyState {
//     Resting,
//     Fighting,
//     Collecting(u32),
//     Defending,
// }

// fn main() {
//     // let current_state = CrabbyState::Fighting;
//     let current_state = CrabbyState::Collecting(15);

//     match current_state {
//         CrabbyState::Resting => println!("Crabby is resting."),
//         CrabbyState::Fighting => println!("Crabby is fighting bad crabs!"),
//         CrabbyState::Collecting(amount) => println!("Crabby is collecting {} treasures.", amount),
//         CrabbyState::Defending => println!("Crabby is defending his treasure."),
//     }
// }

// -----
enum CrabbyStates {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyStates {
    fn state_represent(&self) {
        match self {
            CrabbyStates::Fighting => println!("Crabby is fighting"),
            CrabbyStates::Collecting(amount) => println!("Crabby is collecting {}", amount),
            CrabbyStates::Defending => println!("Crabby is defending"),
        }
    }
}

fn main() {
    let fighting = CrabbyStates::Fighting;
    let collecting = CrabbyStates::Collecting(15);
    let defending = CrabbyStates::Defending;

    fighting.state_represent();
    collecting.state_represent();
    defending.state_represent();
}
