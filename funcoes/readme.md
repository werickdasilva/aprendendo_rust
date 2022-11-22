# Funções

a forma de criar função é igual a python ou javascript só muda a palavra chave que será usada que será `fn`
```rust
fn funcao() {
    // ....
}
```

## Argumento em função 
```rust
fn func_args(num: i32, num2: i32) {
    // ....
}
```

## Retorno de função
Em rust a se a ultima linha for o retorno dá função não precisa usar o `return`  
```rust
fn retorno_de_funcao() -> i32 {
    100
}
```
Agora função sem retorno
```rust
fn sem_retorno() {
    // ...
}
```

## Função de ordem superior
São função passada como argumento em função
```rust
fn funcao_dentro_de_funcao<F>(fun: F) where F: Fn()-> String {
    // ...
}
```
A letra `F` seria um tipo genérico e a cláusala `where`é usado para fazer ligação entres os tipos mais não é obrigado a usar ele que quiser pode usar sem 
```rust
fn funcao_dentro_de_funcao<F:  Fn()-> String >(fun: F){
    // ...
}
``` 

## Função Anônimas ou  Closures
são função que não tem definição estatica e para criar em rust usa:

```rust 
let func = || {
    //..
}

let func_args = |n: i32| {
    // ..
}
```