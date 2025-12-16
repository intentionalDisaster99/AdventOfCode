mod file_reader;
mod board;
use crate::file_reader::*;
#[allow(unused_imports)]
use crate::board::*;
use core::num;
use std::time::Instant;
use colored::Colorize;
use std::collections::VecDeque;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use nalgebra::{DMatrix, DVector};

const FIRST_HALF_ANSWER: Option<u128> = Some(7);
const SECOND_HALF_ANSWER: Option<u128> = Some(33);

// I apologize, I just left all of my random comments on here of me thinking
// I swear, I am not crazy

fn main() {
    // Running and timing everything
    let start: Instant = Instant::now();
    let first_answer = first_half(false);
    let duration_first = start.elapsed();
    let second_answer = second_half(false);
    let duration_second = start.elapsed() - duration_first;

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
    first_half_check();
    second_half_check(); 

    // A line at the bottom because I like how it looks
    print!("\n{:-^50}", "-");

}


fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);


    
    // My first thought is that this could be solved by a system of linear equations
    // Turns out there is a crate for that
    // Only thing is that we aren't looking for specific numbers in our answers, just whether they are even or odd
    // I might be able to use that actually

    // Parsing data
    let mut answers: Vec<Vec<bool>> = vec![];
    let mut buttons: Vec<Vec<Vec<u8>>> = vec![];
    for row in data.into_iter() {
        let split_row = row.split("] ").collect::<Vec<&str>>();
        answers.push(split_row[0]
            .chars()
            .collect::<Vec<char>>()
            .iter().skip(1) // Skipping the first one because it is a '['
            .map( |c| 
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Found a button that is not on or off"),
                }
            ).collect()
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


    // Now that we have the buttons, we need to make some of them even and some of them odd
    // I was thinking solving a system of linear equations would work, but because they aren't a specific number of times it doesn't make as much sense
    // My next thought is optimization problem, but there isn't exactly a clear function for it.

    // I will make a function to make iteration nicer
    let out = buttons.par_iter()
        .zip(answers.par_iter())
        .map(|(btns, ans)| find_minimum(btns, ans))
        .sum();

    // If you want a non-parallel solution
    // let mut out: u128 = 0;
    // for i in 0..buttons.len() {

    //     // println!("{}/{}", i, buttons.len());

    //     out += find_minimum(&buttons[i], &answers[i]);

    // }


    out

}



fn find_minimum(buttons: &Vec<Vec<u8>>, answer: &Vec<bool>) -> u128 {


    // So what I think we need to do if just brute force the fewest number of buttons needed to make each number either even or odd
    // If the light is off, then the number of presses must either result in an even number (including zero)
    // If the light is on, then it must be odd

    // Welp, screw elegeance, brute force it is

    // Hashset to make sure we don't revisit any things
    let mut combo_hash: HashSet::<Vec<usize>> = HashSet::new();

    // A deque that holds each combination tried so far representing a number of button presses
    // Each combination is a vector of the number of times each button has been pressed
    let mut combos: VecDeque<Vec<usize>> = VecDeque::new();

    // The starting value of all zeros
    combos.push_back(vec![0usize; buttons.len()]);

    // We iterate until we find a single combination that works, the first one will have the least 
    loop {

        let working_combo = combos.pop_front().unwrap();

        // println!("{:?}", working_combo);

        // Checking to see if this one works
        let mut is_answer: Vec<bool> = vec![false; answer.len()];
        for (button_index, number_of_presses) in working_combo.iter().enumerate() {
            
            // Incrementing for what changed because of the button
            for light_index in buttons[button_index].iter() {
                if number_of_presses % 2 == 1 {
                    is_answer[*light_index as usize] = !is_answer[*light_index as usize];
                }
            }

        }
        if is_answer == *answer {
            return (working_combo.iter().sum::<usize>()) as u128;
        }

        // Adding all of the next possibilities
        for (i, _) in working_combo.iter().enumerate() {
            let mut next: Vec<usize> = working_combo.clone();
            next[i] += 1;
            if next[i] > 1 { continue; } // Making sure we don't cancel out what we just did
            if combo_hash.insert(next.clone()) {
                combos.push_back(next);
            }
        }


    }
    
}



//* */ What you could do is just find the ratio of the smallest one as an integer to everything else and then continually add that ratio to a running count until all of them are integers
fn second_half(test: bool) -> u128 {

      // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let mut data: Vec<String> = get_lines(&raw_data);
    
    // My first thought is that this could be solved by a system of linear equations
    // Turns out there is a crate for that
    // Only thing is that we aren't looking for specific numbers in our answers, just whether they are even or odd
    // I might be able to use that actually

    // Parsing data
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


    // Now that we have the buttons, we need to make some of them even and some of them odd
    // I was thinking solving a system of linear equations would work, but because they aren't a specific number of times it doesn't make as much sense
    // My next thought is optimization problem, but there isn't exactly a clear function for it.

    // I will make a function to make iteration nicer
    let total_jobs = 171;
    let counter = AtomicUsize::new(0);

    let out = buttons.par_iter()
        .zip(answers.par_iter())
        .map(|(btns, ans)| {
            // Do the heavy lifting
            let result = find_minimum2(btns, ans);

            // atomic increment
            // fetch_add returns the PREVIOUS value, so add 1 for display
            let completed = counter.fetch_add(1, Ordering::Relaxed) + 1;
            
            // Print progress. 
            // Note: In parallel code, println! can sometimes interleave output lines, 
            // but for a simple counter it's usually readable enough.
            println!("{}/{}", completed, total_jobs);

            result
        })
        .sum();
    // let mut out: u128 = 0;
    // for i in 0..buttons.len() {

    //     println!("{}/{}", i, buttons.len());

    //     out += find_minimum2(&buttons[i], &answers[i]);

    // }

    out

}


