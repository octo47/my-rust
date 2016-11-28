use std::io;

fn main() {
    let input = read_ints::<i32>();
    let n = input[0];
    let m = input[1];

    // supply over the fence;
    //  _ _
    // | | |....
    //  -x-
    // | | |
    //  - -
    //  ...
    //
    let mut fences = 0;

    fences += (n / 2) * (m / 2);

    if n % 2 > 0 && m % 2 > 0 {
        fences += 1; // right corner;
    }

    // leftover right/bottom lines.
    // -----|x
    // -----|x
    // ----- x
    // xxxxxxx
    if m % 2 > 0 {
        fences += n / 2;
    }
    if n % 2 > 0 {
        fences += m / 2;
    }

    println!("{}", fences);
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
