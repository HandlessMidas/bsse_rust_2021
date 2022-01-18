fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn read_int() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let ans = ((random_int()) % 100) as u8;
    loop {
        let current_num = read_int();
        if ans == current_num {
            break;
        }
        println!("The number is {}", if ans > current_num { "bigger" } else { "smaller" });
    }
    println!("That's right. The number is {}", ans)
}