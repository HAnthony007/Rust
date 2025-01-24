// fn main() {
//     println!("\n\t# Welcome to the Rust Programming Language! #");
//     println!("\nWhat is Ownership?");
//     let s1 = String::from("Hello"); // s rentre dans la portee

//     prendre_possession(s1); // La valeur de s est deplacee dans la fonction ...
//     // ... et n'est plus accessible ici

//     let x = 5; // x rentre dans la portee

//     creer_copie(x); // x va etre deplacee dans la fonction,
//                             // mais i32 est copie, donc x reste valide ici
//                             //utiliser x ensuite.
// } // Ici, x sort de la portee, puis ensuite s. Mais puisque la valeur de s a 
// //ete deplacee, rien ne se passe rien de special.

// fn prendre_possession(texte: String) { // texte rentre dans la portee.
//     println!("{}", texte);
// } // Ici, texte sort de la portee et `drop` est appele. La memoire est liberee.

// fn creer_copie(entier: i32) { // entier retnre dans la portee.
//     println!("{}", entier);
// } // Ici, entier sort de la portee. Il ne se passe rien de special.



// fn main(){
//     let s1= donne_possession();

//     let s2 = String::from("Hello");

//     let s3 = prend_et_rend(s2);

//     println!("s1: {}, s3: {}", s1, s3);
// }

// fn donne_possession() -> String{
//     let texte = String::from("yours");
//     texte
// }

// fn prend_et_rend(texte:String) -> String {
//     texte
// }


fn main() {
    let s1 = String::from("Hello");

    let (s2, taille) = calculer_taille(s1);
    println!("La taille de '{}' est de {}.", s2, taille)
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len();
    (s, taille)
}
