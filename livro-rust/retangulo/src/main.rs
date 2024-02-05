// Calculando a area do retangulo com variáveis
/* fn main() {

    let length1 = 50;
    let width1 = 30;

    println!(
        "The are fo the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
 */

//  Calculando a area do retangulo com tuplas
/* fn main() {

    let rect1 = (30, 30);

    println!("The are fo the rectangle is {} square pixels.", area(rect1));
}

fn area(dimession: (u32, u32)) -> u32 {
    dimession.0 * dimession.1
}
 */

 struct Rectangle {
    length: u32,
    width: u32,
 }

//  Calculando a area do retangulo com structs
 fn main() {
    let rect1 = Rectangle {length:50, width: 30};

    println!("The are fo the rectangle is {} square pixels.", area(&rect1));
 }

 fn area(rectangle: &Rectangle) -> u32 {

    rectangle.length * rectangle.width

 }
// parei na linha 109