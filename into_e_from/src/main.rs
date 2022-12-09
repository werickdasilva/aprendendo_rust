fn main() {
    let numero = 10.0;

    let numero: Numero = numero.into();

    println!("Into e From  valor {:?}", numero)
}
#[derive(Debug)]
struct Numero {
    n: i32,
}

impl From<f64> for Numero {
    fn from(num: f64) -> Self {
        Numero { n: num as i32 }
    }
}
