mod file_reader;
use crate::file_reader::{ read_contents, get_lines };


fn main() {

    println!("First half answer: ");
    first_half();
    // println!("Second half answer: ");
    // second_half();

}

fn first_half() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

}



fn second_half() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    todo!();

}
