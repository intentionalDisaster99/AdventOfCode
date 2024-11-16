// Sonar Swap

// Reading from the input file
// use std::env;
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\Practice\\AdventOfCode2021\\One\\input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn main() {
    // Getting the data ans splitting it into a vector
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The total value, which is our output
    let mut sum = 1;

    // Looping once for each of the groups
    for i in 1..data.len() - 2 {
        // Adding something if it is higher
        if &data[i].trim().parse::<u32>().unwrap()
            + &data[i + 1].trim().parse::<u32>().unwrap()
            + &data[i - 1].trim().parse::<u32>().unwrap()
            < &data[i + 1].trim().parse::<u32>().unwrap()
                + &data[i + 2].trim().parse::<u32>().unwrap()
                + &data[i].trim().parse::<u32>().unwrap()
        {
            sum += 1;
        }
    }

    println!("{}", sum);
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

    output
}
