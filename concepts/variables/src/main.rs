// // Constants
// const MAX_POINTS: u32 = 100_000;

fn main() {
    // // Variables immutable
    // let mut x = 5;
    // println!("La valeur de x est {x}");
    // x = 6;
    // println!("La valeur de x est {x}");
    // // Cannot change type of variable
    // let mut first_spaces = "   ";
    // first_spaces = first_spaces.len();
    // println!("Number of space: {first_spaces}");

    // // Constants
    // println!("La valeur du constante Max points: {MAX_POINTS}")

    // // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("La valeur de x dans le block interne est: {x}");
    }
    println!("La valeur de x est: {x}");
    // Can change the type of the variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of space: {spaces}")
}
