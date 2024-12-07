fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`
                     //
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); // This will throw an error because s1 has been moved to s2
    println!("{s2}"); // This will print `hello`

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {s3}, s4 = {s4}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s5 = String::from("hello");
    takes_ownership(s5); // s5's value moves into the function and so is no longer valid here

    let x = 5;
    makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x
                   // afterward

    let _s6 = gives_ownership(); // gives_ownership moves its return value into s6
                                 // so s6 is valid here
    let s7 = String::from("hello");
    let _s8 = takes_and_gives_back(s7); // s7 is moved into takes_and_gives_back, which also moves its
                                        // return value into s8
                                        //
    let s9 = String::from("hello");

    let (s10, len) = calculate_length(s9);

    println!("The length of '{s10}' is {len}");
} // Here, x goes out of scope, then s5. But because s5's value was moved, nothing special happens.

fn takes_ownership(u_string: String) {
    // u_string comes into scope
    println!("{u_string}");
} // Here, u_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(u_int: i32) {
    // u_int comes into scope
    println!("{u_int}");
} // Here, u_int goes out of scope. Nothing special happens.
  //
fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
                // where it is assigned to s6
                // some_string goes out of scope and `drop` is called. The backing memory is freed.
}

fn takes_and_gives_back(u_string: String) -> String {
    // u_string comes into scope
    u_string // u_string is returned and moves out to the calling function
             // where it is assigned to s8
             // u_string goes out of scope and `drop` is called. The backing memory is freed.
}

fn calculate_length(u_string: String) -> (String, usize) {
    let length = u_string.len(); // len() returns the length of a String
    (u_string, length)
}
