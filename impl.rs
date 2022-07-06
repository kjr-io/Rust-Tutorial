struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    println!("My name is {}.", p.full_name());
}