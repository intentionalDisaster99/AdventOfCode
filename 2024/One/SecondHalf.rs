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

    // The output
    let mut sum: i64 = 0;

    // Looping to find teh similarity
    for left_number in left.iter() {
        // Counting the occurrences
        let mut occurrences: i64 = 0;
        for right_number in right.iter() {
            if right_number == left_number {
                occurrences += 1;
            }
        }
        // Adding in the number times the number of times it comes in
        sum += left_number * occurrences;
    }

    println!("Your answer is {}", sum);
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
