#[derive(Debug)]// So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        //Coin:Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Hey, nice hat!");
}
fn remove_fancy_hat() {
    println!("Oh no!, The wind blow up your hat ;~;.");
}
fn move_player(num_spaces: u8) { 
    println!("The player move's {num_spaces} spaces!");
}
fn reroll() {}

fn result_roll(result:u8) {
    
    match result {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        //_ => reroll (),
        //_ => (),
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    println!("Six: {six:?}");
    let none = plus_one(None);
    println!("None: {none:?}");
    
    let dice_roll = 9;

    result_roll(dice_roll);

    let dice_roll_2 = 3;

    result_roll(dice_roll_2);

    let dice_roll_3 = 7;

    result_roll(dice_roll_3);


}
