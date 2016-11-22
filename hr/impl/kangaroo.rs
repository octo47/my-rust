use std::io;

struct Kangaroo {
    pos: i32,
    velocity: i32,
}

impl Kangaroo {
    fn advance(&mut self) {
        self.pos += self.velocity;
    }
}

fn main() {
    let mut input = read_input();
    input.sort_by(|a, b| a.pos.cmp(&b.pos));
    while input[0].pos <= input[1].pos {
        if input[0].pos == input[1].pos {
            println!("YES");
            return;
        }
        input[0].advance();
        input[1].advance();
    }
    println!("NO");
}

fn read_input() -> Vec<Kangaroo> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let input = buffer.split_whitespace()
            .map(|s| s.trim().parse::<_>().unwrap())
            .collect::<Vec<i32>>();
        vec![
            Kangaroo{pos:input[0], velocity: input[1]},
            Kangaroo{pos:input[2], velocity: input[3]},
        ]
}
