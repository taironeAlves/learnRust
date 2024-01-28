// fn main() {
//     println!("Hello, world!");

//     outra_funcao(5);
// }

// fn outra_funcao(x:u8) {
//     println!("Valor de x é {}", x);
// }

fn main(){
    let x = soma_um(5);

    println!("O valor de x é : {}", x);
}

fn soma_um(x: i32) -> i32 {
    x + 1
    // parei na pagina 63
}