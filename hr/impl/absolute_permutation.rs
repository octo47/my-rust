use std::io;

// https://www.hackerrank.com/challenges/absolute-permutation

fn main() {
    let cases = read_ints::<i32>();
    for _case in 0..cases[0] {
        print_vec(&handle_case());
    }
}

fn handle_case() -> Vec<i32> {
    // read N and K
    let input = read_ints::<i32>();
    let n = input[0];
    let k = input[1];
    let mut coll = vec![];
    if k == 0 {
        for i in 1..(n + 1) {
            coll.push(i);
        }
        return coll;
    } else if n % k != 0 {
        return vec![-1];
    }
    for m in 0..(n/k/2) {
        for i in 1..(k+1) {
            coll.push(2* k * m + i + k);
        }
        for i in (k+1)..(2*k+1) {
            coll.push(2* k * m + i-k);
        }
    }
    coll
}

fn print_vec<T>(vec: &Vec<T>)
    where T: ToString
{
    let str_list = vec.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", str_list);
}

fn read_ints<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.split_whitespace()
        .map(|s| s.trim().parse::<_>().unwrap())
        .collect::<Vec<T>>()
}
