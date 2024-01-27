extern crate rand;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Advinhe o número");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        // A função trim() remove os espaços em branco do inicio e fim de uma string
        let test: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match test.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou");
                break;
            }
        }
    }
    // Parei na pagina 42/359
}
