mod file_reader;
mod board;
use crate::file_reader::*;
#[allow(unused_imports)]
use crate::board::*;
use std::time::Instant;
use colored::Colorize;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::collections::HashSet;

const FIRST_HALF_ANSWER: Option<u128> = Some(2);
const SECOND_HALF_ANSWER: Option<u128> = None;

fn main() {
    // Running and timing everything (Also printing out the stdout)
    println!("\n{:-^50}", " stdout ");    
    let mut start: Instant = Instant::now();
    let first_answer = first_half(false);
    let duration_first = start.elapsed();
    start = Instant::now();
    let second_answer = second_half(false);
    let duration_second = start.elapsed();

    // Printing
    println!("\n{:-^50}", " results ");
    print!("First half answer:  {}", first_answer.to_string().bold());
    if FIRST_HALF_ANSWER == None { println!("{}", " <--Warning, first half test is not initialized.".red()); }
    println!();
    print!("Second half answer: {}", second_answer.to_string().bold());
    if SECOND_HALF_ANSWER == None { println!("{}", " <--Warning, second half test is not initialized.".red()); }
    println!();

    // Showing the times
    println!("{:-^50}", " times ");
    println!("First half took: {:?}", duration_first);
    println!("Second half took: {:?}\n", duration_second);

    // Running the tests
    println!("\n{:-^50}", " testing stdout ");    
    start = Instant::now();
    let first_half_test_result = first_half_check(); 
    let first_half_test_time = start.elapsed();
    start = Instant::now();
    let second_half_test_result = second_half_check();
    let second_half_test_time = start.elapsed();


    println!("{:-^50}", " test results ");
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
    print!("\n{:-^50}", "-");
}

fn first_half(test: bool) -> u128 {
    // Reading data
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let mut presents: Vec::<Present> = vec![];
    let mut regions: Vec<Area> = vec![];
    let mut present = Present {index: 0, area: 0, shape: [[2; 3]; 3]};

    // Parsing
    for row in data.iter() {
        match row.len() {
            0 => {
                presents.push(present);
                present = Present {index: 0, area: 0, shape: [[2; 3]; 3]};
            }
            2 => {
                present.index = row.chars().collect::<Vec<char>>()[0].to_string().parse::<usize>().expect("Got bad index");
            },
            3 => {
                for shape_row in present.shape.iter_mut() {
                    if shape_row[0] == 2 {
                        *shape_row = row.chars().map(|e| if e == '.' {0} else {1}).collect::<Vec<u8>>().try_into().unwrap();
                        break;
                    }
                }
                present.area += row.chars().filter(|x| *x == '#').count()
            },
            _ => {
                let parts: Vec<&str> = row.split(": ").collect();
                let dim_parts: Vec<usize> = parts[0].split('x').map(|n| n.parse().unwrap()).collect();
                let width = dim_parts[0];
                let height = dim_parts[1];
                let present_numbers: [usize; 6] = parts[1].split(' ').map(|n| n.parse().unwrap()).collect::<Vec<usize>>().try_into().unwrap();
                regions.push( Area { height, width, present_numbers } );
            }
        }
    }

    // Parallel Processing
    let progress_counter = Arc::new(AtomicUsize::new(0));
    let total_regions = regions.len();

    // Precalculate all rotations/flips once
    let all_variants: Vec<Vec<[[u8; 3]; 3]>> = presents
            .iter()
            .map(|p| p.get_all_orientations())
            .collect();

    regions.par_iter().map(|region| {
        // Quick check: Does the region have enough area for all presents?
        let total_present_area: usize = region.present_numbers.iter()
            .enumerate()
            .map(|(i, &count)| count * presents[i].area)
            .sum();

        if region.width * region.height < total_present_area {
            let current = progress_counter.fetch_add(1, Ordering::Relaxed);
            println!("Progress: {}/{} (Skipped - Area too small)", current + 1, total_regions);
            return 0;
        }
        
        // Heavy Lifting
        let out = check_region(region, &all_variants);
        
        let current = progress_counter.fetch_add(1, Ordering::Relaxed);
        println!("Progress: {}/{}", current + 1, total_regions);

        out
        
    }).sum() 
}

fn check_region(region: &Area, present_variations: &Vec<Vec<[[u8; 3]; 3]>>) -> u128 {
    // Initialize empty board (0)
    let mut board: Board<u8> = Board::new(region.width, region.height, 0);
    
    // Calculate initial free space
    let initial_free_space = region.width * region.height;

    // Start recursion at (0,0)
    if add_presents(region, present_variations, &mut board, 0, 0, 0, initial_free_space) {
        return 1
    }
    0
}

