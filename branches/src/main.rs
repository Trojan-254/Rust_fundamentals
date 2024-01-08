// Control flow
use std::io;
// If expressions

fn main() {
    
    println!("Please enter your input");
    println!(" ");
    let mut number = String::new();

    io::stdin()
        .read_line(  &mut number)
        .expect("failed to read line");

    let num: Result<i32, _> = number.trim().parse();

    match num {
        Ok(parsed_num) => {
            if parsed_num % 4 == 0{
                println!("number is divisible by 4");
            } else if parsed_num % 3 == 0 {
                println!("Number is divisible by 3");
            } else if parsed_num % 2 == 0 {
                println!("Number is divisible by 2");
            } else {
                println!("Number is not divisible by 4, 3, or 2");
            }
        }
        Err(_) => {
            println!("Invalid input, please input a valid integer");
            
        }
    }

    // Using if in a let statement
    let condition = true;

    let new_number = if condition { 5 } else { 6 };

    println!("The value on number is: {new_number}");


    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is: {result}");
    
}
