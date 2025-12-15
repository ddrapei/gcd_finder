use std::io;

fn main() {
    println!("Please enter 2 numbers you need to find greatest common diviser for:");

    let mut user_input = String::new();

        
    io::stdin().read_line(&mut user_input).expect("Failed to read the line.");

    let mut user_input_split = user_input.split_whitespace();
    
    let first_number = user_input_split.next().unwrap();    

    let second_number = user_input_split.next().unwrap();
    
    let parsed_first_number = first_number.trim().parse::<i32>();

    let parsed_second_number = second_number.trim().parse::<i32>(); 

    
    let mut number_1 = match parsed_first_number {
        Ok(num)=>num,
        Err(_) => {
            println!("That input can't be parsed, enter a number");
            0
        }
    };

    let mut number_2 = match parsed_second_number{
        Ok(num) => num,
        Err(_) => {
            println!("That input can't be processed");
            0    
        }
    };
   

    println!("Your greatest common divider: {}",  gcd_finder(number_1, number_2));    
}

fn gcd_finder(mut number_1: i32, mut number_2: i32) -> i32 {
    let mut remainder = number_1 % number_2;
    
    while remainder != 0 {
        number_1 = number_2;
        number_2 = remainder;
        remainder = number_1 % number_2;
    }
    return number_2
}
