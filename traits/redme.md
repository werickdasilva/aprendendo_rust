# Trait
Seria o e quivalente a um `interface` em POO


`trait`não é particular de struct pois tambem pode ser implementada por enum e por anotação `#[derive(...)]`

## Criando trait
```rust
trait Info {
    fn show(&self);

    fn is_stat(&self, stata: bool) {

    }
}
```
> função que não tem escopo é obrigado a ser implementa 

## Implementando trait
```rust
struct People;

impl Info for People {
    fn show(&self) {
        println!("Metodo show Usado a trait info na struct People")
    }
}
```

## trait bouns
usado e tipo generico que define que for o tipo tem que implementar a `trait`
```rust
fn show_info<I: Info>(info: I) {
    info.show()
}
```