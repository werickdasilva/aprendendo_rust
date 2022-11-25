use std::fs::File;

fn main() {
    let is_correndo = true;
    let is_andando= true;

    if is_correndo {
        println!("Está correndo")
    } else if is_andando {
        println!("Não está correndo só está andando")
    } else {
        println!("Está Parada")
    }


    for n in 0..5 {
        println!("for >>> {n} ")
    }

    let mut x = 0;

    while x < 5 {
        println!("while >>> {x} ");
        x+= 1;
    }


    let numero = Some(19);
    // pattern match com if
    if let Some(num) = numero {
        println!("Pattern match com if valor do numeor é {num}")
    } 

    let mut numeros = vec![1,2,3,4,5,6,7,8,9,];

    while let Some(numero) = numeros.pop()  {
        println!("Numeros é {numero}")
    }

    println!("=========== Usando o match expressão ===============");

    let value_number = 5;
    match value_number {
        1 => println!("Numero 01"),
        2..=10 => println!("Numero entre 2 a 10"),
        12 | 14 | 16 => println!("Numero foi 12, 16, ou 18"),
        _ => println!("Numero maior que 16")
    }

    //  @ seria como uma renomeação de valores 
    match value_number {
        n @ 5 => println!("numero escolhido foi {n}"),
        n @ 10 => println!("numero escolhido foi {n}"),
        _ => println!("")
    }


    println!("match com Option .........................");
    let some = Some(value_number);
    match some {
        Some(n) => println!("Tem valor e seu valor é {n}"),
        None => println!("Não tem valor, então seu valor está sendo None")
    }

    println!("match com Result...............");
    let result = File::open("./Cargo.toml");
    match result {
        Ok(_) => println!("Arquivo existe...."),
        Err(_) => println!("Arquivo não existe............")
    }

    println!("match e usando if juntos");
    match result {
        Ok(file) => {
            println!("Tem arquivo");
            match file.metadata() {
                Ok(_) => println!("Sem Informação sobre os dados do arquivo"),
                
                Ok(data) if data.is_file()  => {
                    println!("è um arquivo ")
                },
                 Err(_) => println!("Deu Erro buscar informação sobre o arquivo")

            }
        },
        Err(_) => println!("Error")
    }
}
