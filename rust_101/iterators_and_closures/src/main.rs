fn main() {
    let treasures = [100, 200, 300, 400];

    let doubled_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled_treasures);
}
