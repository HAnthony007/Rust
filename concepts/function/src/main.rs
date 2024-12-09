fn main() {
    // Statement do not return values.
    // Expression return values.

    // Statements do include semicolons
    // Expressions do not include semicolons
    let x = {
        let y = 6;
        y
    };
    println!("La valeur de x est: {}", x);

    affiche_mesure_avec_unite(5, 'h');

    // functions with return values
    let chiffre_cinq = cinq();
    println!("Le chiffre cinq est: {}", chiffre_cinq);

    let incremente = plus_un(cinq());
    println!("Le chiffre incremente est: {}", incremente);
}

fn affiche_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est: {}{}", valeur, unite);
}

fn cinq() -> i32 {
    5
}

fn plus_un(nombre: i32) -> i32 {
    nombre + 1
}