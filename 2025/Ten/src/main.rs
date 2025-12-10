mod file_reader;
mod board;
use crate::file_reader::*;
#[allow(unused_imports)]
use crate::board::*;
use std::time::Instant;
use colored::Colorize;


const FIRST_HALF_ANSWER: Option<u128> = None;
const SECOND_HALF_ANSWER: Option<u128> = None;

fn main() {
    // Running and timing everything
    let start: Instant = Instant::now();
    let first_answer = first_half(false);
    let duration_first = start.elapsed();
    let second_answer = second_half(false);
    let duration_second = start.elapsed();

    // Printing
    println!("\n{:-^50}", " results ");
    print!("First half answer:  {}", first_answer.to_string().bold());
    if FIRST_HALF_ANSWER == None { println!("{}", " <--Warning, first half test is not initialized.".red()); }
    print!("Second half answer: {}", second_answer.to_string().bold());
    if SECOND_HALF_ANSWER == None { println!("{}", " <--Warning, second half test is not initialized.".red()); }
    println!();

    // Showing the times
    println!("{:-^50}", " times ");
    println!("First half took: {:?}", duration_first);
    println!("Second half took: {:?}\n", duration_second);

    // Printing out what happened with the tests
    println!("{:-^50}", " tests ");
    second_half_check();
    first_half_check(); 

    // A line at the bottom because I like how it looks
    print!("\n{:-^50}", "-");

}


fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);
    let mut out: u128 = 0;

    out

}

fn second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let mut out: u128 = 0;

    out
}

// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn first_half_check() {
        let answer: Option<u128> = FIRST_HALF_ANSWER;
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, first_half(true)),
        }
    }
    
    #[test]
    pub fn second_half_check() {
        let answer: Option<u128> = SECOND_HALF_ANSWER;
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, second_half(true)),
        }
    }

}

// Duplicate functions so that we can run them normally 
fn first_half_check() {
    let answer: Option<u128> = FIRST_HALF_ANSWER;
    match answer {
        None => {println!("First half test answer not given")},
        Some(a) => { println!("First half {}", {if a == first_half(true){"passing!"} else {"FAILING"}})},
    }
}
fn second_half_check() {
    let answer: Option<u128> = SECOND_HALF_ANSWER;
    match answer {
        None => {println!("Second half test answer not given")},
        Some(a) => { println!("Second half {}", {if a == first_half(true){"passing!"} else {"FAILING"}})},
    }
}