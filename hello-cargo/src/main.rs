// Objetos ou Structs
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

//Struct somente com tipos de dados
struct Grades(char, char, char, char, f32);

//Struct tipo unidade
struct Unit;

fn main() {
    // Call prinln! whit three arguments: a string, a value, a value;
    println!(
        "The first letter of the English alphabet is {} and the last letter is {}.",
        "A", "Z"
    );

    let a_number;

    a_number = 10;

    // tornando a variável mutavel
    let mut b_number = 10;
    println!("The number is {}.", b_number);

    // Sombreamento de variável
    let shdow_num = 5;
    let shdow_num = shdow_num * 2;

    println!("Th number is {}", shdow_num);

    //Tipando variáveis como number de 32 bits
    let number: u32 = 14;

    println!("Tipo de numero de 32bts recebe {} ", number);

    //Cálculos com tipagem
    println!(
        "1 + 2 = {} and 8 -5 = {} 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Teste tipagem bolean
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    //tuplas (array)
    let tuple_e = ('E', 5i32, true);

    //Exemplo de uso da tupla
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    // Exemplo de uso do Structs
    let user_1 = Student {
        name: String::from("Constance Sharma"),
        remote: true,
        level: 2,
    };

    let user_2 = Student {
        name: String::from("Tairone Alves"),
        level: 5,
        remote: false,
    };
    let mark_1 = Grades('A', 'A', 'A', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );

    // Exemplo de enumeração
}
