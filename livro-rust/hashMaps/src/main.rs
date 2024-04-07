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

/* use std::collections::HashMap;

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
 */

/*
// Substituindo valores do hashmap
 use std::collections::HashMap;

 fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
 }
 */

// Adicionando valor a um hashmap caso esteja vazio

/*
use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
 */

// Incrementando valor do hashMap

use std::collections::HashMap;
fn main() {
    let text = "Hello world wonderfull world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // count = 0
        *count += 1; // count = 0 + 1 -> quando chegar a palavra world count = 1 + 1
    }

    println!("{:?}", map);
}
// Parei na pagina 177