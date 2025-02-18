use tokio;

async fn gather_herbs() {
    println!("Crabby is gathering herbs...");
}

async fn collect_gold_coins() {
    println!("Crabby is collecting gold coins...");
}

async fn fight_monster() {
    println!("Crabby is fighting the monster!");
}

#[tokio::main]
async fn main() {
    // Crabby wants all quests to run at the same time!

    let gather_herbs_task = tokio::spawn(gather_herbs());
    let collect_gold_coins_task = tokio::spawn(collect_gold_coins());
    let fight_monster_task = tokio::spawn(fight_monster());

    let _ = tokio::join!(
        gather_herbs_task,
        collect_gold_coins_task,
        fight_monster_task,
    );
}
