fn main() {
    // variables
    // By default all variables in rust are inmutables
    let mut x = 5;
    println!("The Value of x: {x}");
    x = 6;
    println!("The Value of x: {x}");

    //Constants
    // Constants are always immutable
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    // Shadowing
    // Shadowing is when you declare a new variable with the same name as a previous variable
    // The new variable shadows the previous variable
    
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
}
