fn main() {
    // First, we have to read the data
    let raw_data: String = read_contents(false);
    let data_lines: Vec<Vec<char>> = get_words(&raw_data);

    // The score that we have
    let mut score = 0;

    // We need to get all of the zeros (trail heads)
    for (i, row) in data_lines.clone().iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == '0' {
                score += rate_trail_head(&data_lines, i, j);
            }
        }
    }

    println!("The total score is {}", score);
}

fn rate_trail_head(data: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    // The output
    let mut count: i32 = 0;

    // The working position
    let pos: (i32, i32) = (i as i32, j as i32);

    // This board (we edit it to make sure that we don't recount something)
    let mut this_data = data.clone();

    // Starting off moving in each direction
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        // START THE RECURRENCE
        let update = try_next(&mut this_data, pos.clone(), *dir, '0');

        count += update;
    }

    count
}

// A function to move a direction and try it
fn try_next(
    data: &mut Vec<Vec<char>>,
    initial_pos: (i32, i32),
    dir: (i32, i32),
    last: char,
) -> i32 {
    // If the next position is out of bounds, we return 0
    if initial_pos.0 + dir.0 < 0
        || initial_pos.0 + dir.0 >= data.len() as i32
        || initial_pos.1 + dir.1 < 0
        || initial_pos.1 + dir.1 >= data[0].len() as i32
    {
        return 0;
    }

    // The position we are now in
    let pos = (initial_pos.0 + dir.0, initial_pos.1 + dir.1);

    // The number we are looking for
    let this_num: char =
        char::from_digit((last.to_string().parse::<u32>().unwrap() + 1) as u32, 10).unwrap();

    // If it is anything other than last + 1, we return 0
    if data[(initial_pos.0 + dir.0) as usize][(initial_pos.1 + dir.1) as usize] != this_num {
        return 0;
    }

    // If this one is a nine, then we can return 1 because this one is over
    if data[(initial_pos.0 + dir.0) as usize][(initial_pos.1 + dir.1) as usize] == '9' {
        // I actually read the question wrong, but I think I can salvage it if I overwrite the location where the nine is so that this zero doesn't use this nine again
        data[pos.0 as usize][pos.1 as usize] = '.';

        return 1;
    }

    let mut output: i32 = 0;

    // Otherwise, we can recur for all the direction leading from here
    // I could ignore the way I came in, but it will just cut there and won't be too much overhead
    for next_dir in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        output += try_next(data, pos.clone(), *next_dir, this_num);
    }

    output
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents(test: bool) -> String {
    let mut file_path: &str = "Testing.txt";
    if !test {
        file_path = "Input.txt";
    }

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
