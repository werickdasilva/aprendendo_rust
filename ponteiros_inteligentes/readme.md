

## Box<T>
Permite armazenar os valores direto na heap,  e algum tipo de casos que ele pode ser usado é 

* Quando se tem um tipo que não pode ter seu tamanho conhecido no momento da compilação
* Quando tem uma grande contidade de dados e não quer que ela seja copiada
* Quando deseja que um tipo implementa uma regra(`trait`) que não um tipo especifico

> `dyn` são usado para destacar a chamada de métodos na trait enviado dinamicamente mais informação [click aqui](https://doc.rust-lang.org/std/keyword.dyn.html) 

Uma coisa que não será mudado é que quando a variavel sair do escopo não será mantida e ela será destruida 


# Rc<T>
Usado quando não se sabe quantos o tamanho da referencia e a palavra `Rc` é uma abreviação para contavem de referência

Seu uso será para quando quiser ter adicionar varios dados na heap em diferentes parte do programa

> Umas das diferencia é que em tempo de compilação não quem como saber quem será usado primeiro e quem sair primeiro ou por ultimo