// Fomas de representar uma String().

fn main() {
    let data = "Initial contents";
    let s = data.to_string();
    let s = "initial contes".to_string();
    let s = String::from("Initial Contents");

    // Concatenando com a macro format! ou sinal +

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // Formas de acessar uma string.
    /*
       let b = String::from("hello");
       let c = b[0];
    */

    // Como o rust armazena string
    // let a = String::from("Hola").len(); // retorna 4

    let a = String::from("Здравствуйте").len(); // retorna 24
    println!("{}", a);

}
