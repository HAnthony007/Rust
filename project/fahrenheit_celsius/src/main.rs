use std::io::{self, Write};

fn main() {
    println!("\n\t## Fahrenheit & Celsius Converter: ##");

    loop {
        println!("\n1. Fahrenheit to Celsius\n2. Celsius to Fahrenheit\n0. Exit");
        let mut choice = String::new();

        input("Enter your choice [1/2/0] (1): ", &mut choice);

        let choice = if choice.trim().is_empty() {
            1
        } else {
            match choice.trim().parse() {
                Ok(num) => match num {
                    0 | 1 | 2 => num,
                    _ => {
                        println!("Invalid input. Please enter 1 or 2 or 0.");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Invalid input. Please enter 1 or 2 or 0.");
                    continue;
                }
            }
        };

        if choice == 1 {
            converter("Fahrenheit", "Celsius", false);
        } else if choice == 2 {
            converter("Celsius", "Fahrenheit", true);
        } else {
            break println!("\nGoodbye!");
        }
    }
}

fn input(message: &str, variable: &mut String) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(variable)
        .expect("Failed to read line");
    variable.to_string()
}

fn converter(from_desc: &str, to_desc: &str, is_to_fahrenheit: bool) {
    println!("\n#Converting {from_desc} to {to_desc}");
    loop {
        let mut from = String::new();
        input(
            &format!("Enter the temperature in °{from_desc}: "),
            &mut from,
        );
        let from: f64 = match from.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid {from_desc}, please enter number");
                continue;
            }
        };
        let to = if is_to_fahrenheit {
            (from * 1.8) + 32.0
        } else {
            (from - 32.0) / 1.8
        };
        break println!("The temperature: {from}°{from_desc} is {to:.4}°{to_desc}\n");
    }
}
