fn main() {
    let numero = 100;
    let numero_por_tipo_no_valor = 155_u8;
    let numero_por_tipo_definindo: i64 = 1000;

    println!("Mostrando valores do tipos inteiros.....");
    println!("Sem Difinir o tipo {numero}");
    println!("Definindo tipo no valor da variavel {numero_por_tipo_no_valor}");
    println!("Definindo tipo da variavel na criação {numero_por_tipo_definindo}");

    println!();
    println!("Outros tipos de Variavel ");

    let tipo_floot = 1.0_f32;
    let tipo_char = 'A';
    let tipo_boolean = true;
    let tipo_str: &str = "Este é meu valor...";

    println!("Tipo Flutuante pode ser f32 e f64 {tipo_floot}");
    println!("Tipo char pode ser char {tipo_char}");
    println!("Tipo boolean pode ter o valor true ou false e seu valor atual é {tipo_boolean}");
    println!("O valor do tipo &str é {tipo_str}")
}
