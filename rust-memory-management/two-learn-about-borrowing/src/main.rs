fn main() {
    // As referências nos permitem "emprestar" valores sem se apropriar deles.
    // & é o simbolo para referencia/emprestimo do valor da variável.
    /*
    let greeting = String::from("Helo, my brother!");

    let greeting_refence = &greeting;

    println!("greeting: {}", greeting);
     */

    // Usando emprestimo dentro da função
    /*
    fn print_greeting(message: &String) {
        test(&message);

    }

    fn test(message: &String){
        println!("{}", message);
    }

    let greeting = String::from("Helo, my sister!");
    print_greeting(&greeting);
    print_greeting(&greeting);

    */

    
    // Valores emprestados não podem ser modificados.
    /* 
     fn change(message: &String){
        message.push_str("string");
    }

    let greeting = String::from("Hello, is me");
    change(&greeting);
    
     */

    // Para modificar o valor da variavel que foi emprestada é necessário tipar como &mut
    /* 
    
    let mut greeting = String::from("Hello, boy");
 
    fn change(text: &mut String){
     text.push_str("!");
     println!("{}", text);
    }
     */ 
   

    //  Rust não permite compartilhar o valor de uma mesma variavel dentro do mesmo escopo.

    /* 
    
    let mut value = String::from("hello, girl");
    let ref1 = &mut value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
    
     */

}
