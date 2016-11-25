use std::io;
use std::collections::VecDeque;

#[derive(Debug)]
struct State {
    array: Vec<i32>,
    chunks: VecDeque<Coords>,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
struct Coords {
    l: usize,
    r: usize,
}

const IS_DEBUG: bool = false;
const MAX_CHUNKS: usize = 1000;

impl Coords {
    fn adjust(&mut self, i: usize, j: usize, cut_i: usize, cut_j: usize) {
        let d1 = if i < cut_i { cut_i - i } else { 0 };
        let d2 = if j > cut_j { j - cut_j } else { 0 };
        let diff = self.r - self.l;
        if d1 > diff || d2 > diff {
            // make empty
            self.l = 0;
            self.r = 0;
        } else {
            self.l += d1;
            self.r -= d2;
        }
    }

    fn is_empty(&self) -> bool {
        self.l == self.r
    }
}

enum Op {
    Append,
    Prepend,
}

impl State {
    fn cut_chunks(&mut self, i: usize, j: usize, op: Op) {

        let mut cut_chunks = Vec::<Coords>::new();
        let mut new_chunks = VecDeque::<Coords>::with_capacity(self.chunks.len() + 4);
        let mut loff = 0;
        for chunk in &self.chunks {
            let roff = loff + (chunk.r - chunk.l);

            let mut remain = *chunk;
            remain.adjust(loff, roff, loff, i);
            if !remain.is_empty() {
                new_chunks.push_back(remain);
            }

            let mut remain = *chunk;
            remain.adjust(loff, roff, j, roff);
            if !remain.is_empty() {
                new_chunks.push_back(remain);
            }

            let mut cut = *chunk;
            cut.adjust(loff, roff, i, j);
            if !cut.is_empty() {
                cut_chunks.push(cut);
            }

            if IS_DEBUG {
                println!("[loff={},roff={}) X [i={}:j={}): {:?} new: {:?} cut: {:?}",
                         loff,
                         roff,
                         i,
                         j,
                         chunk,
                         new_chunks,
                         cut_chunks);
            }


            loff += chunk.r - chunk.l;
        }

        if IS_DEBUG {
            self.print_chunks(new_chunks.iter());
            self.print_chunks(cut_chunks.iter());
            println!("[i={}:j={}): new: {:?} cut: {:?}",
                     i,
                     j,
                     new_chunks,
                     cut_chunks);
        }

        match op {
            Op::Prepend => {
                cut_chunks.reverse();
                cut_chunks.iter()
                    .inspect(|&k| {
                        new_chunks.push_front(*k);
                    })
                    .count();
            }
            Op::Append => {
                cut_chunks.iter()
                    .inspect(|&k| {
                        new_chunks.push_back(*k);
                    })
                    .count();
            }
        }

        self.chunks = new_chunks;

        if self.chunks.len() > MAX_CHUNKS {
            self.optimize();
            if IS_DEBUG {
                println!("Optimized");
            }
        }
    }

    fn move_back(&mut self, i: usize, j: usize) {
        self.cut_chunks(i, j, Op::Append);
    }

    fn move_front(&mut self, i: usize, j: usize) {
        self.cut_chunks(i, j, Op::Prepend);
    }

    fn print(&self) {
        self.print_chunks(self.chunks.iter());
    }

    fn optimize(&mut self) {
        let mut new_array = Vec::<_>::with_capacity(self.array.len());
        for value in &self.chunks {
            for i in value.l..value.r {
                new_array.push(self.array[i]);
            }
        }
        self.array = new_array;
        self.chunks.clear();
        self.chunks.insert(0,
                            Coords {
                                l: 0,
                                r: self.array.len(),
                            });
    }

    fn print_chunks<'a, T>(&self, vec: T)
        where T: std::iter::Iterator<Item = &'a Coords>
    {
        for value in vec {
            for i in value.l..value.r {
                print!("{} ", self.array[i]);
            }
        }
        println!("");
    }
}

fn main() {
    let input = read_ints::<usize>();
    let n = input[0];
    let m = input[1];
    let mut state = State {
        array: read_ints::<i32>(),
        chunks: VecDeque::new(),
    };
    state.chunks.insert(0,
                        Coords {
                            l: 0,
                            r: state.array.len(),
                        });
    assert!(state.array.len() == n);
    for _q in 0..m {
        let query = read_ints::<usize>();
        match query[0] {
            1 => state.move_front(query[1] - 1, query[2]),
            2 => state.move_back(query[1] - 1, query[2]),
            _ => panic!("Unknown type {}", query[0]),
        }
        if IS_DEBUG{
            state.print();
        }
    }
    state.optimize();
    println!("{}", (state.array[0] - state.array.last().unwrap()).abs());
    state.print();
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
