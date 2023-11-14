//Ownership and function returning ownership:
fn main() {
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("Length of '{}' is: {}", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
