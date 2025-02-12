fn longest_map<'a>(map_1: &'a str, map_2: &'a str) -> &'a str {
    if map_1.len() > map_2.len() {
        map_1
    } else {
        map_2
    }
}

fn main() {
    let map_1 = "Ancient Map of the Sea";
    let map_2 = "Map to Hidden Gold";

    let chosen_map = longest_map(map_1, map_2);
    println!("Crabby's longest map: {}", chosen_map);
}
