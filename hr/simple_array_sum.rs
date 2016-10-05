use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let sum: u32 = buffer.split_whitespace().
        map(|s| s.trim().parse::<u32>().unwrap()).
        fold(0, |acc, x| acc+x);
    println!("{}", sum)
}
