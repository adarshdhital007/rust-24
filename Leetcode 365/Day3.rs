// 2125 Number of Laser Beams in a Bank

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let n = bank.len();

    let mut prev_bank_count = 0;

    let mut result = 0;
    for i in 0..n {
        let mut curr_bank_count = 0;
        for ch in bank[i].chars() {
            if ch == '1' { 
                curr_bank_count += 1;
            }
        }
        result += prev_bank_count * curr_bank_count;
        if curr_bank_count != 0 {
            prev_bank_count = curr_bank_count;
        }
    }
    result as i32
}

fn main() {
    let bank = vec![
        String::from("1001"),
        String::from("0110"),
        String::from("0011"),
        String::from("1100"),
    ];

    let result = number_of_beams(bank);

    println!("Number of Laser Beams: {}", result); //12
}
