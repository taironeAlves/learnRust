// Usando vetores. Vetores podem armazenar muitos valores do mesmo tipo.
/* fn main() {
    let v: Vec<i32> = Vec::new();
}
 */

// Usando uma macro para referenciar uma Vec.
fn main() {
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);

    // Lendo valores dos vetores
    let _third = &v[4];
    let _third = v.get(2);
    

   
}

// Parei pagina  164