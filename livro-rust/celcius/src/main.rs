use std::io;

fn main() {
    loop {
        //  Entro com a mensagem de boas vindas
        println!("Entre com a temperatura em fahrenheit");

        //  Crio uma estância mutavel do tipo string
        let mut temperatura: String = String::new();

        //  Utilizo a biblioteca io::stdin().read_line() para pegar o que foi digitado pelo usuário
        io::stdin()
            .read_line(&mut temperatura)
            .expect("Faltou colocar a temperatura");

        // Na varíavel armazeno o que foi digitado pelo usuário tentando converter para int
        let resp: f32 = match temperatura.trim().parse() {
            Ok(num) => convert_to_celcius(num),
            Err(_) => continue,
        };

        println!("fahrenheit => Celcius é {}", resp);
        break;
    }
}

fn convert_to_celcius(num: f32) -> f32 {
    (&num - 32f32) * 5f32 / 9f32
}
