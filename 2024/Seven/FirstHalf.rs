fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The sum we are looking for
    let mut sum = 0;

    // Looping for each equation
    for row in data.iter() {
        // Making an equation from the rest of the row
        let mut equation: Vec<i64> = row
            .split(|c| c == ' ' || c == ':')
            // .collect::<Vec<i64>>()
            // .iter();
            .filter(|x| *x != "")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        // The answer that we want the other side to equal
        let ans = equation.remove(0);

        // I plan to use binary to do this because I can just take the equation.len()th rightmost digits
        // If it is a zero, then it is a space, if it is a one it is a multiplier

        // Now we need to check every possible combination until it works
        // There will be 2 ^ spaces opportunities
        let base: i64 = 2;
        for bin in 0..base.pow((equation.len() - 1) as u32) {
            // The string that we will use to represent the things
            let mut operators = format!("{bin:b}");

            // It automatically gets rid of the leading zeros, so I have to add them back in
            while operators.len() < equation.len() - 1 {
                operators = format!("{}{}", "0", operators);
            }

            // Now we need to try this style
            let mut this_sum = equation[0];
            for i in 0..operators.len() {
                // Adding this one
                if operators[i as usize..(i + 1) as usize] == *"0" {
                    this_sum += equation[(i + 1) as usize];
                } else {
                    this_sum *= equation[(i + 1) as usize];
                }
            }

            // If this_sum == ans, we get to add ans to sum
            if this_sum == ans {
                sum += ans;
                break;
            }
        }
    }

    println!("Your total calibration result is {}", sum);
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Seven\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
