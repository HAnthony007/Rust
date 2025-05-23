use std::io::{self, Write};

fn main() {
    println!("\n\t## WELCOME ! ##");
    loop {
        println!("1. Calcule Fibonacci and Fibonacci sequence");
        println!("2. Exit");
        let mut choice = String::new();
        input("Enter your choice [1/2]", &mut choice);

        match choice.trim().parse() {
            Ok(1) => {
                println!("\n1. Calcule Fibonacci and Fibonacci sequence");
                loop {
                    let mut number = String::new();
                    input("Enter a number", &mut number);

                    let number: u32 = match number.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a valid number positive\n");
                            continue;
                        }
                    };

                    let sequence = fibonacci_sequence(number);

                    println!("\nFibonacci of {} is {}", number, fibonacci(number));
                    println!("Fibonacci sequence: {:?}\n", sequence);
                    break;
                }
            }
            Ok(2) => {
                println!("\n\t## GOOD BYE! ##");
                break;
            }
            Ok(_) => {
                println!("Please enter a valid number [1/2]\n");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number [1/2]\n");
                continue;
            }
        }
    }
}

fn input(message: &str, variable: &mut String) -> String {
    print!("{}: ", message);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(variable)
        .expect("Failed to read line");
    variable.trim().to_string()
}

fn fibonacci_sequence(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();
    for i in 0..=n {
        sequence.push(fibonacci(i));
    }
    sequence
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(fibonacci_sequence(0), vec![0]);
        assert_eq!(fibonacci_sequence(1), vec![0, 1]);
        assert_eq!(fibonacci_sequence(2), vec![0, 1, 1]);
        assert_eq!(fibonacci_sequence(3), vec![0, 1, 1, 2]);
        assert_eq!(fibonacci_sequence(4), vec![0, 1, 1, 2, 3]);
        assert_eq!(fibonacci_sequence(5), vec![0, 1, 1, 2, 3, 5]);
    }
}