// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

pub fn read_contents(test: bool) -> String {
    let file_path = if test { "input/Test.txt" } else { "input/Input.txt" };
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn get_lines(s: &String) -> Vec<String> {
    s.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}
