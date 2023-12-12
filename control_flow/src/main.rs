fn main() {
    println!("Hello, world!");

    let condition_1 = 5 > 10;
    println!("condition_1 is {}", condition_1);

    //compound conditionals
    // && and operator || or operator and ! not operator

    let condition_2 = true || condition_1;
    println!("condition_2 is {}", condition_2);

    let condition_3 = !condition_2;
    println!("condition_3 is {}", condition_3);

    let food = "apple";

    if food == "apple" {
        println!("I like apples");
    } else if food == "orange" {
        println!("I like oranges");
    } else {
        println!("I don't like apples or oranges");
    }

}
