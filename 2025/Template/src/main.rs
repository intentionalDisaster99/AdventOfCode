mod file_reader;
mod board;
use crate::file_reader::*;
use crate::board::*;
use std::time::Instant;


fn main() {
    let start: Instant = Instant::now();
    print!("First half answer: {}\n", first_half(false));
    let duration_first = start.elapsed();
    print!("Second half answer: {}\n", second_half(false));
    let duration_second = start.elapsed();

    println!("First half took: {:?}", duration_first);
    println!("Second half took: {:?}", duration_second);
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
    fn first_half_check() {
        let answer: Option<u128> = None;
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, first_half(true)),
        }
    }
    
    #[test]
    fn second_half_check() {
        let answer: Option<u128> = None;
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, second_half(true)),
        }
    }


}