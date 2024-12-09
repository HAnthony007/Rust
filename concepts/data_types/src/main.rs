use std::io;

fn main() {
    // // Scalar Types

    // // Integer Types
    // // signed: i8, i16, i32, i64, i128, isize
    // // -(2^(n-1)) to (2^(n-1) - 1)
    // //
    // // unsigned: u8, u16, u32, u64, u128, usize
    // // 0 to 2^n - 1
    // //
    // //  isize and usize depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    // let x: u8 = 255;
    // println!("La valeur de x est: {x}.");

    // // Floating-Point Types
    // // f32, f64
    // // f32: single-precision, f64: double-precision
    // // Represented according to IEEE 754 standard
    // // f64 is the default type for floating-point numbers because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    // // f64 64 bits
    // // f32 32 bits
    // let single: f32 = 3.0 / 9.0;
    // let double: f64 = 3.0 / 9.0;
    // println!("La valeur de simple precision est: {single}");
    // println!("La valeur de double precision est: {double}");

    // // Numeric Operations
    // // addition
    // let sum = 5 + 10;
    // // subtraction
    // let difference = 95.5 - 4.3;
    // // multiplication
    // let product = 4 * 30;
    // // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Results in 0
    // // modulo
    // let remainder = 43 % 5;

    // println!("La somme est: {sum}");
    // println!("La difference est: {difference}");
    // println!("Le produit est: {product}");
    // println!("Le quotient est: {quotient}");
    // println!("Le quotient entier est: {floored}");
    // println!("Le reste est: {remainder}");

    // // Booleans
    // let t = true;
    // let f: bool = false; // with explicit type annotation
    // println!("La valeur de t est: {t}");
    // println!("La valeur de f est: {f}");
    
    // // character type
    // let c = 'z';
    // let z = 'Z';
    // let heart_eyed_cat = '😻';
    // println!("La valeur de c est: {c}");
    // println!("La valeur de z est: {z}");
    // println!("La valeur de heart_eyed_cat est: {heart_eyed_cat}");


    // // Compound Types

    // // Tuples
    // Generally, tuples group together values of different types.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // Tuples are useful for grouping together a number of values under a single variable name.
    // Tuples can be used to return multiple values from a function.
    // Tuples can be destructured to create bindings to their individual values.
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("La valeur de x est: {} et aussi {}", x, tup.0);
    // println!("La valeur de y est: {} et aussi {}", y, tup.1);
    // println!("La valeur de z est: {} et aussi {}", z, tup.2);

    // // () is a tuple with 0 elements: Type UNIT
    // // Values of type unit is also called UNIT VALUE 
    // let tuple = ();
    // println!("La valeur de tuple est: {:?}", tuple);

    // // Arrays
    // Each element of an array must have the same type.
    // Arrays are fixed in size.
    // The type of an array is [T; N] where T is the type of the elements and N is the number of elements.
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("La valeur de first est: {}", first);
    println!("La valeur de second est: {}", second);

    // Project
    let a = [1, 2, 3, 4, 5];
    println!("Veuillez entrer un indice de tableau: ");
    let mut indice = String::new();
    io::stdin()
        .read_line(&mut indice)
        .expect("Echec de la lecture de l'entrée utilisateur");
    let indice: usize = indice
        .trim()
        .parse()
        .expect("L'indice doit être un entier");
    let element = a[indice];
    println!(
        "La valeur de l'element d'indice {} est : {}",
        indice, element
    )
}
