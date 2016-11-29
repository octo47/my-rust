use std::io;


fn main() {
    let input = read_ints::<i32>();
    let n = input[0];

    let mut best = (0, 0);
    let mut i = n;
    while i > 0 {
        if n % i == 0 {
            best = choose_better(&best, &i);
            //println!("{:?}", best);
        }
        i -= 1;
    }
    println!("{}", best.0)
}

fn choose_better(best: &(i32, i32), candidate: &i32) -> (i32, i32) {
    let new_sum = calc_sum(candidate);
    if new_sum > best.1 || (new_sum == best.1 && *candidate < best.0) {
        //println!("better now {}", candidate);
        (*candidate, new_sum)
    } else {
        *best
    }
}

fn calc_sum(num: &i32) -> i32 {
    let mut sum = 0;
    let mut curr = *num;
    while curr > 0 {
        sum += curr % 10;
        curr /= 10;
    }
    //println!("{}->{}", num, sum);
    sum
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
