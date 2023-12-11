fn main() {
    println!("Hello, world!");
    //scalar datatype
    let x = 2; //implicit type assignment
    let y: i32 = 3; //explicit type assignment
    // i8, i16, i32, i64, i128, isize can use negative numbers
    // u8, u16, u32, u64, u128, usize can't use negative numbers
    // floating type values
    let z = 2.0; //implicit type assignment
    let w: f32 = 3.0; //explicit type assignment
    // boolean type
    let t = true; //implicit type assignment // 1 for true and 0 for false
    let f: bool = false; //explicit type assignment
    // character type
    let c = 'z'; //implicit type assignment
    let d: char = 'x'; //explicit type assignment

    //compound datatype
    // tuple
    let tup = (1, true, 's', 4, 5); //implicit type assignment // fixed length sequence of elements which are immutable, can be different types
    let tup: (i32, bool, char, i32, i32) = (1, true, 's', 4, 5); //explicit type assignment
    // accessing from tuple
    println!("tup_0 = {}", tup.0);
    println!("tup_1 = {}", tup.1);
    println!("tup_2 = {}", tup.2);

    // mutate tuple
    let mut tup = (1, true, 's', 4, 5);
    tup.0 = 2;
    println!("tup_0 = {}", tup.0);

    // array
    let arr = [1, 2, 3, 4, 5]; //implicit type assignment // fixed length sequence of elements which are immutable, has to be same types
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; //explicit type assignment
    // accessing from array
    println!("arr_0 = {}", arr[0]);
    println!("arr_1 = {}", arr[1]);
    
    //mutable array
    let mut arr_1 = [1, 2, 3, 4, 5];
    arr_1[0] = 2;
    println!("arr_1_0 = {}", arr_1[0]);


}
