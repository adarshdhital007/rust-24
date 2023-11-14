//References and borrowing: & and &mut:
fn main(){
    let s1=String::from("hello world");
    let len=calculate_length(&s1);
    println!("length is '{}' is :  {}",s1,len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}