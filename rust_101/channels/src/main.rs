use std::{
    sync::{mpsc, Arc},
    thread,
};

fn main() {
    let loots = vec![10, 20, 30];
    let mut crabby_gold_coin = 100;

    let (sender, receiver) = mpsc::sync_channel(loots.len());
    let sender_arc = Arc::new(sender);

    for loot in loots.clone().into_iter() {
        thread::spawn({
            let sender = Arc::clone(&sender_arc);
            move || {
                sender.send(loot).unwrap();
            }
        });
    }

    for _ in 0..loots.len() {
        let loot = receiver.recv().unwrap();
        crabby_gold_coin += loot;
    }

    println!("Crabby has {} gold coins", crabby_gold_coin);
}
