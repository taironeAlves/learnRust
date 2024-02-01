/* fn main() {

    loop {
        println!("Novamente para sempre");
    }
}
 */

/* fn main() {

   let mut numero = 3;
   while numero != 0 {
       println!("{}", numero);
       numero -= 1;
   }

   println!("Fim do loop!")
} */

/* fn main () {

   let a = [10,20,30,40,50];
   let mut indice = 0;

   while indice < 5 {
       println!("O valor é : {}", a[indice]);
       indice += 1;
   }
} */

/*
fn main() {
    let a = [10, 20, 30, 40, 50];
    for elemento in a.iter() {
        println!("O novo valor é {}", elemento);
    }
}
 */

fn main() {
    for numero in (1..4).rev() {
        // .rev() representa reverso, ou seja, de traz pra frente

        println!("{}", numero);
    }
    println!("Fim do Loop!!!");
}
