// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let &Rectangle{
        p1: Point{x: x1, y: y1},
        p2: Point{x: x2, y: y2},
    } = rect;

    (x2-x1) * (y2 - y1)
}

fn square(lower_left: &Point, width: f32) -> Rectangle {
    let &Point{x: x1, y: y2} = lower_left;

    Rectangle{
        p1: Point{x: x1, y: y2},
        p2: Point{x: x1+width, y: y2+width},
    }
}

fn main(){
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    println!("S({:?}) = {}", _rectangle, rect_area(&_rectangle));
    let sq = square(&Point{x:0.0, y:0.0}, 10.0);
    println!("S({:?}) = {}", sq, rect_area(&sq));

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
