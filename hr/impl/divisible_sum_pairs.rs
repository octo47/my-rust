use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let consts = buffer.split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    buffer.clear();
    let k = consts[1];
    io::stdin().read_line(&mut buffer).unwrap();
    let vec = buffer.split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut pairs = 0;
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            let sum = vec[i] + vec[j];
            if sum % k == 0 {
                pairs += 1;
            }
        }
    }
    println!("{}", pairs);
}
