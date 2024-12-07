// I really should have looked into oop (yes I know the weirdness in Rust with oop)¯\_(ツ)_/¯

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The board, but as a vector of vectors instead of just a vector of strings
    let mut board: Vec<Vec<&str>> = data
        .clone()
        .into_iter()
        .map(|x| x.split("").collect::<Vec<&str>>())
        .collect();

    // The direction that the guard is facing (upwards right now)
    let mut dir: (i8, i8) = (-1, 0);

    // The location of the guard (starting in the top left)
    let mut pos: (usize, usize) = (0, 0);

    // Calculating the bounds
    let bounds: (usize, usize) = (board.len() - 1, board[0].len() - 1);

    // Finding the location
    for (i, row) in board.iter().enumerate() {
        if row.iter().position(|x| *x == "^") != None {
            pos = (i, row.iter().position(|x| *x == "^").unwrap());
        }
    }

    // Saving the starting location
    let start = pos;

    // The main thing that comes to mind is just trying all of them...
    // The counter variable
    let mut total = 0;

    // I'm going to see how long just looping through every single place will take...
    for (x, row) in board.clone().iter().enumerate() {
        for (y, character) in row.iter().enumerate() {
            // Skipping if it already is an obstacle
            if *character != "." {
                continue;
            }

            // Changing this one to an obstacle
            board[x][y] = "#";

            // If it gets above 10000, I think we're probably good
            let mut counter = 0;

            // The little loopy thing
            while !(pos.0 == 0 && dir.0 == -1)
                && !(pos.1 == 0 && dir.1 == -1)
                && !(pos.0 == bounds.0 && dir.0 == 1)
                && !(pos.1 == bounds.1 && dir.1 == 1)
            {
                // Counting this iteration
                counter += 1;

                // Checking to see if she can move
                if board[add(pos.0, dir.0)][add(pos.1, dir.1)] != "#" {
                    pos = move_guard(&mut board, &pos, &dir);
                } else if board[add(pos.0, dir.0)][add(pos.1, dir.1)] == "#" {
                    dir = rotate(&dir);
                } else {
                    // We've reached the end (though this should never get called)
                    break;
                }

                // Making sure it isn't an infinite loop
                if counter > 500000 {
                    break;
                }
            }

            if counter > 500000 {
                // We found one!
                total += 1;
                println!("Found one!");
            }

            // Resetting everything back to how it was
            board = data
                .clone()
                .into_iter()
                .map(|x| x.split("").collect::<Vec<&str>>())
                .collect();

            // Fixing the starting location
            dir = (-1, 0);
            pos = start;
        }
    }

    println!("We can use {} distinct locations.", total);
}

// A function to turn her right
fn rotate(dir: &(i8, i8)) -> (i8, i8) {
    // Straight up mathing it
    // and by it, haha, lets jusr say,
    // the direction
    (dir.0 * 0 - dir.1 * -1, dir.0 * -1 + dir.1 * 0)
    // And yes I know that there is multiplication by zero, this just matches the rotation matrix
}

// A function to move her one in a direction
fn move_guard(board: &mut Vec<Vec<&str>>, pos: &(usize, usize), dir: &(i8, i8)) -> (usize, usize) {
    // Moving the character
    board[pos.0][pos.1] = "X";
    let new_pos = (add(pos.0, dir.0), add(pos.1, dir.1));
    board[new_pos.0][new_pos.1] = match dir {
        (0, 1) => ">",
        (1, 0) => "v",
        (0, -1) => "<",
        (-1, 0) => "^",
        _ => "*",
    };
    new_pos
}

// A function to help subtract from the position because it is usize
fn add(u: usize, i: i8) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u8 as usize
    } else {
        u + i as usize
    }
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Six\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
