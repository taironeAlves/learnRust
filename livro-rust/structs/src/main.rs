struct User {
    // São semelhantes a propriedades de uma class
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("amado2vezes@gmail.com"),
        username: String::from("Tairone Alves Barreto"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("outro@outro.com.br");
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
