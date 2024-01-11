fn main() {
    loop {
        println!("Looping eterno!");
        break; // A unica forma de para um looping
    }

    // Programando uma pausa no loop quando for true
    let mut counter = 1;
    let stop_loop = loop {
        let teste: &str = "10";
        counter *= 2;
        if counter > 100 {
            break teste; // Nesse caso é necessário retornar algum valor porque o loop está dentro de uma var
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);

    counter = 0;

    while counter < 5 {
        println!("Nosso lopping com While");
        counter += 1;
    }

    // uso do for..in é necessário colocar .iter() no valor da variável
    let big_birds = ["aba", "ababa", "bbaafa"];
    for bird in big_birds.iter() {
        println!("O pássaro {} é um grande pássaro", bird);
    }

    // A var number é declarada dentro do scope for e 0..5 pecorre a numeração até o valor 4 e não executa o 5
    for number in 0..5 {
        println!("{}", number * 2);
    }
}
