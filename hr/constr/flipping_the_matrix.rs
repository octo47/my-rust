use std::io;

fn main() {
    let num_q = read_int();
    for _q in 0..num_q {
        let mut matrix: Vec<Vec<i32>> = vec![];
        let n = read_int();
        for _i in 0..n * 2 {
            let row = read_ints::<i32>();
            assert!(row.len() == (n * 2) as usize);
            matrix.push(row);
        }

        let mut sum = 0;
        for i in 0..n {
            for j in 0..n {
                sum += std::cmp::max(
                    matrix[i][j], std::cmp::max(
                        matrix[i][2*n - j - 1], std::cmp::max(
                            matrix[2*n - i - 1][j],
                            matrix[2*n - i - 1][2*n - j - 1]
                        )
                    )
                )
            }
        }

        println!("{}", sum);
    }
}


fn read_int() -> usize {
    let input = read_ints::<usize>();
    input[0]
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
