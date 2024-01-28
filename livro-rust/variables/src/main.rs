fn main() {
    let mut x: u32 = 5;
    println!("O valor de x é {}", x);

    x = 6;
    println!("O valor novo de x é {}", x);

    const PONTOS_MAXIMOS: u32 = 100_000;

    // Prática de sobrebar variável
    let a = 1;
    let a = a + 2;
    let a = a + 3;

    println!("O valor de A é {}", a);

    // Tipo tuplas
    let tup = (1, "a", 2, "b");

    // let (a, b, c,d) = tup;

    println!("Tuplas {}", tup.0);

    let abc = tup.3;
    println!("Tuplas {}", tup.3);

    // Matriz
    let matriz = [1, 2, 3, 4, 5];
    
    // Acessando uma matriz
    let a = matriz[0];

    println!("A vale {}", a);

    // Nome de funções por convenção são snake case tudo minusculo ex. teste_funcao()
}
