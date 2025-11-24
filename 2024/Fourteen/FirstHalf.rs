fn main() {
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    // Making robots out of the input
    let mut bots: Vec<Robot> = data
        .iter()
        .map(|row| build_robot(row.clone()))
        .collect::<Vec<Robot>>();

    // The bounds
    let bounds: (isize, isize) = (101, 103);

    // Iterating over each one 100 times
    for _ in 0..100 {
        bots.iter_mut().for_each(|bot| bot.move_once(bounds));
    }

    // The quadrants (we will be multiplying each element)
    let mut quadrants: [isize; 4] = [0; 4];

    // Figuring out how many are in each quadrant
    bots.iter_mut()
        .for_each(|bot| match bot.check_quad(bounds) {
            Some(value) => quadrants[value as usize] += 1,
            None => {}
        });

    // Finding the actual value
    let mut total = 1;
    for quad in quadrants.iter() {
        total *= quad;
    }

    println!("The total safety factor is {}", total);
}

// A function that makes a Robot out of a line of input
fn build_robot(row: String) -> Robot {
    // Initializing each value
    let mut pos: (isize, isize) = (0, 0);
    let mut vel: (isize, isize) = (0, 0);

    // Finding pos
    pos.0 = row[(row.find("p=").unwrap() + 2)..row.find(",").unwrap()]
        .parse::<isize>()
        .unwrap();
    pos.1 = row[(row.find(",").unwrap() + 1)..row.find(" v").unwrap()]
        .parse::<isize>()
        .unwrap();

    // The velocity offset
    let vel_offset = row.find("v=").unwrap() - 1;

    // Finding vel
    vel.0 = row[vel_offset..]
        [(row[vel_offset..].find("v=").unwrap() + 2)..row[vel_offset..].find(",").unwrap()]
        .parse::<isize>()
        .unwrap();
    vel.1 = row[vel_offset..][(row[vel_offset..].find(",").unwrap() + 1)..]
        .parse::<isize>()
        .unwrap();

    Robot {
        pos: pos,
        vel: vel,
        quad: None,
    }
}

// Abstracting each robot away
struct Robot {
    // The position (x, y)
    pos: (isize, isize),

    // The velocity
    vel: (isize, isize),

    // The quadrant (We start at None so this is an option)
    quad: Option<isize>,
}

impl Robot {
    // To move the robot one time
    fn move_once(&mut self, bound: (isize, isize)) {
        // Increasing the position based on the velocity
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;

        // Wrapping if needed
        if self.pos.0 < 0 {
            self.pos.0 = (self.pos.0 % bound.0 + bound.0) % bound.0;
        }
        if self.pos.1 < 0 {
            self.pos.1 = (self.pos.1 % bound.1 + bound.1) % bound.1;
        }
        if self.pos.0 >= bound.0 {
            self.pos.0 %= bound.0;
        }
        if self.pos.1 >= bound.1 {
            self.pos.1 %= bound.1;
        }
    }

    // A function to check which quadrant it's in
    fn check_quad(&mut self, bound: (isize, isize)) -> Option<isize> {
        let quad_bound: (isize, isize) = (bound.0 / 2, bound.1 / 2);

        // If it is directly in the middle then it isn't counted
        if self.pos.0 == quad_bound.0 || self.pos.1 == quad_bound.1 {
            return None;
        }

        self.quad = match (quad_bound.0 > self.pos.0, quad_bound.1 > self.pos.1) {
            (true, true) => Some(3),
            (false, true) => Some(0),
            (true, false) => Some(2),
            (false, false) => Some(1),
        };

        self.quad
    }
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents(test: bool) -> String {
    let file_path = if test { "Test.txt" } else { "Input.txt" };
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_lines(s: &String) -> Vec<String> {
    s.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}
