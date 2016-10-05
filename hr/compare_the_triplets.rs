use std::io;

struct Score {
    alice: u32,
    bob: u32,
}

fn main() {
    let alice = read_score().unwrap();
    let bob = read_score().unwrap();
    let res = diff_score(&alice, &bob);
    println!("{:?} {:?}", res.alice, res.bob)
}

fn diff_score(alice: &Vec<u32>, bob: &Vec<u32>) -> Score {
    let mut rv = Score{
        alice:0,
        bob:0,
    };
    for pair in alice.iter().zip(bob) {
        if pair.0 > pair.1 {rv.alice+=1}
        else if pair.1 > pair.0 {rv.bob+=1};
    }
   return rv
}

fn read_score() -> io::Result<Vec<u32>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    Ok(buffer.split_whitespace().
        map(|s| s.trim().parse::<u32>().unwrap()).
        collect())
}
