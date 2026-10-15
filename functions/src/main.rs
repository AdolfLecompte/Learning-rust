fn main() {
    another_function(5);
    print_labeled_measurements(5, 'h');
    statements_expressions();
    fun();

    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32){
    println!("The value of x is -> {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char){
    println!("The measurement is -> {value}{unit_label}");
}

// Statements and expressions
fn statements_expressions(){
    let y = 6;
    println!("{y}")
}

fn fun() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
