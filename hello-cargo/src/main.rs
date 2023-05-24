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
    print!("Is 1 > 4? {}", is_bigger);
}
