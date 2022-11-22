fn main() {
    mostra_resultado(soma_numero(846, 372));

    let closure = |numero: i32| {
        (numero + 50) * 55
    };
    mostra_resultado_da_conta(closure);
}

fn soma_numero(numero: i32, numero_2: i32) -> i32 {
    numero + numero_2
}

fn mostra_resultado(resultado: i32) {
    println!("O Resultado foi {resultado}")
}


fn mostra_resultado_da_conta<F>(fun: F) where F: Fn(i32) -> i32 {
    let resultado = fun(2);
    println!("O Rssultado da conta foi {resultado}")
}
