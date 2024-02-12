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

/*
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
 */

/* // Imprimir com structs para debugar

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {length:50, width:30};

    println!("rect1 is {:#?}", rect1);
}
 */

// Utilizando structs com metodos traits

/*
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
}
 */

//  Colocando um retangulo dentro de outro
/*
#[derive(Debug)]
struct Rectagle {
    length: u32,
    width: u32,
}

trait Area {
    fn area(&self) -> u32;
}

trait Hold {
    fn can_hold(&self, other: &Rectagle) -> bool;
}

impl Area for Rectagle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

impl Hold for Rectagle {
    fn can_hold(&self, other: &Rectagle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectagle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectagle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectagle {
        length: 45,
        width: 60,
    };

    println!("Can rect 1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect 1 hold rect3? {}", rect1.can_hold(&rect3));
}

 */

// Funções associadas, usando com o impl. String::from é um exemplo de funções associadas
// Essas funções não precisam do parametro &self
// O &self serve para usar as propriedades da Struct, enum...

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

struct Quadrado {}

impl Quadrado {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            length: size,
            width: size,
        };
    }
}

fn main() {
    let sq = Quadrado::square(3);
    println!("{:?}", sq.length);
}
