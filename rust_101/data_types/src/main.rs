fn main() {
    let x = 5;
    let y = 10.0;

    let sum: i32 = x + y as i32;
    println!("{}", sum);

    let msg_1 = String::from("Hello World!");
    let msg_2 = "Hello World!".to_string();
    let msg_3 = "Hello World!";
    let msg_4 = format!("Point: {}, {}", x, y);

    println!("{}", msg_1);
    println!("{}", msg_2);
    println!("{}", msg_3);
    println!("{}", msg_4);
}
