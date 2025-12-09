mod file_reader;
mod board;
use crate::file_reader::*;
use crate::board::*;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::VecDeque;

const FIRST_HALF_ANSWER: Option<u128> = Some(50);
const SECOND_HALF_ANSWER: Option<u128> = Some(24);

fn main() {
    // Running and timing everything
    println!("\n{:-^50}", " results ");
    let start: Instant = Instant::now();
    print!("First half answer: {}\n", first_half(false));
    let duration_first = start.elapsed();
    print!("Second half answer: {}\n\n", second_half(false));
    let duration_second = start.elapsed();

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

    
    // Making a vector of all of the positions we have
    let reds: Vec<Pos> = data.iter().map( |row| {
        let temp= row.split(",").collect::<Vec<&str>>();
        let x = temp[0].parse::<usize>().unwrap();   
        let y = temp[1].parse::<usize>().unwrap();
        Pos::new(x,y)
    }).collect::<Vec<Pos>>();

    // Now we want to find the ones that make the biggest rectangle, aka the ones that are furthest rectangularly away from each other
    let mut highest: u128 = 0;
    for (i, this) in reds.iter().enumerate() {
        for other in reds.iter().skip(i + 1) {
            let score = ((this.x as isize - other.x as isize).abs() + 1) * ((this.y as isize - other.y as isize).abs() + 1);
            if score as u128 > highest {
                highest = score as u128;
            }
        }
    }
    highest

}



fn second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);


    // Making a vector of all of the positions we have
    let mut carpets: Vec<Pos> = data.iter().map(|row| {
        let temp = row.split(",").collect::<Vec<&str>>();
        Pos::new(
            temp[0].parse::<usize>().unwrap(),
            temp[1].parse::<usize>().unwrap()
        )
    }).collect();

    // Remembering where the red tiles are for later
    let reds = carpets.clone();

    // Adding connections between each square
    println!("Adding connections");
    let corners = carpets.clone();
    for (i, this) in corners.iter().enumerate() {
        let other = corners[(i + 1) % corners.len()].clone();

        if this.x != other.x {
            let x_range: (usize, usize) = (std::cmp::min(this.x, other.x), std::cmp::max(this.x, other.x));
            for x in (x_range.0..x_range.1).skip(1) {
                carpets.push(Pos::new(x, this.y));
            }
        } else {
            let y_range: (usize, usize) = (std::cmp::min(this.y, other.y), std::cmp::max(this.y, other.y));
            for y in (y_range.0..y_range.1).skip(1) {
                carpets.push(Pos::new(this.x, y));
            }
        }
    }
    println!("Connections added.\nLooking for inside.");

    // We need to find a wall that is vertical, so basically just not where there is a corner
    let mut corner_ys: Vec<usize> = corners.iter().map(|c| c.y).collect();
    corner_ys.sort_unstable();
    corner_ys.dedup();
    let mut start_y = 0;
    let mut found_safe_row = false;
    for window in corner_ys.windows(2) {
        if window[1] > window[0] {
            found_safe_row = true;
            start_y = window[0] + 1;
            break;
        }
    }

    if !found_safe_row {
        // Fallback: This usually only happens in tiny 2x2 box maps
        panic!("No vertical segments found");
    }

    // Now that the order doesn't matter, I will convert to a hashmap
    let mut hashbrown: HashMap<Pos, bool> = HashMap::new();
    for carpet in carpets.iter() {
        hashbrown.insert(carpet.clone(), true);
    }

    // Now we need to look for a safe column
    let min_x = corners.iter().map(|p| p.x).min().unwrap_or(0);
    let max_x = corners.iter().map(|p| p.x).max().unwrap_or(0);

    let mut start = Pos::new(0, 0);
    let mut found_start = false;

    for x in min_x..=max_x {
        let p = Pos::new(x, start_y);
                if hashbrown.contains_key(&p) {
            start = Pos::new(x + 1, start_y);
            found_start = true;
            break;
        }
    }
    
    if !found_start {
        panic!("Could not find inside");
    }

    println!("Inside found.\nBeginning flood.");

    // Converting to a grid because it is slightly faster
    let width = carpets.iter().map(|p| p.x).max().unwrap_or(0) + 4;
    let height = carpets.iter().map(|p| p.y).max().unwrap_or(0) + 4;
    let mut grid = vec![0u32; width * height];

    // Mark the walls
    for wall in carpets.into_iter() {
        grid[wall.y * width + wall.x] = 1;
    }

    // Now to fill in inside carpets
    flood_fill_1d_vector(start, &mut grid, width, height);

    println!("Flood complete.\nSearching for answer.");

    // Create a 2D Prefix Sum grid (1-based indexing to avoid boundary checks) (my poor poor ram)
    // p_sum[y][x] stores the count of filled pixels in the rectangle from (0,0) to (x-1, y-1)
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let current_val = grid[idx]; // This is 0 or 1
            
            let left = if x > 0 { grid[idx - 1] } else { 0 };
            let top = if y > 0 { grid[(y - 1) * width + x] } else { 0 };
            let top_left = if x > 0 && y > 0 { grid[(y - 1) * width + (x - 1)] } else { 0 };

            // Overwrite the cell with the sum
            grid[idx] = current_val + left + top - top_left;
        }
    }

    // Now we just need to check to see which reds make the biggest rectangle
    let mut highest: u128 = 0;
    for (i, this) in reds.iter().enumerate() {
        for other in reds.iter().skip(i + 1) {
            let min_x = this.x.min(other.x);
            let max_x = this.x.max(other.x);
            let min_y = this.y.min(other.y);
            let max_y = this.y.max(other.y);

            let w = (max_x - min_x + 1) as u128;
            let h = (max_y - min_y + 1) as u128;

            if w * h <= highest { continue; }

            // Integral Image Lookup on Flat Grid
            // P(x,y) is at grid[y * width + x]
            let bottom_right = grid[max_y * width + max_x];
            let bottom_left  = if min_x > 0 { grid[max_y * width + (min_x - 1)] } else { 0 };
            let top_right    = if min_y > 0 { grid[(min_y - 1) * width + max_x] } else { 0 };
            let top_left     = if min_x > 0 && min_y > 0 { grid[(min_y - 1) * width + (min_x - 1)] } else { 0 };

            let actual_area = bottom_right - bottom_left - top_right + top_left;

            if actual_area as u128 == w * h {
                highest = w * h;
            }
        }
    }

    highest
}


