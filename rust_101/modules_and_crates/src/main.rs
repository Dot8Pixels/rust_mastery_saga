use modules_and_crates::calculator_1;
use modules_and_crates::calculator_2;

fn main() {
    println!("Hello, world!");

    println!("{}", calculator_1::add::add(3, 2));
    println!("{}", calculator_1::sub::sub(3, 2));

    println!("{}", calculator_2::mul::mul(3, 2));
    println!("{}", calculator_2::power::power(3, 2));
}
