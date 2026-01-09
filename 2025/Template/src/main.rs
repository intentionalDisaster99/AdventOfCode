mod file_reader;
mod utils;
use crate::file_reader::*;
#[allow(unused_imports)]
use crate::utils::*;
use std::time::Instant;
use colored::Colorize;


const FIRST_HALF_ANSWER: Option<u128> = Some(0);
const SECOND_HALF_ANSWER: Option<u128> = Some(33);

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

    let mut matrix = Matrix{num_cols: 3, num_rows: 2, data: vec![1.0, 5.0, 1.0, 2.0, 11.0, 5.0]};
    println!("Initial:\n{}", matrix);

    matrix.transpose();
    // matrix.add_rows(0, 1, -0.3333);
    println!("Calculated:\n{}", matrix);

    let ans = Matrix::from_vectors(&vec![
        vec![1.0, 0.0, -14.0],
        vec![0.0, 1.0, 3.0]
    ]);
    println!("Should be:\n{}", ans);

    0

}

fn second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let mut data: Vec<String> = get_lines(&raw_data);

    let mut out: u128 = 0;

    // Parsing the data
    let mut answers: Vec<Vec<usize>> = vec![];
    let mut buttons: Vec<Vec<Vec<u8>>> = vec![];
    for row in data.iter_mut() {
        row.pop(); // Getting rid of the last }
        let split_row = row.split(" {").collect::<Vec<&str>>();
        answers.push(split_row[split_row.len() - 1]
            .to_string()
            .split(",")
            .map( |num|
                num.parse::<usize>().unwrap()
            ).collect::<Vec<usize>>()
        );

        let mut this_row_buttons: Vec<Vec<u8>> = vec![];

        for button_raw in row.split(" ").collect::<Vec<&str>>().iter().skip(1).rev().skip(1).rev() {
            // Now we have a (1, 1, , 2), so we need to split it again
            let mut button: Vec<u8> = vec![];
            for number in button_raw.chars().collect::<Vec<char>>().iter().skip(1).rev().skip(1).rev() {
                if *number == ',' { continue; } // I am well aware there is a better way, but this is faster that figuring it out
                button.push(number.to_string().parse::<u8>().expect("Bad unwrap of a button number"));
            }
            this_row_buttons.push(button);
        }

        buttons.push(this_row_buttons);

    }

    // Now we need to solve each system of equations and all the solutions to the output
    for i in 0..answers.len() {

        let mut matrix = {

            let mut matrix: Vec<Vec<f64>> = vec![];

            // Each activation set makes up one column
            for column in buttons[i].iter() {

                let mut column_to_add: Vec<f64> = vec![0.0; answers[i].len()];

                for element in column.iter() {
                    column_to_add[*element as usize] = 1.0;
                }
                matrix.push(column_to_add);

            }       
            println!("{:?}", matrix);
            Matrix::from_vectors(&matrix)

        };
        println!("{:#?}", matrix.transpose().reduced_row_echelon().solve(answers[i].iter().map(|a| *a as f64).collect()));

        println!("Rows: {}, Cols: {}", matrix.num_rows, matrix.num_cols);
        println!("{}", matrix);
        // out += solve_for_integers(matrix, );
    }


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