// Definindo HashMaps

/* use std::collections::HashMap;

fn main() {
let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


}

 */

// Acessando valores do hashMap
/* 
use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get("Blue");

    println!("{:?}", scores["Blue"]);
}
 */

use std::collections::HashMap;

// Interando com um um HashMap. HashMap a key pode ser do tipo inteiro ou String e value pode ser de qualquer tipo
// porem todos os keys e values tem que ser do mesmo tipo que foi definido primeiro.

fn main() {
    let mut times = HashMap::new();

    times.insert(String::from("tairone"),10);
    times.insert(String::from("noeli"), 12);

    // println!("{:?}", times.get("tairones"));

    for(key, value) in &times {
        println!("{}: {}", key, value);
    }
}

// Parei na pagina 175