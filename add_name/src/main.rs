use std::io;


fn add_character(name: &mut String, addition: &String) {
    name.push_str(addition.trim_end());
}

fn main() {
    

    let mut user_name = String::new(); 
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut user_name)
        .expect("Failed to read the line!");

    println!("Hello world {}!", user_name.trim());

    let mut add_characters = String::new();
    println!("Add your last name:");
    io::stdin()
        .read_line(&mut add_characters)
        .expect("Failed to read the line!");
    
    add_character(&mut user_name, &add_characters);

    println!("Your full name is {}", user_name.trim());
}
