fn main() {
    let mut s1 = String::from("hello");

    // let long = calcluer_taille(&s1);
    // println!("La taille de '{}' est {}.", s1, long);

    // changer(&mut s1);

    let mut s = String::from("hello");

    let r1 = &mut s;
    
    println!("r1: {}", r1);
}


fn calcluer_taille(s: &String) -> usize {
    s.len()
}

fn changer(texte: &mut String) {
    texte.push_str(", world");
}