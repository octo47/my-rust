use std::io;
use std::f32;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Clone,Copy,Default,Debug)]
struct Vector2 {
    s: Point2,
    e: Point2,
}

impl Vector2 {
    fn from_points(from: &Point2, to: &Point2) -> Vector2 {
        Vector2{s: *from, e: *to}
    }

    fn normalize(mut self) -> Vector2 {
        self.e.x = self.e.x - self.s.x;
        self.e.y = self.e.y - self.s.y;
        let len = self.len();
        self.e.x /= len;
        self.e.y /= len;
        self.s.x = 0.0;
        self.s.y = 0.0;
        self
    }

    #[inline]
    fn len(&self) -> f32 {
        (self.x_len().powi(2) +
         (self.y_len().powi(2))).sqrt()
    }

    #[inline]
    fn x_len(&self) -> f32 {
        self.e.x - self.s.x
    }

    #[inline]
    fn y_len(&self) -> f32 {
        self.e.y - self.s.y
    }

    fn angle(&self, other: &Vector2) -> f32 {
        let v1 = self.clone().normalize();
        let v2 = other.clone().normalize();
        let denom = v1.len() * v2.len();
        if denom == 0.0 {
            return f32::INFINITY
        };
        let numerator = v1.e.x * v2.e.x + v1.e.y * v2.e.y;
        (numerator / denom).acos()
    }
}

#[derive(Clone,Copy,Default,Debug)]
struct Point2 {
    x: f32,
    y: f32,
}

#[derive(Clone,Copy,Default,Debug)]
struct Bot {
    initialized: bool,
    boosts: i32,
    speed: Vector2,
    pos: Point2,
}

impl Bot {
    fn update(&mut self, new_pos: &Point2) {
        if !self.initialized {
            self.pos = *new_pos;
            self.initialized = true;
        } else {
            self.speed = Vector2::from_points(&self.pos, &new_pos);
            self.pos = *new_pos;
        }
    }

    fn abs_speed(&self) -> f32 {
        self.speed.len()
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    let pi = f32::consts::PI;
    let mut max_speed = 600.0;
    let corner_speed = 300.0;
    let speed_reduce = 100.0;
    let mut bot = Bot{boosts: 1, ..Bot::default()};

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let curr_pos = Point2{
            x: parse_input!(inputs[0], f32),
            y: parse_input!(inputs[1], f32),
        };

        let next_checkpoint_x = parse_input!(inputs[2], f32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], f32); // y position of the next check point
        let next_checkpoint_dist = parse_input!(inputs[4], f32); // distance to the next checkpoint
        let next_checkpoint_angle = parse_input!(inputs[5], f32); // angle between your pod orientation and the direction of the next checkpoint

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        //let inputs = input_line.split(" ").collect::<Vec<_>>();
        //let opponent_x = parse_input!(inputs[0], i32);
        //let opponent_y = parse_input!(inputs[1], i32);

        bot.update(&curr_pos);
        if bot.speed.len() > max_speed {
            max_speed = bot.speed.len();
        }

        let mut target = Point2{x: next_checkpoint_x, y: next_checkpoint_y};
        // calculate angle between target point and current speed vector
        let bearing_angle = Vector2::from_points(
            &curr_pos,
            &target
        ).angle(&bot.speed) / pi * 180.0;

        print_err!("Angle: {} {}", bearing_angle, next_checkpoint_angle);
        let steps_to_target = next_checkpoint_dist / bot.speed.len();
        print_err!("Speed: {}, turns to target: {}",
                   bot.speed.len(), steps_to_target);

        let mut thrust = 100.0;

        if next_checkpoint_angle.abs() > 90.0 {
            print_err!("Bearing is too large!");
            thrust = 10.0;
        }

        if steps_to_target < 10.0 && bearing_angle > 45.0 {
            thrust = 50.0;
        }
        if steps_to_target < 5.0 && bearing_angle > 45.0 {
            thrust = 0.0;
        }

        if thrust > 70.0 && bot.boosts > 0 && next_checkpoint_dist > 3000.0 {
            thrust = 101.0;
            bot.boosts-=1;
        }

        // if next_checkpoint_dist < 900 {
        //     target.x = curr_pos.x - bot.speed.x_len();
        //     target.y = curr_pos.y - bot.speed.y_len();
        // }

        if thrust == 101.0 {
            println!("{:.0} {:.0} BOOST", target.x , target.y);
        } else {
            println!("{:.0} {:.0} {:.0}", target.x , target.y, thrust);
        }
    }
}
