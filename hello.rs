use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}


struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        try!(write!(f, "["));
        for (count, i) in vec.iter().enumerate() {
            if count != 0 {try!(write!(f, ", "))}
            try!(write!(f, "{}: {}", count, i));
        }
        write!(f, "]")
    }
}

fn main() {
    println!("haha!");
    println!("{} days", 31);
    println!("{} know binary, {:b} don't", 1, 2);

    let dp = Deep(Structure(1));
    println!("{:?}", dp);
    println!("{:?}", Deep(Structure(1)));
    println!("{}", dp.0);

    let cpml = Complex{real:3.3, img:7.2};
    println!("{}", cpml);
    println!("{:?}", cpml);

    let v = List(vec![1, 2, 3]);
    println!("{}", v)
}
