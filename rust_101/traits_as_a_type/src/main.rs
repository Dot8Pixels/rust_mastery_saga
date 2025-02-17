trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Fire arrow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drink potion");
    }
}

// // Static Dispatch -> Generics
// fn use_gear<T: Gear>(item: T) {
//     item.use_gear();
// }

// // Static Dispatch -> impl
// fn use_gear(item: &impl Gear) {
//     item.use_gear();
// }

// Dynamic Dispatch -> Box dyn
fn use_gear(item: Box<dyn Gear>) {
    item.use_gear();
}

fn main() {
    // // Static Dispatch -> Generics
    // let crabby_sword = Sword;
    // let crabby_bow = Bow;
    // let crabby_potion = Potion;

    // use_gear(crabby_sword);
    // use_gear(crabby_bow);
    // use_gear(crabby_potion);

    // // Static Dispatch -> impl
    // let crabby_sword = Sword;
    // let crabby_bow = Bow;
    // let crabby_potion = Potion;

    // use_gear(&crabby_sword);
    // use_gear(&crabby_bow);
    // use_gear(&crabby_potion);

    // Dynamic Dispatch -> Box dyn
    let crabby_sword = Box::new(Sword);
    let crabby_bow = Box::new(Bow);
    let crabby_potion = Box::new(Potion);

    use_gear(crabby_sword);
    use_gear(crabby_bow);
    use_gear(crabby_potion);
}
