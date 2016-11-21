use std::io;

fn main()  {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut b = 0;
    let mut words: Vec<&str> = vec![];
    for (idx, ch) in buffer.chars().enumerate() {
        if ch.is_uppercase() {
            words.push(&buffer[b..idx]);
            b = idx;
        }
    }
    if b != buffer.len() {
        words.push(&buffer[b..buffer.len()]);
    }

    //let cnt = words.iter().inspect(|e| println!("{}", e)).count();
    println!("{}", words.len());
}
