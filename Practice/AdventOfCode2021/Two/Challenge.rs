use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\Practice\\AdventOfCode2021\\Two\\Input.txt";

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

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The initial depth and position
    let mut pos: usize = 0;
    let mut depth: usize = 0;

    // Looping through the input
    for (_, &item) in data.iter().enumerate() {
        // Reading the data
        if item.contains("forward") {
            pos += item[8..].trim().parse::<usize>().unwrap();
        } else if item.contains("down") {
            depth += item[5..].trim().parse::<usize>().unwrap();
        } else {
            depth -= item[3..].trim().parse::<usize>().unwrap();
        }
    }

    // Printing out the data
    println!(
        "The amount of data read: {}\nPosition found: {}\nDepth found: {}\nProduct: {}",
        data.len(),
        pos,
        depth,
        pos * depth
    );
}
