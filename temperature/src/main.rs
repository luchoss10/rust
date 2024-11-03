fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn main() {
    println!("Hello, This program convert the temperature to celcius or farenheit!");

    let mut temperature: f32;
    let mut converted_temperature: f32;

    loop {
        println!("Select the system you want to convert to: ");
        println!("1. Celcius");
        println!("2. Farenheit");
        println!("3. Exit");

        let choice = read_string().trim().parse::<i32>().unwrap();
        if choice == 1 {
            println!("Enter the temperature in Celcius: ");
            temperature = read_string().trim().parse::<f32>().unwrap();
            converted_temperature = ((9.0 / 5.0) * temperature) + 32.0;
            println!("The temperature in Farenheit is: {}", converted_temperature);
        } else if choice == 2 {
            println!("Enter the temperature in Farenheit: ");
            temperature = read_string().trim().parse::<f32>().unwrap();
            converted_temperature = (temperature - 32.) * (5. / 9.);
            println!("The temperature in Celcius is : {}", converted_temperature);
        } else if choice == 3 {
            break;
        } else {
            println!("The selected choice is not correct!!!!");
        }
    }
}
