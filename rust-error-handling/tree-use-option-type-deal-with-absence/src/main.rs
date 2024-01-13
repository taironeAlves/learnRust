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
}
