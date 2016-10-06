use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut parts = buffer[0..8].split(":").
        map(|x| x.trim().parse::<i32>().unwrap()).
        collect::<Vec<i32>>();
    if &buffer[8..10]=="PM" {
        if parts[0] != 12 {
            parts[0]+=12;
        }
    } else if &buffer[8..10]=="AM" {
        if parts[0] == 12 {
            parts[0] = 0;
        }
    }
    println!("{:02}:{:02}:{:02}", parts[0],parts[1],parts[2])
}
