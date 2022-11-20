
// criando enum
enum Dias {
    _Segunda,
    Terca,
    _Quarta,
    // colocando dados em uma variante enum
    Quinta(String),
    _Sexta(String),
    _Sabado,
}

enum Jogador {
    Andando {x: i32, y:i32},
    _Pausado(bool),
    Correndo {x: i32, y: i32, velocidade: f32},
    _Cor(u8, u8, u8),
    _Parado,
}

impl Jogador {
    fn iniciando() -> Self {
        Jogador::Andando { x: 10, y: 0 }
    }

    fn correndo(&self, velocidade: f32) -> Self {
        Jogador::Correndo { x: 10, y: 0, velocidade}
    }
}

fn main() {

    let _terca = Dias::Terca;

    let _quinta = Dias::Quinta(String::from("Hoje é Quinta-feira"));    


    let jogador = Jogador::iniciando();
    
    match jogador {
        Jogador::Andando { x, y } => println!("Jogador está andando na posição x {x} e y {y}"),
        _ => ()
    }

    let jogador_correndo = jogador.correndo(1.2);

    match jogador_correndo {
        Jogador::Correndo { x, y, velocidade } => println!("Jogador correndo na posição x {x} e y {y} com a veloridade {velocidade}"),
        _ => ()
        
    }
}
