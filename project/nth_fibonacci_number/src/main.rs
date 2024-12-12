fn main() {
    println!("Hello, fibonacci!");
    println!("fibonacci de 5 est {}", fibonacci(3));
    println!("fibonacci de 6 est {}", fibonacci(6));
    println!("fibonacci de 7 est {}", fibonacci(7));
    println!("fibonacci de 8 est {}", fibonacci(8));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}