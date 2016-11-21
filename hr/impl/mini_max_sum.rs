use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut vec = buffer.split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    vec.sort();
    let max = vec.iter().skip(1).fold(0, |a, e| a + e);
    let min = vec.iter().take(4).fold(0, |a, e| a + e);
    println!("{} {}", min, max);
}
