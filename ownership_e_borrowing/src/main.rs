fn main() {
    let text_one = String::from("Este é um simples texto");
    let text_two = String::from("Este é texto não é nada de mais");

    let larger_text = get_larger_text(&text_one, &text_two);
    println!("{larger_text:?}");

    // life time em struct
    let name = "Werick";
    let info = Info { name: &name };

    println!("info {info:?}")
}

fn get_larger_text<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

#[derive(Debug)]
struct Info<'a> {
    name: &'a str,
}
