//Ownership and ownership transfer in a struct:
struct Person {
    name: String,
}

fn main() {
    let name = String::from("Alice");
    let person = create_person(name);
    // Uncommenting the line below would result in a compile error
    // println!("{}", name);
    println!("Person's name: {}", person.name);
}

fn create_person(name: String) -> Person {
    Person { name }
}
