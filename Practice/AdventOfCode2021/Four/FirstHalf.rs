fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let mut data: Vec<&str> = get_words(&raw_data);

    // Now I want to pull out the first line
    let numbers: Vec<i32> = data
        .remove(0)
        .trim()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    // Removing the extra space so that I don't mess it up because of it
    data.remove(0);

    // Splitting it into boards
    let mut boards: Vec<Vec<Vec<i32>>> = data
        .split(|line| line.trim().is_empty()) // Splitting by blank lines to get into a Vec of Vectors
        .filter(|chunk| !chunk.is_empty()) // Remove any excess blank lines
        .map(|chunk| {
            chunk
                .iter() // Iterate over the lines in the chunk, which is just the boards
                .map(|line| {
                    line.split_whitespace() // Split each line in the board into words
                        .map(|num| num.parse::<i32>().unwrap()) // Parse the words into numbers
                        .collect::<Vec<i32>>() // Collect numbers into a row
                })
                .collect::<Vec<Vec<i32>>>() // Collect rows into a board
        })
        .collect();

    // Looping to update the boards for the numbers at the top
    for num in numbers.iter() {
        // Looping for each of the boards
        for (i, board) in boards.iter_mut().enumerate() {
            // Getting rid of the numbers that match
            update_board(board, *num);

            // Checking this board to see if we won
            if check_board(board) {
                // This is the winning board
                println!("{} got bingo!", i);

                println!("{:?}", board);

                // We need now to find the actual answer
                println!("The answer we got is {}", get_answer(&board, *num));

                return; // Finished
            }
        }
    }
}

// A function that gets rid of the numbers that have been called in a board
fn update_board(board: &mut Vec<Vec<i32>>, token: i32) {
    // Looping through the rows
    for row in board {
        // Checking the number against the inputted token
        for num in row {
            if *num == token {
                // Getting rid of the number if it is there
                *num = -1;
            }
        }
    }
}

fn check_board(board: &Vec<Vec<i32>>) -> bool {
    // Here is a little array that holds bools so that I can tell if a vertical one works
    let mut vert: [bool; 5] = [true; 5];

    // Looping
    for row in board.iter() {
        // Flag will be turned off if any of the numbers aren't -1
        let mut worked: bool = true;

        // Looping for each number in the row of the board
        for (i, &num) in row.iter().enumerate() {
            if num != -1 {
                worked = false;
                vert[i] = false;
            }
        }
        // Returning true if this horizontal one worked
        if worked {
            return true;
        }
    }

    // If any of the things in the vert array are stull true, then it didn't work
    vert.contains(&true)
}

fn get_answer(board: &Vec<Vec<i32>>, multiplier: i32) -> i32 {
    let sum: i32 = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&num| num != -1) // Exclude marked numbers
        .sum();
    sum * multiplier
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\Practice\\AdventOfCode2021\\Four\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
