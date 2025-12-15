use std::io;

fn main() {
    println!("Please enter 2 numbers you need to find greatest common diviser for:");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read the line.");

    let parsed_number = user_input.trim().parse::<i32>();

    let number = match parsed_number {
        Ok(num)=>num,
        Err(_) => {
            println!("That input can't be parsed, enter a number");
            0
        }
    };
    
    println!("User input: {}", number);    
}
