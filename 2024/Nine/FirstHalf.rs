fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data_lines: Vec<Vec<char>> = get_words(&raw_data);

    // Splitting by characters
    let data: Vec<char> = data_lines[0].clone();

    // Now we want to make a new vector that describes the file blocks with their ID numbers
    let mut file_blocks: Vec<String> = collapse_data(&data);

    // Now we need to compress the thing to make it smaller like they want
    squish(&mut file_blocks);

    // Now to count
    println!("Your answer is {}", checksum(file_blocks));
}

// Counting it up
fn checksum(input: Vec<String>) -> u64 {
    let mut output: u64 = 0;

    for (i, element) in input.iter().enumerate() {
        if *element == ".".to_string() {
            break;
        }
        output += element.parse::<u64>().unwrap() * (i as u64);
    }
    output
}

// Moves the data around to make it smaller
fn squish(blocks: &mut Vec<String>) {
    // The index of the first blank space
    let mut index = blocks.iter().position(|r| *r == ".".to_string()).unwrap();

    // We need to iterate in reverse
    for (i, _) in blocks.clone().iter().enumerate().rev() {
        // Breaking out if the index of the . is after this index
        if i <= index {
            break;
        }

        // Swapping the things
        blocks.swap(i, index);

        // This works but is so slow
        // index = blocks.iter().position(|r| *r == ".".to_string()).unwrap();

        // So I do this instead
        for new_index in index..blocks.len() {
            if blocks[new_index] == '.'.to_string() {
                index = new_index;
                break;
            }

            if new_index == blocks.len() - 1 {
                return; // We're done
            }
        }
    }
}

// Collapses data into a representation of the file blocks by their index
fn collapse_data(input: &Vec<char>) -> Vec<String> {
    let expanded = input.iter().enumerate().map(|(index, representation)| {
        // Here, we have to decode the number and add it that many times
        // These will all be their own vectors that we will have to collapse later

        let mut output: Vec<String> = Vec::new();
        for _ in 0..representation.to_string().parse::<usize>().unwrap() {
            if index % 2 == 0 {
                output.push((index / 2).to_string());
            } else {
                output.push(".".to_string());
            }
        }
        output
    });

    let mut output: Vec<String> = Vec::new();

    // Collapsing
    for block in expanded {
        output.append(&mut block.clone());
    }

    output
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Nine\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
