fn main() {
    let john = Person{
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };

    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };

    assert_eq!(build_full_name(&alice), "Alice Stevens");

    

}

struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(middle) = &person.middle {
        full_name.push_str(&middle);
        full_name.push_str(" ")
    }

    full_name.push_str(&person.last);

    full_name // return
}

