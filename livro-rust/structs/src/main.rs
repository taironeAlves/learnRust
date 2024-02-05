/*
struct User {
    // São semelhantes a propriedades de uma class
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("amado2vezes@gmail.com"),
        username: String::from("Tairone Alves Barreto"),
        sign_in_count: 1,
        active: true,
    };

    let user2: User = User {
        username: String::from("Tenhoster Master Services"),
        email: String::from("tenhoster@gmail.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user: User = User {
        username: String::from("Evandro Menezete"),
        email: String::from("menezete@gmail.com"),
        ..user2
    };

    user1.email = String::from("outro@outro.com.br");

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
 */


 struct User{
    // São semelhantes a propriedades de uma class
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1: User = User {
        username: String::from("Tese") ,
         email:  String::from("teste@teste.com.br"),
        sign_in_count: 1,
        active: true,
    };
}
