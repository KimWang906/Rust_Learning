use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    //expect
    let input_number: u32 = input.trim().parse().expect("Please type a number!");

    println!("{}", input_number);
}