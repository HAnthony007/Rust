use std::io::{self, Write};

fn main() {
    println!("Fahrenheit to Celsius converter!");

    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    loop {
        let mut choice = String::new();

        print!("Enter your choice [1/2] (1): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = if choice.trim().is_empty() {
            1
        } else {
            match choice.trim().parse() {
                Ok(num) => match num {
                    1 | 2 => num,
                    _ => {
                        println!("Invalid input. Please enter 1 or 2.");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Invalid input. Please enter 1 or 2.");
                    continue;
                }
            }
        };

        if choice == 1 {
            println!("# Fahrenheit to Celsius");

            loop {
                print!("Enter the temperature in Fahrenheit: ");
                io::stdout().flush().unwrap();
                let mut fahrenheit = String::new();
                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to read line");

                let fahrenheit: f64 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid fahrenheit. Please enter a number.");
                        continue;
                    }
                };

                let celsius = (fahrenheit - 32.0) / 1.8;

                println!("The temperature in Celsius is: {celsius}");

                break;
            }
        } else {
            println!("# Celsius to Fahrenheit");
            loop {
                println!("Enter the temperature in Celsius: ");
                io::stdout().flush().unwrap();
                let mut celsius = String::new();
                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read line");

                let celsius: f64 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid celsius, please enter number");
                        continue;
                    }
                };

                let fahrenheit = (celsius * 1.8) + 32.0;
                println!("The temperature in Fahrenheit is: {fahrenheit}");

                break;
            }
        }
        break;
    }
}
