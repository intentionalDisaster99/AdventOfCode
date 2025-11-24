fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // Now we need to split the data into two vectors, the left vector and the right vector

    // A vector for each side
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    // Populating the vectors on each side
    for line in data.iter() {
        left.push(line[..5].parse::<i64>().unwrap());
        right.push(line[8..].parse::<i64>().unwrap());
    }

    // Sorting first so that they match
    left.sort();
    right.sort();

    // The answer we want
    let mut sum: i64 = 0;

    // Now to compare the left and right side to see how much more the right is
    for (i, left_number) in left.iter().enumerate() {
        sum += (right[i] - left_number).abs();
    }
    println!("Your answer is {}", sum)
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\One\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
