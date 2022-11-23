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

}