fn find_minimum2(buttons: &Vec<Vec<u8>>, answer: &Vec<usize>) -> u128 {


    // So what I think we need to do if just brute force the fewest number of buttons needed to make each number either even or odd
    // If the light is off, then the number of presses must either result in an even number (including zero)
    // If the light is on, then it must be odd

    // Welp, screw elegeance, brute force it is

    // Hashset to make sure we don't revisit any things
    let mut combo_hash: HashSet::<Vec<usize>> = HashSet::new();

    // A deque that holds each combination tried so far representing a number of button presses
    // Each combination is a vector of the number of times each button has been pressed
    let mut combos: VecDeque<Vec<usize>> = VecDeque::new();

    // The starting value of all zeros
    combos.push_back(vec![0usize; buttons.len()]);

    // We iterate until we find a single combination that works, the first one will have the least 
    loop {

        let working_combo = combos.pop_front().unwrap();

        // println!("{:?}", working_combo);

        // Checking to see if this one works
        let mut is_answer: Vec<usize> = vec![0usize; answer.len()];
        for (button_index, number_of_presses) in working_combo.iter().enumerate() {
            
            // Incrementing for what changed because of the button
            for light_index in buttons[button_index].iter() {
                is_answer[*light_index as usize] += number_of_presses;
            }

        }
        if is_answer == *answer {
            return (working_combo.iter().sum::<usize>()) as u128;
        }

        // Adding all of the next possibilities
        for (i, _) in working_combo.iter().enumerate() {
            let mut next: Vec<usize> = working_combo.clone();
            next[i] += 1;
            if combo_hash.insert(next.clone()) {
                combos.push_back(next);
            }
        }


    }
    
}



// fn find_minimum2(buttons: &Vec<Vec<u8>>, answer: &Vec<usize>) -> u128 {

//     // While brute forcing is fun, this one I think I have to do linear algebra
//     // We got the Ax = b 
//     // With A is the matrix we make out of what each button does
//     // b is the answer we want
//     // x is the vector of the number of times we press each button

//     let num_rows = answer.len();
//     let num_cols = buttons.len();

//     // Making matrix A
//     let matrix = DMatrix::from_fn(num_rows, num_cols, |r, c| {
//         if buttons[c].contains(&(r as u8)) {
//             1.0
//         } else {
//             0.0
//         }
//     });
//     println!("{}", matrix);

//     // Making b
//     let target = DVector::from_iterator(num_rows, answer.iter().map(|&x| x as f64));

//     println!("{}", target);

//     // 3. Decompose and Solve with nalgebra
//     let svd = matrix.svd(true, true);
    
//     // We use a small epsilon (1e-9) to determine if a singular value is "zero"
//     match svd.solve(&target, 1e-9) {
//         Ok(solution) => {

//             let min_val = solution.iter()
//                  .filter(|&&x| x.abs() > 1e-6) // Ignore zeros
//                  .map(|x| x.abs())
//                  .fold(f64::INFINITY, f64::min);

//             // let integerized = integerize(solution
//             let scale_factor = 10000.0;
//             let rough_ints: Vec<u128> = solution.iter()
//                 .map(|x| (x * scale_factor).round() as u128)
//                 .collect();

//             let common_divisor = vector_gcd(&rough_ints);
//             let integer_solution: Vec<u128> = rough_ints.iter()
//                 .map(|x| x / common_divisor as u128)
//                 .collect();

//             println!("Integer Solution {:#?}", solution);

//             integer_solution.iter().sum::<u128>() as u128
//         },
//         Err(_) => 0 // System has no solution
//     }
        
// }
// fn gcd(a: u128, b: u128) -> u128 {
//     let mut a = a;
//     let mut b = b;
//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }
//     a
// }

// // Helper to calculate GCD of a whole Vector (to simplify a Null Vector)
// fn vector_gcd(vec: &Vec<u128>) -> u128 {
//     if vec.is_empty() { return 1; }
//     let mut result = vec[0];
//     for &val in vec.iter().skip(1) {
//         result = gcd(result, val);
//     }
//     result

// }

// // fn integerize(matrix: )



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
        Some(a) => { println!("First half {}", {if a == first_half(true){"passing!".green()} else {"FAILING".red()}})},
    }
}
fn second_half_check() {
    let answer: Option<u128> = SECOND_HALF_ANSWER;
    match answer {
        None => {println!("Second half test answer not given")},
        Some(a) => { println!("Second half {}", {if a == second_half(true){"passing!".green()} else {"FAILING".red()}})},
    }
}