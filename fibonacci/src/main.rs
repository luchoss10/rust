fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

//fn fibonacci(n: i32) -> i32 {
//    if n <= 1 {
//        return n;
//    }
//    return fibonacci(n - 1) + fibonacci(n - 2);
//}

fn fibonacci(n: i32) -> Vec<i32> {
    let mut fibonacci_numbers: Vec<i32> = Vec::new();

    if n == 1 || n == 0 {
        fibonacci_numbers.push(1);
        return fibonacci_numbers;
    }

    if n == 2 {
        fibonacci_numbers.push(1);
        fibonacci_numbers.push(1);
    }

    let mut a: i32 = 0;
    let mut b: i32 = 1;
    for _i in 0..n {
        let current: i32 = a + b;
        a = b;
        b = current;
        fibonacci_numbers.push(current);
    }

    return fibonacci_numbers;
}

fn main() {
    println!("Hi! This program calculate the fibonacci nth number");
    println!("Enter the nth number please: ");
    let number = read_string().trim().parse::<i32>().unwrap();
    let calculous: Vec<i32> = fibonacci(number);
    println!("The susesion is {:?}", calculous);
}
