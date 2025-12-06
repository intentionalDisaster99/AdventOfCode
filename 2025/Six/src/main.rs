mod file_reader;
use std::ptr::eq;

use crate::file_reader::{ get_lines, read_contents, read_csv };

fn main() {

    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let operations: Vec<String> = data[data.len()-1].split_ascii_whitespace().map(String::from).collect();
    let numbers: Vec<Vec<String>> = data.iter().rev().skip(1).map(|line| line.split_ascii_whitespace().map(String::from).collect()).collect(); 

    let mut out: u128 = 0;

    for (i, operation) in operations.iter().enumerate() {

        let mut current: u128 = numbers[0][i].parse().expect("Bad unwrap of number");
        
        // Doing it for each row 
        for row in numbers.iter().skip(1) {
            if operation == "*" {
                current *= row[i].parse::<u128>().expect("Bad unwrap of number");
            } else {
                current += row[i].parse::<u128>().expect("Bad unwrap of number");
            }
        }

        out += current;

    }

    out

}


fn second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);
    let mut out: u128 = 0;

    let operations: Vec<String> = data[data.len()-1]
        .split_ascii_whitespace()
        .map(String::from)
        .collect();
    let mut problems: Vec<Vec<u128>> = vec![];
    
    let mut current_problem_nums: Vec<u128> = vec![];

    let max_len = data.iter().rev().skip(1).rev()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);
    for col_index in (0..max_len).rev() {
        let mut working_num_str = "".to_string();
        let mut has_digit = false;
        for row in data.iter().rev().skip(1).rev() {
            let c = row.chars().nth(col_index).unwrap_or(' ');
            if c.is_ascii_digit() {
                working_num_str.push(c);
                has_digit = true;
            }
        }
        if has_digit {
            let num = working_num_str.parse::<u128>().expect("Vertical Parse Fail");
            current_problem_nums.push(num);
        } else {
            if !current_problem_nums.is_empty() {
                problems.push(current_problem_nums);
                current_problem_nums = vec![];
            }
        }
    }
    if !current_problem_nums.is_empty() {
        problems.push(current_problem_nums);
    }
    for (i, nums) in problems.iter().enumerate() {
        let op_index = operations.len() - 1 - i;
        let operation = &operations[op_index];
        let mut current = nums[0];
        for val in nums.iter().skip(1) {
            if operation == "*" {
                current *= val;
            } else {
                current += val;
            }
        }
        out += current;
    }

    out
}


// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        let answer: Option<u128> = Some(4277556);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, first_half(true)),
        }
    }
    
    #[test]
    fn second_half_check() {
        let answer: Option<u128> = Some(3263827);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, second_half(true)),
        }
    }


}