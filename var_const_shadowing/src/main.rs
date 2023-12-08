fn main() {
    let x = 42; //implicit type assignment can't change the type throughout the program (immutable variable)
    println!("x = {}", x);
    // x = 13; //error: cannot assign twice to immutable variable `x`
    // println!("x = {}", x);

    let mut y = 13; // (mutable variable so changing the value is possible)
    println!("y = {}", y);

    y = 42;
    println!("y = {}", y);

    {
        let x = x -10; // this also works, strange but works, x is picking the initial value from the outer scope
        println!("x = {}", x);
    }

    {
        let x = 142; // shadowing the name variable x in another scope
        println!("x = {}", x);
    }


    // mutating variables with same name
    let x = x+10;
    println!("x = {}", x); // this would work because we are creating a new variable with same name

    let x = "Hello"; // this also works changing the datatype of the variable
    println!("x = {}", x);

    // y = "Hello"
    // println!("y = {}", y); // this would not work because y is already declared as integer and is mutable

    // use capital letters for constants or you will get a warning from rust compiler
    const MAX_POINTS: u32 = 100_000; // constants are always immutable and always have to be declared with type 
    println!("MAX_POINTS = {}", MAX_POINTS);
}
