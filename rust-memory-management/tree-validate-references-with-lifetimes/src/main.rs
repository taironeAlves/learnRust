fn main() {
    // O Rust tabalha com tempo de vida, ou seja, não permite que uma variável faça
    // referencia a um valor que não existe mais.

    /*
    let x;
    {
        let y = 42;
        x = &y;
    }
    println!("x: {}", x);

     */

    // Anotando o tempo de vida
    /*
    fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let magic1 = String::from("abracadabra");
    let magic2 = String::from("Shazam");

    let result = longest_word(&magic1, &magic2);
     */

     /* 
     let magic1 = String::from("Abracadabra"); // Esse tem vida longa porque está no escopo superior
     let result;
     {
         let magic2 = String::from("Shazam"); // vida curta porque está no escopo inferior
         result = longest_word(&magic1, &magic2);
     }
 
     println!("The longest magic word is {}", result);
 
     fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
         if x.len() > y.len() {
             x
         }else {
             y
         }
     }
     
      */

    // Definindo o tempo de vida para funções

    /* 
    
     */

    #[derive(Debug)]
    struct Highlight<'document>(&'document str);

     let text = String::from("the quick brown fox jumps over the lazy dog");
     let fox = Highlight(&text[4..19]);
     let dog = Highlight(&text[35..43]);
     println!("{:?}", fox);
     println!("{:?}", dog);




}
