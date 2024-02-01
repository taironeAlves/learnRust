fn main() {
    let vezes = 10;
    let resp = fibonatti(vezes);

    println!("{:?}", resp);
}

fn fibonatti(n: usize) -> Vec<u32> {
    let mut total: Vec<u32> = Vec::new();

    if n >= 1 {
        total.push(0);
    };

    if n >= 2 {
        total.push(1);
    };

    while total.len() < n {
        let next: u32 = total[total.len() - 1] + total[total.len() - 2];
        total.push(next);
    }

    total
    // parei na pagina 83
}
