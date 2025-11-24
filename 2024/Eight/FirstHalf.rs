fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<Vec<char>> = get_words(&raw_data);

    // This will be a copy of the data, but where we can put markers on it
    let mut board: Vec<Vec<char>> = get_words(&raw_data);

    // Finding each of the antennas
    for (y, row) in data.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            // If it is a thingy, then we're gold
            if *character == '.' {
                continue;
            }

            // Now we need to find the others
            let others: Vec<(usize, usize)> = get_others(&data, &x, &y, &character);

            // Now that we have the others, we can just find the distance and update them in the board
            update_board(&mut board, (x, y), others);
        }
    }

    board.iter().for_each(|row| {
        row.iter().for_each(|x| {
            print!("{}", x);
        });
        println!("");
    });

    println!("There were {} antinodes in the map", count(board));
}

// Counts the hashtags in the board
fn count(board: Vec<Vec<char>>) -> u64 {
    let mut count: u64 = 0;

    for row in board.iter() {
        for character in row.iter() {
            if *character == '#' {
                count += 1;
            }
        }
    }
    count
}

// A function to update data to have the locations of the antinodes
fn update_board(board: &mut Vec<Vec<char>>, center: (usize, usize), others: Vec<(usize, usize)>) {
    for (x, y) in others.iter() {
        // We want to put a point the distance away
        let difference = (
            ((center.0 as isize) - (*x as isize)),
            ((center.1 as isize) - (*y as isize)),
        );
        let mut new_pos: (isize, isize) = (
            (center.0 as isize) + difference.0,
            (center.1 as isize) + difference.1,
        );

        // Trying to add it
        if new_pos.1 >= 0
            && new_pos.1 < (board.len() as isize)
            && new_pos.0 >= 0
            && new_pos.0 < (board[0].len() as isize)
        {
            board[new_pos.1 as usize][new_pos.0 as usize] = '#';
        }

        // There are actually two
        new_pos = ((*x as isize) - difference.0, (*y as isize) - difference.1);

        // Trying to add it
        if new_pos.1 >= 0
            && new_pos.1 < (board.len() as isize)
            && new_pos.0 >= 0
            && new_pos.0 < (board[0].len() as isize)
        {
            board[new_pos.1 as usize][new_pos.0 as usize] = '#';
        }
    }
}

// A function that gets the locations of all of the other ones
fn get_others(
    board: &Vec<Vec<char>>,
    x1: &usize,
    y1: &usize,
    target: &char,
) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();

    for (y, row) in board.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if *x1 == x && *y1 == y {
                continue;
            }
            if character == target {
                output.push((x, y));
            }
        }
    }
    output
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Eight\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
