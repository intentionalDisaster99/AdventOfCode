mod file_reader;
mod utils;
use crate::file_reader::*;
#[allow(unused_imports)]
use crate::utils::*;
use std::time::Instant;
use colored::Colorize;


const FIRST_HALF_ANSWER: Option<u128> = None;
const SECOND_HALF_ANSWER: Option<u128> = None;

fn main() {
    // Running and timing everything (Also printing out the stdout)
    println!("\n{:─^50}\n", " stdout ");    
    let mut start: Instant = Instant::now();
    let first_answer = first_half(false);
    let duration_first = start.elapsed();
    start = Instant::now();
    let second_answer = second_half(false);
    let duration_second = start.elapsed();

    // Printing
    println!("{:─^50}", " results ");
    print!("First half answer:  {}", first_answer.to_string().bold());
    if FIRST_HALF_ANSWER == None { println!("{}", " <--Warning, first half test is not initialized.".red()); }
    println!();
    print!("Second half answer: {}", second_answer.to_string().bold());
    if SECOND_HALF_ANSWER == None { println!("{}", " <--Warning, second half test is not initialized.".red()); }
    println!();

    // Showing the times
    println!("{:─^50}", " times ");
    println!("First half took: {:?}", duration_first);
    println!("Second half took: {:?}\n", duration_second);

    // Running the tests
    println!("\n{:─^50}\n", " testing stdout ");    
    start = Instant::now();
    let first_half_test_result = first_half_check(); 
    let first_half_test_time = start.elapsed();
    start = Instant::now();
    let second_half_test_result = second_half_check();
    let second_half_test_time = start.elapsed();


    println!("{:─^50}", " test results ");
    match first_half_test_result {
        (0, _, _) => { println!("{}", "First half test passed successfully!".green()); }
        (1, expected, calculated) => { println!("{}\t{} != {}", "First half test failed!".red(), expected, calculated) }
        (2, _, _) => { println!("{}", "First half test not created yet.".yellow()) }
        (_, _, _) => panic!("Bad error code from testing first half")
    }
    println!("First half test took {:?}", first_half_test_time);
    match second_half_test_result {
        (0, _, _) => { println!("{}", "Second half test passed successfully!".green()); }
        (1, expected, calculated) => { println!("{}\t{} != {}", "Second half test failed!".red(), expected, calculated) }
        (2, _, _) => { println!("{}", "Second half test not created yet.".yellow()) }
        (_, _, _) => panic!("Bad error code from testing second half")
    }
    println!("Second half test took {:?}", second_half_test_time);


    // A line at the bottom because I like how it looks
    print!("\n{:─^50}", "─");
}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);




    // Testing out the matrix struct

    let mut matrix = Matrix{num_cols: 4, num_rows: 3, data: vec![0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.0, 11.0]};
    println!("{}", matrix);

    matrix.add_rows(0, 1, -1.0);
    println!("{}", matrix);

    0

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


// These return the error code (0 is good, 1 is wrong, 2 is not created), then the expected, then the actual
fn first_half_check() -> (u8, u128, u128) {
    let answer: Option<u128> = FIRST_HALF_ANSWER;
    match answer {
        None => { (2, 0, 0) },
        Some(a) => {
            let result = first_half(true);
            if a == result {
                    (0, a, result) 
                } else { 
                    (1, a, result)
                } 
            },
    }
}
fn second_half_check() -> (u8, u128, u128) {
    let answer: Option<u128> = SECOND_HALF_ANSWER;
    match answer {
        None => { (2, 0, 0) },
        Some(a) => {
            let result = second_half(true);
            if a == result {
                    (0, a, result) 
                } else { 
                    (1, a, result)
                } 
            },
    }
}