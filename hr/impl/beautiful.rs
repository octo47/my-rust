use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let vec = buffer.split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let k = vec[2];
    let mut beautiful = 0;
    for v in vec[0]..vec[1]+1 {
        if (v - reverse_num(v)) % k == 0 {
            beautiful+=1;
        }
    };
    println!("{}", beautiful);
}

fn reverse_num(num: i32) -> i32 {
    let mut rval = 0 as i32;
    let mut orig = num;

    while orig > 0 {
        rval = rval * 10 + orig % 10;
        orig = orig / 10;
    };

    rval
}
