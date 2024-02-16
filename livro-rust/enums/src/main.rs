/* enum VersaoIp {
    V4(String),
    V6(u32),
}

struct EnderecoIp {
    versao: VersaoIp,
    endereco: String,
}

fn main() {
    // let local = EnderecoIp {
    //     versao: VersaoIp::V4,
    //     endereco: String::from("127.0.0.1"),
    // };

    // let loopback = EnderecoIp {
    //     versao: VersaoIp::V6,
    //     endereco: String::from("::1"),
    // };

    // Outra forma de representar as enums sem o uso de Struct
    let outro_local = VersaoIp::V4(String::from("127.0.0.1"));
    // let outro_loopback = VersaoIp::V6(String::from("::1"));
    let outro_loopback = VersaoIp::V6(1);
}

fn rotear(versao_ip: &VersaoIp) {}
 */

// Outra forma de usar enums. As enums assim como structs podem receber métodos com impl
/*
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

 */

// Usando a enum Option<t>
// O enum Option ele faz parte do Padrao do Rust, como o rust não permite valores null
// Option retorna um Some() quando tem algum valor ou None quando não tem nada.
/* fn main (){
   let algum_numero = Some(5);
   let algum_texto = Some("um texto");

   let numero_ausente:Option<i32> = None;
} */

// Utlizando o match para capturar o valor de Option<T>

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Dime => 10,
        Moeda::Nickel => 5,
        Moeda::Quarter => 25,
    }
}

fn main() {
    let resp = valor_em_cents(Moeda::Quarter);
    println!("{}", resp);
}

