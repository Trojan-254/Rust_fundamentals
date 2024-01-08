// Control flow
use std::io;
// If expressions

fn main() {
    loop {

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
            println!("Invalid input, please input a valid integer")
        }
    }

    }
}
