fn main() {
    let mut treasure = String::from("gold coins");

    let friend1 = &treasure;
    let friend2 = &treasure;

    println!("Friend 1 sees: {}", friend1);
    println!("Friend 2 sees: {}", friend2);

    let trusted_friend = &mut treasure;

    trusted_friend.push_str(" & silver coins");
    println!("Trusted friend updates: {}", trusted_friend);
}
