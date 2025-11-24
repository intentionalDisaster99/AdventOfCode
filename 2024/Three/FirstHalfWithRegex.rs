use regex::Regex;

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    // let mut data: Vec<&str> = get_words(&raw_data);

    // The format that we are looking for
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    // The ones that match
    let found = re.captures_iter(&raw_data);

    // The answer
    let mut answer: u128 = 0;

    // The number match regex
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    // Looping to get the stuff out of the regex found
    for element in found {
        // Extracting numbers
        if let Some(capture) = element.get(0) {
            let nums = num_re
                .find_iter(capture.as_str())
                .map(|num_match| num_match.as_str().parse::<u128>().expect("Invalid number"))
                .collect::<Vec<u128>>();

            if nums.len() == 2 {
                answer += nums[0] * nums[1];
            }
        }
    }

    // Showing the answer
    println!("Your answer is {}", answer);
    println!("It should be 184576302");
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Three\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
