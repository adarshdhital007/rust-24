//Borrowing mutable references:

fn main() {
    let mut s = String::from("hello");
    modify_string(&mut s);
    println!("{}", s);
}

fn modify_string(s: &mut String) {
    s.push_str(", world!");
}
