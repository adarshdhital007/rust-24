fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let string3 = String::from("xy");

    let longest_result = with_elision(string1.as_str(), string2.as_str(), string3.as_str(), true);
    println!("The longest string is: {}", longest_result);

    let shortest_result = with_elision(string1.as_str(), string2.as_str(), string3.as_str(), false);
    println!("The shortest string is: {}", shortest_result);
}

fn with_elision<'a>(s1: &'a str, s2: &'a str, s3: &'a str, longest: bool) -> &'a str {
    if longest {
        if s1.len() >= s2.len() && s1.len() >= s3.len() {
            s1
        } else if s2.len() >= s1.len() && s2.len() >= s3.len() {
            s2
        } else {
            s3
        }
    } else {
        if s1.len() <= s2.len() && s1.len() <= s3.len() {
            s1
        } else if s2.len() <= s1.len() && s2.len() <= s3.len() {
            s2
        } else {
            s3
        }
    }
}