// The Optimized Recursive Solver
fn add_presents(
    region: &Area, 
    present_variations: &Vec<Vec<[[u8; 3]; 3]>>, 
    board: &mut Board<u8>, 
    present_index: usize, 
    start_x: usize,
    start_y: usize,
    free_space: usize
) -> bool {

    // 1. Guard Clause for small boards (prevents panic on bounds)
    if region.width < 3 || region.height < 3 { return false; }

    // 2. Base Case: Success
    let total_presents: usize = region.present_numbers.iter().sum();
    if present_index >= total_presents {
        return true;
    }

    // 3. Identify Present Type & Calculate Remaining Area Needed (Pruning)
    let mut present_type = 0;
    let mut count_so_far = 0;
    let mut current_type_limit = 0;
    let mut remaining_area_needed = 0;

    for (type_idx, &count) in region.present_numbers.iter().enumerate() {
        let end_idx = count_so_far + count;
        
        // Identify type of current index
        if present_index < end_idx && current_type_limit == 0 {
            present_type = type_idx;
            current_type_limit = end_idx; // The index where this type STOPS
        }

        // Calculate area needed for future presents
        if end_idx > present_index {
            let count_remaining = end_idx - std::cmp::max(present_index, count_so_far);
            // Get area of one instance of this present type
            // (We just peek at the first variant, area is constant)
            let shape_area = present_variations[type_idx][0].iter().flatten().filter(|&&x| x == 1).count();
            remaining_area_needed += count_remaining * shape_area;
        }
        
        count_so_far += count;
    }

    // PRUNING: Impossible to fit remaining items in available space
    if free_space < remaining_area_needed {
        return false;
    }

    // 4. Setup Iteration
    let variations = &present_variations[present_type];
    let this_shape_area = variations[0].iter().flatten().filter(|&&x| x == 1).count();
    
    // Check if the NEXT present is the same type.
    // If so, we enforce order (don't check backwards).
    let next_is_same_type = (present_index + 1) < current_type_limit;

    // 5. Iterate Board
    for y in start_y..=(region.height - 3) {
        let current_start_x = if y == start_y { start_x } else { 0 };
        
        for x in current_start_x..=(region.width - 3) {

            for variant in variations {
                let mut fits = true;

                // CHECK FIT
                for i in 0..9 {
                    let dy = i / 3;
                    let dx = i % 3;
                    
                    if variant[dy][dx] == 1 {
                        // Use consistent (x+dx, y+dy)
                        if board.get(&Pos::new(x + dx, y + dy)) == 1 {
                            fits = false;
                            break;
                        }
                    }
                }
                
                if fits {
                    // SET
                    for i in 0..9 {
                        let dy = i / 3;
                        let dx = i % 3;
                        if variant[dy][dx] == 1 {
                            board.set(&Pos::new(x + dx, y + dy), 1);
                        }
                    }

                    // DETERMINE NEXT START POS
                    let (next_x, next_y) = if next_is_same_type {
                        (x, y) // Optimize: Continue from here
                    } else {
                        (0, 0) // Reset: New type can go anywhere
                    };

                    // RECURSE
                    if add_presents(region, present_variations, board, present_index + 1, next_x, next_y, free_space - this_shape_area) {
                        return true;
                    }

                    // BACKTRACK
                    for i in 0..9 {
                        let dy = i / 3;
                        let dx = i % 3;
                        if variant[dy][dx] == 1 {
                            board.set(&Pos::new(x + dx, y + dy), 0);
                        }
                    }
                }
            }
        }
    } 

    false
}

// --- Structs ---

#[derive(Clone)]
struct Present {
    index: usize,
    area: usize,
    shape: [[u8; 3]; 3],
}

struct Area {
    width: usize,
    height: usize,
    present_numbers: [usize; 6],
}

impl Present {
    fn get_all_orientations(&self) -> Vec<[[u8; 3]; 3]> {
        let mut variants = HashSet::new();
        let mut current = self.shape;

        for _ in 0..2 { 
            for _ in 0..4 { 
                variants.insert(current);
                current = self.rotate_90(current);
            }
            current = self.flip(current);
        }
        variants.into_iter().collect()
    }

    fn rotate_90(&self, m: [[u8; 3]; 3]) -> [[u8; 3]; 3] {
        let mut next = [[0; 3]; 3];
        for r in 0..3 {
            for c in 0..3 {
                next[c][2 - r] = m[r][c];
            }
        }
        next
    }

    fn flip(&self, m: [[u8; 3]; 3]) -> [[u8; 3]; 3] {
        let mut next = [[0; 3]; 3];
        for r in 0..3 {
            for c in 0..3 {
                next[r][2 - c] = m[r][c];
            }
        }
        next
    }
}

fn second_half(test: bool) -> u128 {
    let raw_data: String = read_contents(test);
    let _data: Vec<String> = get_lines(&raw_data);
    0
}

// --- Tests ---

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