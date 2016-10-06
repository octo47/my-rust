use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();

    io::stdin().read_line(&mut buffer).unwrap();
    let sum = buffer.split_whitespace().
       map(|s| s.trim().parse::<i64>().unwrap()).
        fold(0i64, |accum, x| accum+x);
    println!("{}", sum)
}
