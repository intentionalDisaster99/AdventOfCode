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
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .unwrap_or_else(|_| {
                eprintln!("Error parsing row: {}", row);
                vec![]
            });

        // I am not getting the right answer, so here are extra precautions
        if equation.is_empty() {
            eprintln!("Skipping malformed or empty row: {}", row);
            continue;
        }

        // The answer that we want the other side to equal
        let ans = equation.remove(0);

        // I plan to use binary to do this because I can just take the equation.len()th rightmost digits
        // If it is a zero, then it is a space, if it is a one it is a multiplier

        // Now we need to check every possible combination until it works
        sum += check_combos(equation, ans);
    }

    println!("Your total calibration result is {}", sum);
}

// A function to check the different combinations that might get it to work
fn check_combos(equation: Vec<i64>, ans: i64) -> i64 {
    // We are in base 3 now :O
    let base: i64 = 3;

    for bin in 0..base.pow((equation.len() - 1) as u32) {
        // The string that we will use to represent the things
        let mut operators = format_radix(bin as u32, base as u32);

        // It automatically gets rid of the leading zeros, so I have to add them back in
        while operators.len() < equation.len() - 1 {
            operators = format!("{}{}", "0", operators);
        }

        // Now we need to try this style
        let mut this_sum = equation[0];
        for i in 0..operators.len() {
            // Breaking out if it is higher than the answer because we don't have subtraction
            if this_sum > ans {
                break;
            }

            // Adding in this
            this_sum = match &operators[i as usize..(i + 1) as usize] {
                "0" => {
                    match this_sum.checked_add(equation[i + 1]) {
                        Some(value) => value,
                        _ => ans + 1, // Overflow, so we can just return the answer + 1 (to make it never right)
                    }
                }
                "1" => {
                    match this_sum.checked_mul(equation[i + 1]) {
                        Some(value) => value,
                        _ => ans + 1, // Overflow, so we can just return the answer + 1 (to make it never right)
                    }
                }
                "2" => {
                    // Trying to make them concatenate
                    let concatenated = format!("{}{}", this_sum, equation[i + 1]);
                    if let Ok(value) = concatenated.parse::<i64>() {
                        value
                    } else {
                        eprintln!("Overflow or invalid concat: {}", concatenated);
                        ans + 1 // Fail-safe return
                    }
                }
                value => {
                    // Idk how this would ever happen, but here's an error message for it
                    panic!(
                        "How in the world did you break my code????\nIt got a {}",
                        value
                    );
                }
            };
        }

        // If this_sum == ans, we get to add ans to sum
        if this_sum == ans {
            return ans;
        }
    }
    0
}

// There's not a way to play with base 3 yet in std, so
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
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
