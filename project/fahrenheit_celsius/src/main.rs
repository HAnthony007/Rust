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
        let to = convert_temperature(from, is_to_fahrenheit);
        break println!("The temperature: {from}°{from_desc} is {to}°{to_desc}\n");
    }
}

fn convert_temperature(value: f64, is_to_fahrenheit: bool) -> f64 {
    let result = if is_to_fahrenheit {
        (value * 1.8) + 32.0
    } else {
        (value - 32.0) / 1.8
    };
    (result * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit_simple() {
        assert_eq!(convert_temperature(0.0, true), 32.0);
        assert_eq!(convert_temperature(100.0, true), 212.0);
        assert_eq!(convert_temperature(-40.0, true), -40.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius_simple() {
        assert_eq!(convert_temperature(32.0, false), 0.0);
        assert_eq!(convert_temperature(212.0, false), 100.0);
        assert_eq!(convert_temperature(-40.0, false), -40.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit_decimal() {
        assert_eq!(convert_temperature(25.5, true), 77.9);
        assert_eq!(convert_temperature(37.7778, true), 100.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius_decimal() {
        assert_eq!(convert_temperature(77.9, false), 25.5);
        assert_eq!(convert_temperature(100.0, false), 37.7778);
    }

    #[test]
    fn test_rounding_precision() {
        assert_eq!(convert_temperature(36.685, true), 98.033);
        assert_eq!(convert_temperature(98.243, false), 36.8017);
    }
}
