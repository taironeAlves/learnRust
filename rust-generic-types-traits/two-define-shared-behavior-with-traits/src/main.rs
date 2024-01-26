// traits são metodos que podem ser implementados em Structs, enums...
// um trait pode conter varios metodos
// Varios structs podem implementar uma mesma traits.
// Se uma trait tiver 2 ou mais metodos, todos os metodos tem que ser implementados.


fn main() {}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

