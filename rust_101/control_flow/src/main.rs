fn main() {
    let weather = "rainy";
    if weather == "sunny" {
        println!("Crabby will cross the river by swimming! 🌞");
    } else if weather == "rainy" {
        println!("Crabby will build a bridge to stay dry. ☔");
    } else {
        println!("Crabby will wait for better weather. ⛈️");
    }

    let monster = "dragon";
    match monster {
        "goblin" => println!("He uses his rusty sword to attack. 🦀⚔️"),
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
