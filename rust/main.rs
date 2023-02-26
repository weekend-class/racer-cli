use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    while stdin.read_line(&mut buffer).is_ok() {
        // Trim end.
        let trimmed = buffer.trim();
        let inputs = trimmed.split_whitespace();

        if inputs.to_owned().count() != 3 {
            println!("Invalid argument, try again");
            continue;
        }

        let first_number =
            i32::from_str_radix(inputs.to_owned().nth(0).unwrap_or("0"), 10).unwrap_or(0);
        let sign = inputs.to_owned().nth(1).unwrap();
        let second_number =
            i32::from_str_radix(inputs.to_owned().nth(2).unwrap_or("0"), 10).unwrap_or(0);
        let result;

        // println!("Input {:?}, {:?}, {:?}", inputs.nth(0), sign, inputs.nth(1));

        match sign {
            "+" => result = first_number + second_number,
            "-" => result = first_number - second_number,
            "*" => result = first_number * second_number,
            "/" => result = first_number / second_number,
            "^" => result = first_number.pow(second_number as u32),
            "%" => result = first_number % second_number,
            _ => {
                println!("Invalid command, try again");
                continue;
            }
        }

        println!("Result is: {}", result);
        buffer.clear();
    }
}
