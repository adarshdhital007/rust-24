fn main() {
    let s = String::from("hello");
    let slice = get_slice(&s, 0, 2);
    println!("Slice: {}", slice);
}

fn get_slice(s: &str, start: usize, end: usize) -> &str {
    &s[start..end]
}