fn memory_intensive_second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    // Making a vector of all of the positions we have
    let mut reds: Vec<Pos> = data.iter().map( |row| {
        let temp= row.split(",").collect::<Vec<&str>>();
        let x = temp[0].parse::<usize>().unwrap();   
        let y = temp[1].parse::<usize>().unwrap();
        Pos::new(x,y)
    }).collect::<Vec<Pos>>();

    // Finding the bounds of the board
    let mut height: usize = 0;
    let mut width: usize = 0;
    for red in reds.iter() {
        if red.x > width { width = red.x }
        if red.y > height { height = red.y }
    }

    // Making a board that I can play with
    let mut input: Vec<Vec<char>> = vec![];

    for y in 0..height+2 {
        input.push(vec!['.'; width + 2]);
    }
    let mut board = Board::new(input);

    
    // Adding the Red ones and the greens connecting them
    board.set(&reds[0], '#'); // Adding the first
    for (i, red) in reds.iter().enumerate().skip(1) {
        board.set(&red, '#');
        
        // Adding the connecting green to the one before
        if red.x != reds[i - 1].x { 
            // Iterating across all of the ones that we need to add
            for j in (std::cmp::min(red.x, reds[i-1].x) + 1)..(std::cmp::max(red.x, reds[i-1].x)) {
                board.set(&Pos::new(j, red.y), 'X');
            }
        } else {
            // Iterating across all of the ones that we need to add
            for j in (std::cmp::min(red.y, reds[i-1].y) + 1)..(std::cmp::max(red.y, reds[i-1].y)) {
                board.set(&Pos::new(red.x, j), 'X');
            }
        }
    }

    // Adding the connection between the last two
    if reds[reds.len() - 1].x != reds[0].x { 
        // Iterating across all of the ones that we need to add
        for j in (std::cmp::min(reds[reds.len() - 1].x, reds[0].x) + 1)..(std::cmp::max(reds[reds.len() - 1].x, reds[0].x)) {
            board.set(&Pos::new(j, reds[reds.len() - 1].y), 'X');
        }
    } else {
        // Iterating across all of the ones that we need to add
        for j in (std::cmp::min(reds[reds.len() - 1].y, reds[0].y) + 1)..(std::cmp::max(reds[reds.len() - 1].y, reds[0].y)) {
            board.set(&Pos::new(reds[reds.len() - 1].x, j), 'X');
        }
    }


    // Now filling in the board with greens
    println!("Starting flood");
    // flood_fill(&mut board);
    
    // Now that we have a filled space, we need to just check to see which ones work best
    let mut highest: u128 = 0;
    for (i, this) in reds.iter().enumerate() {
        println!("{}/{}", i, reds.len());
        'inner_loop: for other in reds.iter().skip(i + 1) {

            // Making sure this one works
            for y in (std::cmp::min(this.y, other.y) + 1)..(std::cmp::max(this.y, other.y)) {
                for x in (std::cmp::min(this.x, other.x) + 1)..(std::cmp::max(this.x, other.x)) {
                    if board.get(&Pos::new(x, y)) != 'X' {
                        let old = board.get(&Pos::new(x, y));
                        board.set(&Pos::new(x, y), 'O');
                        println!("Skipping because this one doesn't fit\n{}", board);
                        // board.set(&Pos::new(x, y), old);
                        continue 'inner_loop;
                    }
                }
            }

            let score = ((this.x as isize - other.x as isize).abs() + 1) * ((this.y as isize - other.y as isize).abs() + 1);
            if score as u128 > highest {
                highest = score as u128;
            }
        }
    }
    println!("Finished board:\n{}", board);

    highest    

}



