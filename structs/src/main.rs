// criando struct
struct CriandoScruct {
    nome: String,
    idade: i32,
}

impl CriandoScruct {
    // metodos estaticos
    fn new() -> Self {
        Self {
            idade: 22,
            nome: String::from("Werick"),
        }
    }

    fn show_name(&self) {
        println!("Seu nome é {} e tem {} anos ", self.nome, self.idade)
    }
}

fn main() {
    // formas de iniciar a struct
    let primeira_forma = CriandoScruct {
        idade: 33,
        nome: "Ana".to_string(),
    };

    let segunda_forma = CriandoScruct::new();

    primeira_forma.show_name();
    segunda_forma.show_name();
}
