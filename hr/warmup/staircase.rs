use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let size = buffer.trim().parse::<usize>().unwrap();
    for i in 0..size {
        for _ in 0..(size-i-1) {
            print!(" ")
        }
        for _ in 0..(i+1) {
            print!("#")
        }
        println!("")
    }
}