fn flood_fill_1d_vector(start: Pos, grid: &mut Vec<u32>, width: usize, height: usize) {

    let mut q = VecDeque::new(); // VecDeque is faster than Vec for queues
    q.push_back(start.clone());

    // Marking the start
    if grid[start.y * width + start.x] == 0 {
        grid[start.y * width + start.x] = 1;
    }

    let mut count: u128 = 0;

   while let Some(p) = q.pop_front() {
        count +=1;
        // I learned what a closure is :)
        let mut check = |nx: usize, ny: usize| {
            let idx = ny * width + nx;
            if grid[idx] == 0 {
                grid[idx] = 1;
                q.push_back(Pos::new(nx, ny));
            }
        };

        if p.y > 0 { check(p.x, p.y - 1); }
        if p.y < height - 1 { check(p.x, p.y + 1); }
        if p.x > 0 { check(p.x - 1, p.y); }
        if p.x < width - 1 { check(p.x + 1, p.y); }
    }

    println!("Flood fill finished! Filled {} tiles.", count);

}

fn flood_fill_hashmap(start: Pos, board: &mut HashMap<Pos, bool>) {
    
    let mut to_check = vec![start];

    while let Some(working_pos) = to_check.pop() {        // println!("{}", to_check.len());

        // Checking each direction
        // 1. UP
        let up = Pos::new(working_pos.x, working_pos.y + 1);
        if !board.contains_key(&up) {
            board.insert(up.clone(), true);
            to_check.push(up);
        }

        // 2. RIGHT
        let right = Pos::new(working_pos.x + 1, working_pos.y);
        if !board.contains_key(&right) {
            board.insert(right.clone(), true);
            to_check.push(right);
        }

        // 3. LEFT (Check bounds!)
        if working_pos.x > 0 {
            let left = Pos::new(working_pos.x - 1, working_pos.y);
            if !board.contains_key(&left) {
                board.insert(left.clone(), true);
                to_check.push(left);
            }
        }

        // 4. DOWN (Check bounds!)
        if working_pos.y > 0 {
            let down = Pos::new(working_pos.x, working_pos.y - 1);
            if !board.contains_key(&down) {
                board.insert(down.clone(), true);
                to_check.push(down);
            }
        }

    }

}

fn flood_fill_board(board: &mut Board) {

    let mut to_check: Vec<Pos> = vec![];
    let mut checked: Vec<Pos> = vec![];

    // Finding the starting point
    for (y, row) in board.board.clone().iter().enumerate() {
        let mut inside: bool = false;
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                // We just don't want to worry about it if it is a corner
                break;
            }
            if *c == 'X' {
                inside = !inside;
                continue;
            }
            if inside {
                to_check.push(Pos::new(x, y));
                break;
            }
        }
        if to_check.len() != 0 { break; }
    }

    if to_check.len() == 0 { panic!("Could not find inside") }

    while let Some(pos) = to_check.pop() {
        // If it's already filled  or a wall, skip it.
        let current = board.get(&pos);
        if current == 'X' || current == '#' { 
            continue;
        }

        // Mark it IMMEDIATELY so we don't add it again later
        board.set(&pos, 'X');

        // --- 3. Neighbors ---
        // Right
        if pos.x + 1 < board.board[0].len() { to_check.push(Pos::new(pos.x + 1, pos.y)); }
        // Down
        if pos.y + 1 < board.board.len() { to_check.push(Pos::new(pos.x, pos.y + 1)); }
        // Left
        if pos.x > 0 { to_check.push(Pos::new(pos.x - 1, pos.y)); }
        // Up
        if pos.y > 0 { to_check.push(Pos::new(pos.x, pos.y - 1)); }
    }
    

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