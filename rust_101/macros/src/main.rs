use macros::log_time;

macro_rules! magic_spelling {
    (fire) => {
        println!("FIRE!");
    };
    (water) => {
        println!("WATER!");
    };
}

#[log_time]
fn main() {
    // Crabby Spelling with log time
    magic_spelling!(fire);
    magic_spelling!(water);
}
