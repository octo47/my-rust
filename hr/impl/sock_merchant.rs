use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut vec = buffer.split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    vec.sort();
    let mut i = 0;
    let mut pairs = 0;
    while i < vec.len()-1 {
        if vec[i] == vec[i+1] {
            i += 2;
            pairs += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", pairs);
}
