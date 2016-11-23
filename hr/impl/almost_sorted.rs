use std::io;

fn main() {
    let n = read_int::<usize>();
    let d = read_ints::<i32>();
    assert!(n == d.len());

    // lookup for first in a wrong place
    let mut first: Option<usize> = None;
    for i in 1..d.len() {
        if d[i - 1] > d[i] {
            first = Some(i - 1);
            break;
        }
    }
    if !first.is_some() {
        println!("yes");
        return;
    }

    let next_val = d[first.unwrap() + 1];
    let mut second_swap: Option<usize> = None;
    // now scan array to find something smaller then
    for i in first.unwrap() + 1..d.len() {
        if d[i] < next_val {
            if second_swap.is_some() {
                second_swap = None;
                break;
            }
            if i < d.len() -1 && d[first.unwrap()] < d[i+1] {
                second_swap = Some(i);
            }
        }
    }

    // ok lets try to find sequence
    let mut sequence_end: Option<usize> = None;
    for i in first.unwrap() + 1..d.len() {
        // if it reverse ordered sequence
        if d[i] < d[i - 1] {
            sequence_end = Some(i);
        } else {
            break;
        }
    }
    if sequence_end.is_some() && sequence_end.unwrap() < d.len() - 1 {
        // check that end sequence will sort array
        if d[first.unwrap()] > d[sequence_end.unwrap()+1] {
            sequence_end = None;
        }
    }
    if sequence_end.is_some() && first.unwrap() > 0 {
        // check that first sequence element will be larger then previous
        if d[sequence_end.unwrap()] < d[first.unwrap()-1] {
            sequence_end = None;
        }
    }

    // last check
    let check = sequence_end.or(second_swap).or(first);
    /*
    println!("{:?}, {:?}, {:?} {:?}",
             first,
             second_swap,
             sequence_end,
             check);
    */
    for i in check.unwrap() + 1..d.len() {
        if d[i - 1] > d[i] {
            // ok, it seems we have another disorder.
            println!("no");
            return;
        }
    }


    if sequence_end.is_some() {
        println!("yes");
        if sequence_end.unwrap() - first.unwrap() > 1 {
            println!("reverse {} {}",
                     first.unwrap() + 1,
                     sequence_end.unwrap() + 1);
        } else {
            println!("swap {} {}", first.unwrap() + 1, sequence_end.unwrap() + 1);
        }
    } else if second_swap.is_some() {
        println!("yes");
        println!("swap {} {}", first.unwrap() + 1, second_swap.unwrap() + 1);
    } else {
        println!("no")
    }
}

fn read_int<T>() -> T
    where T: std::str::FromStr + Copy,
          T::Err: std::fmt::Debug
{
    read_ints::<T>()[0]
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
