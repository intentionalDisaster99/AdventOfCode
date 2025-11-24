// OOP is so not necessary here

fn main() {
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    // Creating the games Vector
    let games: Vec<Game> = build_games(data);

    // Now we just have to find the sum
    let total_cost: i128 = games.iter().map(|g| g.cost).sum();

    println!(
        "The total price to win each game is {} tokens not 98944414581661",
        total_cost
    );
}

// A function to build a vector of Games holding the data
fn build_games(input: Vec<String>) -> Vec<Game> {
    // A record of what we need to put in
    let mut a: (i128, i128) = (0, 0);
    let mut b: (i128, i128) = (0, 0);
    let mut prize: (i128, i128);

    // The output
    let mut output: Vec<Game> = Vec::new();

    for row in input.iter() {
        // If it contains button, then we need to read the things
        if row.contains("Button") {
            // Reading the x and y values
            let y = row[(row.find("Y+").unwrap() + 2)..]
                .parse::<i128>()
                .unwrap();
            let x = row[(row.find("X+").unwrap() + 2)..row.find(",").unwrap()]
                .parse::<i128>()
                .unwrap();

            // Putting it in the right place depending on whether it is a or b
            if row.contains("A") {
                a = (x, y);
            } else {
                b = (x, y);
            }
        } else if row.contains("Prize") {
            let px = row[(row.find("X=").unwrap() + 2)..row.find(",").unwrap()]
                .parse::<i128>()
                .unwrap()
                + 10000000000000;
            let py = row[(row.find("Y=").unwrap() + 2)..]
                .parse::<i128>()
                .unwrap()
                + 10000000000000;
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
    a: (i128, i128),
    b: (i128, i128),

    // Tuple for the location of the price
    prize_pos: (i128, i128),

    // The least number of tokens that you can use to get the prize
    cost: i128,
}

impl Game {
    // A function that finds the number of tokens it takes to win the prize
    // This is going to return self to make it slightly more efficient
    fn calc_cost(&mut self) -> &mut Game {
        // Mathematically, if there are no more than 100 moves, then there are 1,267,650,600,228,229,401,496,703,205,376 possible combinations each time.
        // Let's not brute force it this time

        // Wait, this is just a system of equations
        // I did the math in mathcha, it's in a PDF file in this folder (If I remember to add it)

        // Okay, I'm going to try to rewrite it so that all of my stuff keeps from doing division as long as possible and makes sure that they don't give any remainder. That way, I don't have to play with any more stupid floating point arithmetic

        // Calculations for b
        let b_denom = self.b.1 * self.a.0 - self.b.0 * self.a.1;
        if b_denom == 0 {
            return self;
        }
        let b_numer = self.prize_pos.1 * self.a.0 - self.prize_pos.0 * self.a.1;
        if b_numer % b_denom != 0 {
            return self;
        }
        let b = b_numer / b_denom;

        // Calculations for A
        let a_denom = self.a.0; // Checked against zeros earlier
        let a_numer = self.prize_pos.0 - b * self.b.0;
        if a_numer % a_denom != 0 {
            return self;
        }
        let a = a_numer / a_denom;

        // Now we need to calculate the price, 3 for each a and 1 for each b
        self.cost = (3 * a + b);
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
