use std::{cell::RefCell, rc::Rc};

fn main() {
    let chest = 100; // Box::new(100)
    let shared_chest = Rc::new(RefCell::new(chest));

    *shared_chest.borrow_mut() += 5;
    println!("Added 5 coins - Total coins: {}", shared_chest.borrow());

    *shared_chest.borrow_mut() += 5;
    println!("Added 5 coins - Total coins: {}", shared_chest.borrow());
}
