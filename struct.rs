struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    println!("My name is {} {}.", p.first_name, p.last_name);
}

