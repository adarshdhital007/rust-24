fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result_longest = longest_with_elision(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result_longest);

    let result_shortest = shortest_with_elision(string1.as_str(), string2.as_str());
    println!("The shortest string is: {}", result_shortest);
}

fn longest_with_elision(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn shortest_with_elision(s1: &str, s2: &str) -> &str {
    if s1.len() < s2.len() {
        s1
    } else {
        s2
    }
}
