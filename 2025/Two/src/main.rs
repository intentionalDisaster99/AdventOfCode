mod file_reader;
use crate::file_reader::{ get_lines, read_contents, read_csv };


fn main() {

    println!("First half answer: ");
    first_half(true);
    println!("Second half answer: ");
    second_half(false);

}

fn first_half(test: bool) {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = read_csv(&raw_data);

    let mut total: u128 = 0;

    for range in data {
        total += check_range(range.split('-').map(|x| x.to_string()).collect::<Vec<String>>()[0].parse::<u128>().expect("Bad input on parse"),range.split('-').map(|x| x.to_string()).collect::<Vec<String>>()[1].parse::<u128>().expect("Bad input on parse"));
    }

    println!("The sum of all of the bad IDs is {}", total);

}

// Checks the range for repeating characters
fn check_range(low: u128, high: u128) -> u128 {

    let mut sum: u128 = 0;

    for working_id in low..=high {

        let working_string = working_id.to_string();

        // If it has an odd number of characters, it can't really be invalid
        if working_string.len() % 2 != 0 { continue; }

        // We just need to split it down the middle to see if it is the same on both sides
        if working_string.ends_with(&working_string[0..working_string.len()/2]) {
            sum += working_id;
        } 

    }

    sum

}



fn second_half(test: bool) {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = read_csv(&raw_data);

    let mut total: u128 = 0;

    for range in data {
        total += check_range_thoroughly(range.split('-').map(|x| x.to_string()).collect::<Vec<String>>()[0].parse::<u128>().expect("Bad input on parse"),range.split('-').map(|x| x.to_string()).collect::<Vec<String>>()[1].parse::<u128>().expect("Bad input on parse"));
    }

    if test { assert_eq!(4174379265, total); }

    println!("The sum of all of the bad IDs is {}", total);
}


// Checks the range for repeating characters
fn check_range_thoroughly(low: u128, high: u128) -> u128 {

    let mut sum: u128 = 0;

    for working_id in low..=high {
        if check_word(&working_id.to_string()) { sum += working_id; }
    }

    sum

}


fn check_word(word: &String) -> bool {

    // We need to check each possible length for the word
    'main_loop: for length in 1..word.len()/2+1 {

        // It only works if the subword length is a divisor of the length of the word
        if word.len() % length != 0 { continue; }

        let subword = &word[0..length];

        // Now we just need to see if we can find it again at every instance after
        for index in (length..word.len()).step_by(length) {

            if subword != &word[index..index+length] {
                continue 'main_loop;
            }


        }

        return true;

    }

    false
}