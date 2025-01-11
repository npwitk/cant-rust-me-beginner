use std::{cell::RefCell, rc::Rc};

fn main() {
    let chest = 10;

    let shared_chest = Rc::new(RefCell::new(chest)); //RefCell เพื่อให้ mut ได้

    *shared_chest.borrow_mut() *= 10; //* คือ pointer ชั้นเดียว
    *shared_chest.borrow_mut() += 5;

    println!("Gold: {}", shared_chest.borrow());
}
