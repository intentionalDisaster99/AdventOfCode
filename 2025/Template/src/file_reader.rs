#[path = "utils.rs"]
mod utils;
use utils::*;
// I actually have no idea where this file wants the files to be located. I haven't changed this from day to day and it only worked in different places each day ¯\_(ツ)_/¯

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

#[allow(dead_code)]
pub fn read_contents(test: bool) -> String {
    let file_path = if test { "input/Test.txt" } else { "input/Input.txt" };
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[allow(dead_code)]
pub fn get_lines(s: &String) -> Vec<String> {
    s.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}

#[allow(dead_code)]
pub fn read_csv(s: &String) -> Vec<String> {
    s.split(',').map(|x| x.to_string()).collect::<Vec<String>>()
}


// #[allow(dead_code)]
// pub fn get_board(input: Vec<String>) -> Board<> {
//     let mut data: Vec<Vec<char>> = vec![];
//     for row in input.iter() {
//         data.push(row.chars().collect::<Vec<char>>());
//     }
//     Board::new(data)
// }
