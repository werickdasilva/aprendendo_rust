fn main() {
    let criando_vetor = vec![10,10, 2,20,303,22];
    let criando_vetor_pela_struct_vec: Vec<String> = Vec::new();

    let convertendo_range_para_vetor :Vec<i32> = (0..10).collect();
    let convertendo_range_para_vetor_sem_passar_parametro_na_variavel = (0..5).collect::<Vec<u8>>();


    println!("Vetores Criados");
    println!("Vetor com a macro vec {:?}", criando_vetor);
    println!("Vetor com a struct Vec {:?}", criando_vetor_pela_struct_vec);
    println!("Convertendo range para vec usando collect {:?}", convertendo_range_para_vetor);
    println!("Convertebdi range para vetor sem passar parametro na variavel {:?}",convertendo_range_para_vetor_sem_passar_parametro_na_variavel );
    println!();

    // with_capacity define o limite de item que o vec podera ter 
    let mut vetor_numero = Vec::with_capacity(5);
    let mut vetor_complementar = vec![22, 33, 44]; 
    // Adicionando item no vec 
    vetor_numero.push(20);
    println!("Adicionado o valor 20 no vec com o metodo push valor atual do vetor é {:?}", vetor_numero);
    
    println!("Tamanho do vetor é {}", vetor_numero.len());
    
    // apprend adicona outro vec ao vec autal
    vetor_numero.append(&mut vetor_complementar);
    println!("Agora Adicionado mais valor ao vetor e ele ficou assim {:?} ", vetor_numero);

    println!("Desminuindo vetor para dois item {:?}", vetor_numero.truncate(2))

    
}
