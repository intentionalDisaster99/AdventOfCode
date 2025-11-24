// OOP is so not necessary here

fn main() {
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    // Creating the games Vector
    let games: Vec<Game> = build_games(data);

    // Now we just have to find the sum
    let total_cost: usize = games.iter().map(|g| g.cost).sum();

    println!("The total price to win each game is {} tokens", total_cost);
}

// A function to build a vector of Games holding the data
fn build_games(input: Vec<String>) -> Vec<Game> {
    // A record of what we need to put in
    let mut a: (isize, isize) = (0, 0);
    let mut b: (isize, isize) = (0, 0);
    let mut prize: (isize, isize);

    // The output
    let mut output: Vec<Game> = Vec::new();

    for row in input.iter() {
        // If it contains button, then we need to read the things
        if row.contains("Button") {
            // Reading the x and y values
            let y = row[(row.find("Y+").unwrap() + 2)..]
                .parse::<isize>()
                .unwrap();
            let x = row[(row.find("X+").unwrap() + 2)..row.find(",").unwrap()]
                .parse::<isize>()
                .unwrap();

            // Putting it in the right place depending on whether it is a or b
            if row.contains("A") {
                a = (x, y);
            } else {
                b = (x, y);
            }
        } else if row.contains("Prize") {
            let px = row[(row.find("X=").unwrap() + 2)..row.find(",").unwrap()]
                .parse::<isize>()
                .unwrap();
            let py = row[(row.find("Y=").unwrap() + 2)..]
                .parse::<isize>()
                .unwrap();
            prize = (px, py);

            // Making and adding the game
            output.push(
                Game {
                    a: a,
                    b: b,
                    prize_pos: prize,
                    cost: 0,
                }
                .calc_cost()
                .clone(),
            );
        }
    }
    output
}

// I'm going to go ahead and make an object that represents each game
#[derive(Clone)]
struct Game {
    // A tuple for the distance of each button
    a: (isize, isize),
    b: (isize, isize),

    // Tuple for the location of the price
    prize_pos: (isize, isize),

    // The least number of tokens that you can use to get the prize
    cost: usize,
}

impl Game {
    // A function that finds the number of tokens it takes to win the prize
    // This is going to return self to make it slightly more efficient
    fn calc_cost(&mut self) -> &mut Game {
        // Mathematically, if there are no more than 100 moves, then there are 1,267,650,600,228,229,401,496,703,205,376 possible combinations each time.
        // Let's not brute force it this time

        // Wait, this is just a system of equations
        // I did the math in mathcha, it's in a PDF file in this folder (If I remember to add it)

        // Calculations for b
        let b: f64 = (((self.prize_pos.1 as f64)
            - ((self.prize_pos.0 as f64 * self.a.1 as f64) / self.a.0 as f64))
            / (self.b.1 as f64 - ((self.b.0 as f64 * self.a.1 as f64) / (self.a.0 as f64))));

        // Calculations for a
        let a: f64 = ((self.prize_pos.0 as f64 - b * self.b.0 as f64) / self.a.0 as f64);

        // Making sure that our answer is an integer (I can't above because of weird rounding stuff)
        let remainder = f64::abs(
            ((self.prize_pos.1 as f64)
                - ((self.prize_pos.0 as f64 * self.a.1 as f64) / self.a.0 as f64))
                % (self.b.1 as f64 - ((self.b.0 as f64 * self.a.1 as f64) / (self.a.0 as f64))),
        );

        if f64::abs(f64::abs(a) - f64::abs(f64::round(a))) > 0.0001
            || f64::abs(f64::abs(b) - f64::abs(f64::round(b))) > 0.0001
        {
            return self;
        }

        // If either button is pressed more than 100 times, we throw it out
        if a.round() > 100.0 || b.round() > 100.0 {
            return self;
        }

        // Now we need to calculate the price, 3 for each a and 1 for each b
        self.cost = (3.0 * a.round() + b.round()) as usize;
        return self;
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
