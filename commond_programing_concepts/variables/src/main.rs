fn main() {
    // Variable and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    // spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    // Data types
    //
    // Scalar types
    //
    // Integer types
    //
    // Length   Signed  Unsigned
    //
    // 8-bit    i8      u8
    // 16-bit   i16     u16
    // 32-bit   i32     u32
    // 64-bit   i64     u64
    //
    // isize and usize types depend on the kind of computer your program is running on: 64 bits if
    // you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    //
    //
    // Floating-point types
    //
    // f32 and f64
    //
    // Numeric operations
    //
    // Addition

    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // Subtraction

    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // Multiplication

    let product = 4 * 30;
    println!("The value of product is: {product}");

    // Division

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");

    // Division with integers results in an integers. If you want a floating-point result, you have
    // to use a floating-point number.
    //
    let quotient = 56 / 32;

    println!("The value of quotient is: {quotient}");

    // Truncation
    // The result of dividing two integers is an integer, and the fractional part is truncated.
    // If you want to get the fractional part, you have to use a floating-point number.
    // The following code will print 1.75.

    let quotient = -5 / 3;
    println!("The value of quotient is: {quotient}");

    // Remainder
    // The remainder operation can be performed using the % operator.
    // The % operator can be used with floating-point numbers.

    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Boolean type
    // The boolean type has two values: true and false.

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    // Character type
    // The character type represents a single Unicode scalar value.

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //👀

    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // Compound types
    // Compound types can group multiple values into one type.
    // Tuple type
    // A tuple is a general way of grouping together a number of values with a variety of types
    // into one compound type.

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // Accessing tuple elements using dot notation
    // You can also access a tuple element directly by using a period (.) followed by the index of
    // the value you want to access.
    // The index of the first element is 0.

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Array type
    // An array is a collection of elements of the same type.
    // Arrays in Rust have a fixed length, like tuples.
    // Once declared, the length of an array cannot be changed.
    // Arrays are useful when you want your data allocated on the stack rather than the heap.
    // The type of an array is written as [T; N], where T is the type of the elements and N is The
    // number of elements.
    // The following code creates an array of integers with a length of 5.

    let _a = [1, 2, 3, 4, 5];

    // Accessing array elements
    // You can access an element of an array using indexing.
    // The index of the first element is 0.

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    // Accessing an element that is out of bounds will result in a runtime error.

    // you can declare an array of integers with a length of 5 like this:
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // If you want to create an array that contains the same value for each element, you can
    // specify the initial value, followed by a semicolon, and then the length of the array in
    // square brackets.

    let _a = [3; 5];

    // The array a will contain 5 elements that will all be set to the value 3 initially.
    // The type of a is [i32; 5].
    // The type annotation is optional because the compiler can infer the type from the initially
    // provided values.
}
