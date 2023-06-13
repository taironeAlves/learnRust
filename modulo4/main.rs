fn main() {
    //tuple são de tamanho e tipos variados
    //Array são de tamanho fixo e mesmo tipo

    //Formas de declarar arrays

    // Forma 1 -> Colocando apenas elementos do mesmo tipo
    let days = ["Sunday", "Monday", "Tuesday"];

    //Forma 2 -> Definindo o tamanho.
    let bytes = [0; 5];

    // Recebe o primeiro dia da semana
    let first = days[0];
    let second = days[1];

    // Diferente de arrays, vetores tem tamanhos dinamico.
    // Declarando um vetor
    let three_nums = vec![15, 3, 46];
    println!("Inicalizando um vetor {:?}", three_nums);

    let zeroes = vec![0;5];
    println!("Zeroes: {:?}", zeroes);

    // Adicionando valores aos vetores
    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    //Removendo o último valor do Vetor
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    //Acessar elemento inexistente
    println!("Elemento 10 {}", fruit[10]);
}
