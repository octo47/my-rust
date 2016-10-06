use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut parts = buffer.split_whitespace().
        map(|x| x.trim().parse::<u32>().unwrap()).
        collect::<Vec<u32>>();
    parts.sort();
    println!("{:?}", parts);
}
