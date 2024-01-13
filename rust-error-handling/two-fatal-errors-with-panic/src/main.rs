use core::panic;

fn main() {

    /*
    O panic encerra a execução do programa imediatamente.
    Normalmente usado quando a falha é inreversivel.
    panic!("Farewell!");
    
    Um vetor é um array dinamico, diferente de um array estático que tem tamanho fixo
    o vetor pode ser adicionado ou removido elementos dentro dele.

    Os elementos dentro de um vetor tem que ser da mesma tipagem.

     */
    let v = vec![1,2,3,4];
    println!("{}", v[4]); // aqui dá panic out of bounds

}
