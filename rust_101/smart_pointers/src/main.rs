use std::{cell::RefCell, rc::Rc};

fn main() {
    let chest = Box::new(10);

    let shared_chest = Rc::new(RefCell::new(chest));

    **shared_chest.borrow_mut() += 10;
    **shared_chest.borrow_mut() += 5;

    println!("Gold: {}", shared_chest.borrow());

    let gear_chest = Box::new(String::from("Sword"));

    let shared_gear_chest = Rc::new(RefCell::new(gear_chest));

    shared_gear_chest.borrow_mut().push_str(" & Shield");

    println!("Gear Chest: {}", shared_gear_chest.borrow());
}
