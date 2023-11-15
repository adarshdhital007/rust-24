use std::net::IpAddr;

fn main() {
    println!("Enter an IP address:");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input.parse::<IpAddr>() {
        Ok(ip) => {
            if ip.is_ipv4() {
                println!("is IPv4.");
            } else if ip.is_ipv6() {
                println!("is IPv6.");
            } else {
                println!("Invalid IP address format.");
            }
        }
        Err(_) => {
            println!("Invalid IP");
        }
    }
}
