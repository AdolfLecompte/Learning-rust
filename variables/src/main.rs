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
    println!("This is a character -> {z}");

    let z: char 'ℤ' //explicit
    println!("This is a character -> {z}");

}
