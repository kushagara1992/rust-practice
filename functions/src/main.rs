fn main() {
    println!("Hello, world!");
    test();
    let sum = add_numbers(5, 10);
    println!("Sum is {}", sum);

    let number = {
        let x = 3;
        x + 1 //no semicolon here it means returning a value for expression
    };

    println!("The value of number is: {}", number);

}


fn test() {
    println!("Test function is called!");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y //no semicolon here it means returning a value from function we can add return or not it works in both cases
}
