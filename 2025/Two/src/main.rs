mod file_reader;
use crate::file_reader::{ get_lines, read_contents, read_csv };


fn main() {

    println!("First half answer: ");
    first_half();
    // println!("Second half answer: ");
    // second_half();

}

fn first_half() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(true);
    let data: Vec<String> = read_csv(&raw_data);

    let mut total: u32 = 0;

    for range in data {
        total += check_range(range.split('-').map(|x| x.to_string()).collect::<Vec<String>>()[0].parse::<u32>().expect("Bad input on parse"),range.split('-').map(|x| x.to_string()).collect::<Vec<String>>()[0].parse::<u32>().expect("Bad input on parse"));
    }

    println!("The sum of all of the bad IDs is {}", total);

}

// Checks the range for repeating characters
fn check_range(low: u32, high: u32) -> u32 {

    let mut sum: u32 = 0;

    for working_id in low..high {

        let working_string = working_id.to_string();

        // If it has an odd number of characters, it can't really be invalid
        if working_string.len() == 0 { continue; }

        // We just need to split it down the middle to see if it is the same on both sides
        if working_string.ends_with(&working_string[0..working_string.len()/2]) {
            sum += 1;
        }

    }

    sum

}



fn second_half() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    todo!();

}
