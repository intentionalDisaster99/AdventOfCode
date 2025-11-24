fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The amount safe
    let mut count: i64 = 0;

    // Looping for each report
    for line in data.iter() {
        // Finding this report
        let report: Vec<i64> = line.split(' ').map(|a| a.parse::<i64>().unwrap()).collect();

        // Recording the last number
        let mut last: i64 = -1;

        // To make sure it is only increasing or decreasing
        let mut increasing: bool = true;
        let mut decreasing: bool = true;

        // A variable to check to see if we need to use the dampener
        let mut safe: bool = true;

        // Looping to check and see if it is safe
        for (i, &number) in report.iter().enumerate() {
            // We can't do anything if it is the first number
            if last == -1 {
                // Updating the last value
                last = number;
                continue;
            }

            // If the difference is less than 1 or greater than 3 it is unsafe
            if (number - last).abs() < 1 || (number - last).abs() > 3 {
                safe = false;
                break; // it is unsafe
            }

            // Checking to see if it is still increasing and decreasing
            if number > last {
                decreasing = false;
            }
            if number < last {
                increasing = false;
            }

            // If it is neither only increasing or decreasing, then it is unsafe
            if !increasing && !decreasing {
                safe = false;
                break;
            }

            // If we made it this far and it is the last one, then we have found a safe report
            if i == report.len() - 1 {
                count += 1;
            }

            // Updating the last value
            last = number;
        }

        // If safe is not true here, we are going to go through ignoring each element to see if it works then
        if !safe {
            for j in 0..report.len() {
                // Reinitializing for the new check
                let mut last: i64 = -1;

                // To make sure it is only increasing or decreasing
                let mut increasing: bool = true;
                let mut decreasing: bool = true;

                // A variable to record whether we it was safe or not
                let mut safe: bool = true;

                // j is the index we are ignoring
                // Looping to check and see if it is safe now
                for (i, &number) in report.iter().enumerate() {
                    // Skipping if it is j
                    if i == j {
                        continue;
                    }

                    // We can't do anything if it is the first number
                    if last == -1 {
                        // Updating the last value
                        last = number;
                        continue;
                    }

                    // If the difference is less than 1 or greater than 3 it is unsafe
                    if (number - last).abs() < 1 || (number - last).abs() > 3 {
                        safe = false;
                        break; // it is unsafe
                    }

                    // Checking to see if it is still increasing and decreasing
                    if number > last {
                        decreasing = false;
                    }
                    if number < last {
                        increasing = false;
                    }

                    // If it is neither only increasing or decreasing, then it is unsafe
                    if !increasing && !decreasing {
                        safe = false;
                        break;
                    }

                    // Updating the last value
                    last = number;
                }

                // We can update count and then exit the loop if it is safe still
                if safe {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("Your answer is {}", count)
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Two\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
