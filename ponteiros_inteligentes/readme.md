

## Box<T>
Permite armazenar os valores direto na heap,  e algum tipo de casos que ele pode ser usado é 

* Quando se tem um tipo que não pode ter seu tamanho conhecido no momento da compilação
* Quando tem uma grande contidade de dados e não quer que ela seja copiada
* Quando deseja que um tipo implementa uma regra(`trait`) que não um tipo especifico

> Não sei porque usa o `dyn` antes da trait mais para frente buscar mais a fundo

Uma coisa que não será mudado é que quando a variavel sair do escopo não será mantida e ela será destruida 
