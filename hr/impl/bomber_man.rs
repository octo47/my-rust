use std::io;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
struct World {
    r: usize,
    c: usize,
    n: usize,
    field: Vec<Vec<Site>>,
    sec: usize,
}

#[derive(Clone,Copy,Debug)]
struct Site {
    timer: usize,
    bomb: bool,
    mark: bool,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for r in 0..self.r {
            for c in 0..self.c {
                let bomb = if self.field[r][c].bomb { "O" } else { "." };
                try!(write!(f, "{}", bomb));
            }
            try!(writeln!(f, ""));
        }
        write!(f, "")
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter) -> Result {
        try!(write!(f, "World {}x{}: sec={}\n", self.r, self.c, self.sec));
        for r in 0..self.r {
            for c in 0..self.c {
                let marked = if self.field[r][c].mark { "*" } else { " " };
                if self.field[r][c].bomb {
                    try!(write!(f, "|{:=^5}{}", self.field[r][c].timer, marked));
                } else {
                    try!(write!(f, "|{: ^5}{}", self.field[r][c].timer, marked));
                };
            }
            try!(writeln!(f, ""));
        }
        write!(f, "")
    }
}

impl World {
    fn next(&mut self) {
        match self.sec % 2 {
            // Bomberman plants bombs in all cells without bombs,
            // thus filling the whole grid with bombs. It is guaranteed that no bombs
            // will detonate at this second.
            0 => {
                self.plant_bombs();
            }
            // After one more second, any bombs planted exactly three seconds ago will
            // detonate. Here, Bomberman stands back and observes.
            1 => {
                self.detonate_bombs();
            }

            _ => panic!("impossible"),
        }
        self.sec += 1;
    }

    fn plant_bombs(&mut self) {
        for r in 0..self.r {
            for c in 0..self.c {
                if !self.field[r][c].bomb {
                    self.field[r][c].bomb = true;
                    self.field[r][c].timer = self.sec + 3;
                }
            }
        }
    }

    fn detonate_bombs(&mut self) {
        self.mark_bombs();
        self.sweep_bombs();
    }

    fn mark_bombs(&mut self) {
        for r in 0..self.r {
            for c in 0..self.c {
                let timer = self.field[r][c].timer;
                if self.field[r][c].bomb && self.sec == timer {
                    let row = r as i32;
                    let col = c as i32;
                    self.mark(row, col);
                    self.mark(row - 1, col);
                    self.mark(row, col - 1);
                    self.mark(row + 1, col);
                    self.mark(row, col + 1);
                }
            }
        }
    }

    fn sweep_bombs(&mut self) {
        for r in 0..self.r {
            for c in 0..self.c {
                if self.field[r][c].mark {
                    self.field[r][c].bomb = false;
                    self.field[r][c].mark = false;
                    self.field[r][c].timer = 0;
                }
            }
        }
    }

    fn mark(&mut self, row: i32, col: i32) {
        if row >= 0 && row < self.r as i32 && col >= 0 && col < self.c as i32 {
            self.field[row as usize][col as usize].mark = true;
        }
    }

    fn compare_state(&self, other: &World) -> bool {
        assert!(self.r == other.r);
        assert!(self.c == other.c);
        for r in 0..self.r {
            for c in 0..self.c {
                if self.field[r][c].bomb != other.field[r][c].bomb {
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() {
    let mut world = read_input();
    let mut history: Vec<(usize, World)> = vec![];
    let mut cycle = (0, 0);
    let detect_cycles = true;
    'main_loop: for step in 0..world.n {
        if detect_cycles && step > 0 && world.sec % 2 == 0 {
            for hi in history.iter() {
                if hi.1.compare_state(&world) {
                    cycle = (hi.0, step);
                    break 'main_loop;
                }
            }
            history.push((step, world.clone()));
        }
        world.next();
    }
    // if cycle was detected
    if cycle.1 != 0 && cycle.0 != 0 {
        let new_n = (world.n - cycle.0) % (cycle.1 - cycle.0);
        //println!("Cycle at {}..{}: need to run only {}!", cycle.0, cycle.1, new_n);
        for _step in 0..new_n {
            world.next();
        }
    }
    println!("{}", &world);
}

fn read_input() -> World {
    let input = read_ints::<usize>();
    let mut world = World {
        r: input[0],
        c: input[1],
        n: input[2],
        field: vec![],
        sec: 1, // starting at 1
    };

    let mut buffer = String::new();
    for _r in 0..world.r {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut row = vec![];
        for ch in buffer.chars() {
            let bomb = ch == 'O';
            row.push(Site {
                bomb: bomb,
                timer: if bomb { 3 } else { 0 },
                mark: false,
            });
        }
        world.field.push(row);
    }
    world
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
