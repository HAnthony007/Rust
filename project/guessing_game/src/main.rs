use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);
    // println!("Le nombre secret est: {}", nombre_secret);

    loop {
        println!("Veuillez entrer un nombre: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Echec de la lecture de la ligne");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("Vous avez devine le nombre {}", guess)

        match guess.cmp(&nombre_secret) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Vous avez gagne !");
                break;
            }
        }
    }
}
