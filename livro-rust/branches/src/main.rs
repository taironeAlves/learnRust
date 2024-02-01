// fn main() {
//     let numero: u8 = 12;

//     if numero < 5 {
//         println!("Condição verdadeira");
//     } else {
//         println!("Condição era falsa");
//     }
// }

// fn main() {
//     let condicao = true;
//     let numero = if condicao { // aqui codição é true
//         5
//     }else {
//         6
//     };

//     println!("O valor do númeor é: {}", numero);

// }

fn main() {
    let condicao = true;
    let numero = if condicao {
        // aqui codição é true
        "5" // O tipo de retorno tem que ser o mesmo tanto para if como pra else porque estão dentro de uma variável
    } else {
        "seis"
    };

    println!("O valor do númeor é: {}", numero);
}


