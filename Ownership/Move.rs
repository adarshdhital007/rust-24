fn main() {
    let s1 = String::from("hello");
    let s2 = take_ownership(s1);
    // Uncommenting the line below would result in a compile error
    // println!("{}", s1);
    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    s
}
