fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // An array to count the bits
    let mut one_counts: [usize; 12] = [0; 12];

    // Looping through the data to check each one
    for (_, &item) in data.iter().enumerate() {
        // Looping for each item
        for (i, c) in item.trim().chars().enumerate() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    }

    println!("One Counts: {:?}", one_counts);

    // Variables to calculate the gamma and epsilon rate
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    // Looping through to add in the things
    for &count in one_counts.iter() {
        if count > data.len() / 2 {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }

    println!(
        "Gamma String: {}\nEpsilon String: {}",
        gamma_str, epsilon_str
    );

    let gamma = u32::from_str_radix(&gamma_str, 2).expect("Failed to parse gamma");
    let epsilon = u32::from_str_radix(&epsilon_str, 2).expect("Failed to parse epsilon");

    println!(
        "Gamma: {}\nEpsilon: {}\nAnswer: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    if let some(toPrint1) = data[831] {
        if let some(toPrint2) = data[856] {
            println!(
                "Found {}",
                u32::from_str_radix(toPrint1, 2).expect("no worky")
                    * u32::from_str_radix(toPrint2, 2).expect("no worky2"),
            );
        }
    }
   
}

use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\Three\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    // The output vector of all the things
    let mut output: Vec<&str> = Vec::new();

    // Converting the input to bytes
    let bytes = s.as_bytes();

    // The last index so that we can split it again there
    let mut last: usize = 0;

    // Looping through the bytes until we get to the point where we have a new line
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            // Pushing the values into our vector
            output.push(&s[last..i]);

            // Updating the last
            last = i;
        }
    }

    // Adding on the last value
    output.push(&s[last..]);

    output
}
