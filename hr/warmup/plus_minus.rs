use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let total = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let vec = buffer.split_whitespace().
        map(|s| s.trim().parse::<i32>().unwrap()).
        collect::<Vec<i32>>();
    let mut cnt = (0f32, 0f32, 0f32);
    for v in vec {
        if v > 0 {
            cnt.0+=1.0;
        } else if v < 0 {
            cnt.1+=1.0;
        } else {
            cnt.2+=1.0;
        }
    };
    println!("{}", cnt.0/(total as f32));
    println!("{}", cnt.1/(total as f32));
    println!("{}", cnt.2/(total as f32));
}
