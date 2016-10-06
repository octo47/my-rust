use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();
    let mut matrix = Vec::<Vec<i32>>::new();
    for _ in 0..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let vec = buffer.split_whitespace().
            map(|x| x.trim().parse::<i32>().unwrap()).
            collect::<Vec<i32>>();
        matrix.push(vec)
    };
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..n {
        sum1= sum1+matrix[i][i];
        sum2=sum2+matrix[i][n-i-1];
    };

    println!("{:?}", (sum2-sum1).abs())
}
