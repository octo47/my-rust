use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let nkq = buffer.split_whitespace().
        map(|x| x.trim().parse::<u32>().unwrap()).
        collect::<Vec<u32>>();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let v = buffer.split_whitespace().
        map(|x| x.trim().parse::<u32>().unwrap()).
        collect::<Vec<u32>>();
    let shift = (nkq[1] % (v.len() as u32)) as usize;
    for _ in 0..nkq[2] {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let q = buffer.trim().parse::<usize>().unwrap();
        let offset = if q > shift {
            q - shift
        } else {
            (v.len() + q - shift)%v.len()
        };
        println!("{}", v[offset]);
    }
}
