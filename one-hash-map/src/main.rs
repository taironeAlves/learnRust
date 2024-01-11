// hashMap são utilizados para guardar chave e valor, semelhante aos obj em Javascript
// Tanto a key como a value tem que ser tipadas <String, String>
use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(
        String::from("Naomi Magalhaes Barreto"),
        String::from("A sintax da priguiça"),
    );
    reviews.insert(
        String::from("Noeli Magalhaes Barreto"),
        String::from("Eu não sei educar os meus filhos"),
    );
    reviews.insert(
        String::from("Artur Magalhaes Barreto"),
        String::from("Você atrapalhou minha historinha"),
    );

    println!(
        "\nReview for teste {:?}",
        reviews.get("Artur Magalhaes Barreto")
    );
}
