mod file_reader;
mod board;
use crate::file_reader::*;
#[allow(unused_imports)]
use crate::board::*;
use std::time::Instant;
use colored::Colorize;
use petgraph::graph::*;
use petgraph::algo::simple_paths::all_simple_paths;
use petgraph::algo::has_path_connecting;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*; 
use petgraph::visit::EdgeRef; 
use petgraph::prelude::*;



const FIRST_HALF_ANSWER: Option<u128> = Some(5);
const SECOND_HALF_ANSWER: Option<u128> = Some(2);

fn main() {
    // Running and timing everything
    let start: Instant = Instant::now();
    let first_answer = first_half(false);
    let duration_first = start.elapsed();
    let second_answer = second_half(false);
    let duration_second = start.elapsed();

    // Printing
    println!("\n{:-^50}", " results ");
    println!("First half answer:  {}", first_answer.to_string().bold());
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
    let mut data: Vec<String> = get_lines(&raw_data);
    
    // This day is particularly annoying because the test inputs are different, so I have to parse them differently today
    if test {
        let mut drop = true;
        for (i, row) in data.clone().iter().enumerate().rev() {
            if *row == "SECOND HALF".to_string() {
                data.remove(i);
                drop = false;
                continue;
            }
            if *row == "FIRST HALF".to_string() {
                data.remove(i);
                continue;
            }
            if drop {
                data.remove(i);
                continue;
            }
        }
    }
    
    let mut devices = Graph::<&str, i32>::new();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    
    // Adding in the nodes
    for row in data.iter() {
        let node = row.split(":").collect::<Vec<&str>>()[0];
        nodes.insert(node, devices.add_node(node));
    }
    // Adding out manually
    nodes.insert("out", devices.add_node("out"));


    // Adding in the edges
    for row in data.iter() {
        let split_row = row.split(": ").collect::<Vec<&str>>();
        let node = nodes.get(split_row[0]).unwrap();

        for other in split_row[1].split(" ") {
            devices.add_edge(*node, *nodes.get(other).unwrap(), 1); 
        }
        
    }

    // This is so obviously a graph that I have decided it is time to learn a bit more about them
    // Today, that will be through the petgraph crate
    let all_paths: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
        &devices, 
        *nodes.get("you").expect("Can't find you!"), 
        *nodes.get("out").expect("Can't find out"),
        0,
        None
    ).collect(); 

    all_paths.len() as u128

}


// I was having trouble with the crate on the second part, so I went and wrote it myself
fn second_half(test: bool) -> u128 {

    let raw_data: String = read_contents(test);
    let mut data: Vec<String> = get_lines(&raw_data);

    // This day is particularly annoying because the test inputs are different, so I have to parse them differently today
    if test {
        let mut drop = false;
        for (i, row) in data.clone().iter().enumerate().rev() {
            if *row == "SECOND HALF".to_string() {
                drop = true;
            }
            if *row == "FIRST HALF".to_string() {
                data.remove(i);
                continue;
            }
            if drop {
                data.remove(i);
                continue;
            }
        }
    }

    // This time I will be loading into a hashmap for easy and fast access to each device with links based on the name (a &str)
    let mut hashbrown: HashMap::<String, Vec<&str>> = HashMap::new();
    for row in data.iter() {
        let split = row.split(": ").collect::<Vec<&str>>();
        hashbrown.insert(split[0].to_string(), split[1].split(" ").collect::<Vec<&str>>());
    }

    // Finding the paths that hit fft first
    let fft_first = count_paths(&mut hashbrown, "svr", "fft") * count_paths(&mut hashbrown, "fft", "dac") * count_paths(&mut hashbrown, "dac", "out");

    // Finding the paths that hit dac first
    let dac_first = count_paths(&mut hashbrown, "svr", "dac") * count_paths(&mut hashbrown, "dac", "fft") * count_paths(&mut hashbrown, "fft", "out");

    dac_first + fft_first
}

// A function to begin the recursive search so that we can memoize it
fn count_paths(hashbrown: &mut HashMap<String, Vec<&str>>, start: &str, end: &str) -> u128{
    let mut memo: HashMap<(String, String), u128> = HashMap::new();
    let mut out: u128 = 0;
    // Begin at every ending point 
    for device in hashbrown.clone().iter() {
        if *device.0 == start {
            out += count_paths_recursive(&hashbrown, start.to_string(), end.to_string(), &mut memo);
        }
    }
    out
}


fn count_paths_recursive(devices: &HashMap<String, Vec<&str>>, start: String, end: String, memo: &mut HashMap<(String, String), u128>) -> u128 {

    // Checking to see if we have been here before
    let key = (start.clone(), end.clone());
    if let Some(out) = memo.get(&key) {
        return *out;
    }

    // The base case: we have found the end 
    if start == end {
        return 1;
    }

    // Skipping out because it doesn't have anywhere else to go (it would have already been counted if it was where we wanted to go)
    if start == "out".to_string() {
        return 0;
    }

    let mut out = 0u128;
    
    // For every device that the start links to, we will recur
    for device in devices.get(&start).unwrap() {
        // println!("Made it to {} trying to get to {}", start, end);
        out += count_paths_recursive(devices, device.to_string(), end.clone(), memo);
    } 

    // Memoizing this result 
    memo.insert(key, out);
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
        Some(a) => { println!("Second half {}", {if a == second_half(true){"passing!"} else {"FAILING"}})},
    }
}