/*
struct Point<T> {
    // <T> representa o recebimento de um tipo ainda indefinido. E todas as variantes tem que ser do mesmo tipo
    // que foi recebido.
    x: T,
    y: T,
}
 */

// Aqui é a forma de usar um struct com dois tipos diferentes indefinidos.

struct Point<T, U, V> {
    x: T,
    y: U,
    z: V,
}

fn main() {
    
    let boolean = Point {
        x: true,
        y: 10,
        z: 11,
    };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point {
        x: "higth",
        y: "low",
    };
}
