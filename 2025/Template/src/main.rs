mod file_reader;
mod board;
use crate::file_reader::*;
use crate::board::*;
use std::time::Instant;


const FIRST_HALF_ANSWER: Option<u128> = Some(42); // Set your actual answer
const SECOND_HALF_ANSWER: Option<u128> = Some(123);

fn main() {
    let start: Instant = Instant::now();
    print!("First half answer: {}\n", first_half(false));
    let duration_first = start.elapsed();
    print!("Second half answer: {}\n", second_half(false));
    let duration_second = start.elapsed();

    println!("\n\nFirst half took: {:?}", duration_first);
    println!("Second half took: {:?}\n", duration_second);

    // Printing out what happened with the tests
    println!("First half {}", {if first_half_check() {"passing!"} else {"FAILING"}});
    println!("Second half {}", {if second_half_check() {"passing!"} else {"FAILING"}});
    
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
fn first_half_check() -> bool {
    let answer: Option<u128> = FIRST_HALF_ANSWER;
    match answer {
        None => panic!("Not yet added"),
        Some(a) => a == first_half(true),
    }
}
fn second_half_check() -> bool {
    let answer: Option<u128> = SECOND_HALF_ANSWER;
    match answer {
        None => false,
        Some(a) => a == second_half(true),
    }
}