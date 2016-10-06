use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut parts = buffer.split_whitespace().
        map(|x| x.trim().parse::<u64>().unwrap()).
        collect::<Vec<u64>>();
    parts.sort_by(|a, b| a.cmp(b).reverse());
    if parts.len() < 3 {
        println!("-1");
        return;
    }
    let mut candidates = vec![];
    let mut perimeter = 0;
    for s1 in 0..(parts.len() - 2) {
        for s2 in (s1+1)..(parts.len() - 1) {
            for s3 in (s2+1)..parts.len() {
                let (v1,v2,v3)=(parts[s1],parts[s2],parts[s3]);
                if v1 >= (v2 + v3) {
                    continue
                }
                let mut v = vec![v1,v2,v3];
                v.sort();
                let cper = v.iter().fold(0, |acc, &x| acc+x);
                if perimeter == 0 {
                    perimeter = cper;
                }
                if cper == perimeter {
                    candidates.push(v)
                }
            }
        }
    }
    if candidates.len() > 0 {
        candidates.sort_by(cmp_sides);
        let lc = candidates.last().unwrap();
        println!("{} {} {}", lc[0], lc[1], lc[2]);
    } else {
        println!("-1");
    }
}

fn cmp_sides(a: &Vec<u64>, b: &Vec<u64>) -> std::cmp::Ordering {
    let o1 = a[2].cmp(&b[2]);
    if o1 == std::cmp::Ordering::Equal{
        o1
    } else {
        a[0].cmp(&b[0])
    }
}
