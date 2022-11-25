
fn main() {
    let slice: [i32; 4] = [10, 242, 138, 120];

    let iterador = slice.iter();

    for num in iterador {
        print!(" {num} ")
    }

    let mut nomes = vec!["Werick", "Cleria", "Hellen", "Edmilson"];

    let iterador_mut = nomes.iter_mut();
    iterador_mut
        .map(|i| i.to_uppercase())
        .for_each(|i| println!("{i}"))
}
