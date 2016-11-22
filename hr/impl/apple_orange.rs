use std::io;

fn main() {
    let mut v = read_ints::<i64>();
    let s = v[0];
    let t = v[1];


    v = read_ints::<i64>();
    let a = v[0];
    let b = v[1];

    read_ints::<i64>();

    let apples = read_ints();
    let oranges = read_ints();

    let apples_in_range = fruits_in_range(
        a, s, t, &apples
    );
    let oranges_in_range = fruits_in_range(
        b, s, t, &oranges
    );

    println!("{}", apples_in_range);
    println!("{}", oranges_in_range);
}

fn fruits_in_range<T>(pivot: T,
                      s: T::Output, t: T::Output,
                      vec: &Vec<T>) -> i32
    where T: std::ops::Add + PartialOrd + Copy,
          T::Output: PartialOrd {
    let mut cnt = 0;
    for v in vec {
        let coord = pivot + *v;
        if coord >= s && coord <= t {
            cnt+=1;
        }
    };
    cnt
}

fn read_ints<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.split_whitespace()
        .map(|s| s.trim().parse::<_>().unwrap())
        .collect::<Vec<T>>()
}
