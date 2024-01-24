// As anotações de tempo de vida situações simples não necessita ser especificada.
// O próprio compilador Rust consegue entender o tempo de vida das referencia.
// Porem, quando trabalhamos com funções um pouco mais complexas é importante
// especificar o tempo de vida para o compilador, assim deixamos o codigo a prova de falhas.
// O recomendado como sempre é deixar o mais especifico possível tudo.

fn main() {
    let name1: &str = "Joe";
    let name1: &str = "Chris";
    let name3: &str = "Anne";

    let mut names = Vec::new();
    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(names, vec!["Joe", "Chris", "Anne"])
}

fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));

    vector.get(vector.len() - 1).unwrap()
}
