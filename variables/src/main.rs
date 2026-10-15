use std::io;

fn main(){
    // Mutable and immutable
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The vualue of x is: {x}");

    println!("------------------------------------------");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of your space is: {spaces}");

    println!("------------------------------------------");

    //floating
    let y: f64 = 2.5;
    println!("This number -> {y} is a 64-bit float");

    let z: f32 = 3.5;
    println!("This number -> {z} is a 32-bit float");

    println!("------------------------------------");

    // numerical operations
    let sum = 5 + 10;
    println!("This is an addition -> 5 + 10 = {sum}");

    let difference = 95.5 - 4.3;
    println!("This is a subtraction -> 95.5 - 4.3 ={difference}");

    let product = 4 * 30;
    println!("This is a multiplication -> 4 * 30 = {product}");

    let quotient = 56.7 / 32.2;
    println!("This is a division -> 56.7 / 32.2 = {quotient}");

    let remainder = 43 % 5;
    println!("This is a remainder -> 43 % 5 = {remainder}");

    // booleans
    let t = true;
    println!("This is true -> {t}");

    let f: bool = false; //explicit
    println!("This is false -> {f}");

    // characters
    let c = 'z';
    println!("This is a character -> {c}");

    let z: char = 'ℤ'; //explicit
    println!("This is a character -> {z}");

    let heart_eyed_cat = '😻';
    println!("This is a character -> {heart_eyed_cat}");

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4 ,1);

    let (x,y,z) = tup;
    println!("The value of x is -> {x}");
    println!("The value of y is -> {y}");
    println!("The value of z is -> {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The first value of the tuple -> {five_hundred}");

    let six_point_four = x.1;
    println!("The first value of the tuple -> {six_point_four}");

    let one = x.2;
    println!("The first value of the tuple -> {one}");

    //array
    let a = [1,2,3,4,5];
    println!("This is a array -> {a:?}");

    let months = ["January", "February", "March","April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let six_month = months[5];
    println!("The six month is -> {six_month}");

    let a: [i32; 5] = [1,2,3,4,5];

    let three = a[2];
    println!("The third number in the array is -> {three}");

    let ar = [3; 5];
    println!("ar {ar:?}");

    let a = [1,2,3,4,5];

    let first = a[0];
    println!("The first number of array is -> {first}");
    let second = a[1];
    println!("The second number of array is -> {second}");

    let ar = [1,2,3,4,5];
    println!("ar {ar:?}");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at the index {index} is -> {element}");

}
