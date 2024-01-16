use std::io::Empty;

fn main() {
    /*
    Aqui praticaremos o uso do enum option<t> que possui dois tipos de retorno
        NONE-> caso não encontre nenhum valor
        SOME(Value) -> caso encontre o valor procurado.
    */

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    let first = fruits.get(0);
    println!("{:?}", first);

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    /*
    Pegando os valores dentro de Some()

     */

    for &index in [0, 2, 99, 1, 10].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("Esta fruta é {}", fruit_name),
            None => println!("Nenhuma fruta"),
        }
    }

    /*
    Executando uma ação específica quando a palavra é encontrada.
    Some(fruit_name) é obrigatorio.
      */

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Este é um coconut"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("nada"),
        }
    }

    /*
    Testando valores com o if let
     */

    let a_number: Option<u8> = Some(7);

    match a_number {
        Some(7) => println!("Esse é o numero 7"),
        _ => {}
    }

    /* Obtendo o mesmo resultado do match com o if let  */

    if let Some(7) = a_number {
        println!("Numero 7 encontrado");
    }

    /* Usando o unwrap e expect, esses podem gerar panic caso receba um None */

    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    /* Aqui de proposito vai retornar um None para gerar panic. 
    
    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy");

    */

    /* O expect faz a mesma coisa, mas gera mensagem personalizada no panic
    */

    let a = Some("value");
    assert_eq!(a.expect("frutas são boas"), "value");

    let b: Option<&str> = None;
    b.expect("Deu ruim");

}
