use std::collections::HashMap;

fn main() {
    let mut hash = HashMap::new();

    hash.insert(String::from("Idade"), 22.0);
    hash.insert(String::from("Altura"), 1.89);

    println!("{:?}", hash);

    println!(
        "Valor da Chave 'Idade' é {}",
        hash.get(&String::from("Idade")).unwrap()
    );

    // Sobreescrevendo valor
    hash.insert(String::from("Idade"), 18.0);
}
