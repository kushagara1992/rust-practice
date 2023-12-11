use std::io; // importing io library from standard library

fn main() {
    println!("Hello, world!");
    let mut input = String::new(); // creating a mutable string variable
    io::stdin().read_line(&mut input).expect("Failed to read line"); // reading input from user and storing it in input variable
    println!("input = {}", input); // printing the input variable
}
