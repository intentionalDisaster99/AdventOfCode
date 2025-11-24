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
        .split(|line| line.trim().is_empty())
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect();

    // Looping to update the boards for the numbers at the top
    for num in numbers.iter() {
        let mut i = 0;
        while i < boards.len() {
            // Update the current board
            update_board(&mut boards[i], *num);

            // Check if this board wins
            if check_board(&boards[i]) {
                // If we're down to the last board and it wins
                if boards.len() == 1 {
                    println!("Got the last bingo!");
                    println!("{:?}", boards[i]);
                    println!("The answer we got is {}", get_answer(&boards[i], *num));
                    return;
                }

                // Remove winning board unless it's the last one
                if boards.len() > 1 {
                    boards.remove(i);
                    // Don't increment i since we removed a board
                    continue;
                }
            }
            i += 1;
        }
    }
}

fn check_board(board: &Vec<Vec<i32>>) -> bool {
    // Check rows
    for row in board {
        if row.iter().all(|&num| num == -1) {
            return true;
        }
    }

    // Check columns
    for col in 0..5 {
        if board.iter().all(|row| row[col] == -1) {
            return true;
        }
    }

    false
}

fn update_board(board: &mut Vec<Vec<i32>>, token: i32) {
    for row in board.iter_mut() {
        for num in row.iter_mut() {
            if *num == token {
                *num = -1;
            }
        }
    }
}

fn get_answer(board: &Vec<Vec<i32>>, multiplier: i32) -> i32 {
    let sum: i32 = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&num| num != -1)
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
