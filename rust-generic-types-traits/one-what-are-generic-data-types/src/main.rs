/*
struct Point<T> {
    // <T> representa o recebimento de um tipo ainda indefinido. E todas as variantes tem que ser do mesmo tipo
    // que foi recebido.
    x: T,
    y: T,
}
 */

// Aqui é a forma de usar um struct com dois ou mais tipos diferentes indefinidos.
struct Point<T, U, V> {
    x: T,
    y: U,
    z: V,
}

fn main() {

    let _boolean = Point {
        x: true,
        y: 10,
        z: 11,
    };
    let _integer = Point { x: 1, y: 9,  z: 11 };
    let _float = Point { x: 1.7, y: 4.3,  z: 11 };
    let _string_slice = Point {
        x: "higth",
        y: "low",
        z: 11,
    };
}
