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
    println!("O valor do tipo &str é {tipo_str}");
    


    println!();
    println!("Criando Tuplas");
    
    let tupla_cor_rgb: (u8, u8, u8) = (122, 99, 255);
    println!("Valor da cor em rgb é {:?}", tupla_cor_rgb);

    println!();
    println!("Criando Array");

    let array_nome_frutas = ["Uva", "Banana", "Goiaba"];
    let array_iniciado_com_valores_padrão = [0; 5];

    println!("A fruta do array na posição 1 é {}", array_nome_frutas[0]);
    println!("Array com todos os item inicializado com zero {array_iniciado_com_valores_padrão:?}");

    println!();
    println!("Variavel mutavel");

    let mut nome = "Werick";
    let primeiro_nome = nome;
    nome = "Werick da Silva Santana";

    println!("Nome de {primeiro_nome} foi atualizado para {nome}.");

    println!();
    const CONSTANTE: bool = false;
    println!("O valor da constante é '{CONSTANTE}'")
}
