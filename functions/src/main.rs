fn five() -> i32 {
    // Rust return the value of the last expression without a ;
    // In a function
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    //println!("Hello, world!");
    another_function(5);
    //

    print_labeled_measurement(5, 'h');

    // In Rust, the last expression in a block of code is the return value of the block
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is:{x}");

    let x = plus_one(5);

    println!("The value of x is: {x}")
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("Then value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}
