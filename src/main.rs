use std::io;

fn main() {
    let mut init_temp: bool = false;

    loop {
        if init_temp {
            println!("Currently you can convert from Celsius to Fahrenheit");
        } else {
            println!("Currently you can convert from Fahrenheit to Celsius");
        }
        println!("Enter 'change' to switch the converter");
        println!("Enter 'stop' to exit");
        println!("Enter the temperature to convert");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line");

        let user_input = user_input.trim();

        if user_input == "change" {
            init_temp = !init_temp;
            continue;
        } else if user_input == "stop" {
            break println!("Bye!");
        } else {
            let user_input: i64 = match user_input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number.");
                    continue;
                }
            };

            if init_temp {
                let result: i64 = (user_input * 9 / 5) + 32;
                println!("{user_input} Celsius is {result} in Fahrenheit");
            } else {
                let result: i64 = (user_input - 32) * 5 / 9;
                println!("{user_input} Fahrenheit is {result} in Celsius");
            }
        }
    }
}
