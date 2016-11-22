use std::io;

fn main() {

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim();
    if buffer.as_bytes()[buffer.len()-1] == '\n' as u8 {
        let new_len = buffer.len() - 1;
        buffer.truncate(new_len);
    }

    let pattern = vec!['S', 'O', 'S'];
    let mut pos = 0 as usize;
    let mut cnt = 0;
    for ch in buffer.chars() {
        if ch != pattern[pos] {
            cnt += 1;
        }
        pos+=1;
        pos %= 3;
    }
    println!("{}", cnt);
}
