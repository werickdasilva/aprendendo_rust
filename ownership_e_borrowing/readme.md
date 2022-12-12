
## lifetime
são os tempo de vida que cada valor(variavel) e os lifetime são feito em tempo de compilação

> `lifetime` esta internamente ligada a referência 
> `lifetime` foi criado para não ter ponteiro empidurado
Exemplo de erro
---
```rust
let numero; // 'a
{
    let novo_numero = 5;
    numero = &novo_numero;
} // novo_numero foi destruido

println!("{}", numero) // erro novo_numero não vive suficiente
```
Exemplo funcionando
--- 
```rust
fn main() {
    let text_one = String::from("Este é um simples texto");
    let text_two = String::from("Este é texto não é nada de mais");

    let larger_text = get_larger_text(&text_one, &text_two);
    println!("{larger_text:?}")
}

fn get_larger_text<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Quando tem casos que não precisa passar o lifetime pois o compilador consegue definir [saiba mais](https://doc.rust-lang.org/book/ch10-00-generics.html)

## lifetime em struct

## Tipo Copy
são tipos copiado que implementa a trait `Copy`