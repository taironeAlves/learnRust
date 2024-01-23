fn main() {
    // Aqui será mostrado como o rust trabalha com transferencia de propriedade.
    // Variáveis também são chamadas de associações

    // Variaveis são válidas somente em seu escopo

    /*
    {
        let mascot = String::from("Ferris");
    }
        println!("{}", mascot);

        */

    /*

    // O Rust faz transferencia de propriedade liberando a memoria quando uma variável é referenciada por outra.
    {
        let mascot = String::from("Ted");
        let ted = mascot;

        println!("{}", mascot);
    }
     */

    //  O Rust aplica a mesma regra para funções, se uma variavel for passada como parametro, ela deixa
    // de pertencer a o antigo escopo.
    /*
    fn process(input: String){}

    fn caller() {
        let s = String::from("Olá Tairone");
        process(s);
        process(s);
    }
    */

    // O rust realiza copia de uma variável quando for do tipo primitivo automaticamente, mas
    // se for de outros tipos ele move. Se for de baixo custo copia, se for de alto custo move.
    /*
    fn process(input: u32){}

    fn caller() {
        let n: u32 = 1;
        process(n);
        process(n);
    }
     */

    //  A forma mais segura é fazer um clone da propriedade, assim, o rust mantem a variável original
    // com seu respectivo valor.
    // Com o .clone() é possível criar uma cópia, mas isso gera um alto custo.
    // O baixo custo é recomendado, então é melhor trabalhar com emprestimos.

    /*
    fn process(s: String) {}

    let s = String::from("Olá Tairone");
    process(s.clone());
    process(s); // aqui ele ainda é usado
    process(s); // aqui vai dar erro porque s não existe mais.
     */


}
