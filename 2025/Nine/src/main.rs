mod file_reader;
mod board;
use crate::file_reader::*;
use crate::board::*;
use std::time::Instant;
use std::collections::VecDeque;
use colored::Colorize;


const FIRST_HALF_ANSWER: Option<u128> = Some(50);
const SECOND_HALF_ANSWER: Option<u128> = Some(24);

fn main() {
    // Running and timing everything
    println!("\n{:-^50}", " results ");
    let start: Instant = Instant::now();
    print!("First half answer: {}\n", first_half(false).to_string().bold());
    let duration_first = start.elapsed();
    print!("Second half answer: {}\n", second_half(false).to_string().bold());
    let duration_second = start.elapsed();
    println!("The second half should be 1343576598");
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

    // Saving these for later
    let corners: Vec<Pos> = data.iter().map(|row| {
        let temp = row.split(",").collect::<Vec<&str>>();
        Pos::new(
            temp[0].parse::<usize>().unwrap(),
            temp[1].parse::<usize>().unwrap()
        )
    }).collect();

    // Compressing the coordinates
    // We collect every relevant X and Y coordinate. These are just the corners plus an extra on each side of the corner
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for c in &corners {
        xs.push(c.x);
        xs.push(c.x + 1);
        ys.push(c.y);
        ys.push(c.y + 1);
    }

    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    // Map real coords to grid indices
    let get_x_idx = |x: usize| xs.binary_search(&x).unwrap();
    let get_y_idx = |y: usize| ys.binary_search(&y).unwrap();

    // The compressed grid size
    // We have xs.len() - 1 intervals
    let width = xs.len() - 1;
    let height = ys.len() - 1;
    
    // Use u8 to save memory
    let mut grid = vec![0u8; width * height];

    // Connecting the corners
    for (i, this) in corners.iter().enumerate() {
        let other = &corners[(i + 1) % corners.len()];

        // Find the indices in our compressed grid
        let x1 = get_x_idx(this.x.min(other.x));
        let x2 = get_x_idx(this.x.max(other.x) + 1); // Exclusive end index
        let y1 = get_y_idx(this.y.min(other.y));
        let y2 = get_y_idx(this.y.max(other.y) + 1); // Exclusive end index

        if this.x == other.x {
            // Vertical Wall
            for y in y1..y2 {
                grid[y * width + x1] = 1;
            }
        } else {
            // Horizontal Wall
            for x in x1..x2 {
                grid[y1 * width + x] = 1;
            }
        }
    }

    // Finding the inside
    let mut start_node = Pos::new(0, 0);
    let mut found = false;

    // Assuming that we will have an inside at half the height
    let mid_y = height / 2;
    for x in 0..width-1 {
        let idx = mid_y * width + x;
        // If this is a wall, and the next spot is empty
        if grid[idx] == 1 && grid[idx+1] == 0 {
            start_node = Pos::new(x + 1, mid_y);
            found = true;
            break;
        }
    }
    
    // If the inside is not at half the height, we have to do this fun thing
    if !found {
        'search: for y in 0..height {
            for x in 0..width-1 {
                if grid[y*width+x] == 1 && grid[y*width+x+1] == 0 {
                    start_node = Pos::new(x + 1, y);
                    found = true;
                    break 'search;
                }
            }
        }
    }
    
    if !found { panic!("No inside found"); }

    // Flooding the thing
    flood_fill_scan_line(start_node, &mut grid, width, height);

    // Weighted prefix sum <-- this is apparently a thing from graphics programming? Anyways, it makes it faster to find an area
    // If I'm being honest, I don't fully understand this, but it works so there's that
    let mut sum_grid = vec![0u128; width * height];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let cell_w = (xs[x+1] - xs[x]) as u128;
            let cell_h = (ys[y+1] - ys[y]) as u128;
            let current_area = if grid[idx] == 1 { cell_w * cell_h } else { 0 };
            let left = if x > 0 { sum_grid[idx - 1] } else { 0 };
            let top = if y > 0 { sum_grid[(y - 1) * width + x] } else { 0 };
            let top_left = if x > 0 && y > 0 { sum_grid[(y - 1) * width + (x - 1)] } else { 0 };
            sum_grid[idx] = current_area + left + top - top_left;
        }
    }

    // Checking all of the rectangles we have
    let mut highest: u128 = 0;
    for (i, this) in corners.iter().enumerate() {
        for other in corners.iter().skip(i + 1) {
            // Again, I will have to look into these things more later but the pseudo code I followed works lol
            let min_rx = this.x.min(other.x);
            let max_rx = this.x.max(other.x);
            let min_ry = this.y.min(other.y);
            let max_ry = this.y.max(other.y);
            let w = (max_rx - min_rx + 1) as u128;
            let h = (max_ry - min_ry + 1) as u128;
            let target_area = w * h;
            if target_area <= highest { continue; }
            let x1 = get_x_idx(min_rx);
            let x2 = get_x_idx(max_rx + 1);
            let y1 = get_y_idx(min_ry);
            let y2 = get_y_idx(max_ry + 1);
            let rx = x2 - 1;
            let ry = y2 - 1;
            let bottom_right = sum_grid[ry * width + rx];
            let bottom_left  = if x1 > 0 { sum_grid[ry * width + (x1 - 1)] } else { 0 };
            let top_right    = if y1 > 0 { sum_grid[(y1 - 1) * width + rx] } else { 0 };
            let top_left     = if x1 > 0 && y1 > 0 { sum_grid[(y1 - 1) * width + (x1 - 1)] } else { 0 };
            let filled_area = bottom_right + top_left - bottom_left - top_right;
            if filled_area == target_area {
                highest = target_area;
            }
        }
    }

    highest
}

fn flood_fill_scan_line(start: Pos, grid: &mut Vec<u8>, width: usize, height: usize) {

    // If we start on something that is wrong, we can't really do this
    if grid[start.y * width + start.x] == 1 { return; }

    // Using a dequeue because it is slightly faster for a queue like this
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some(pos) = q.pop_front() {

        // Noting the location
        let mut x = pos.x;
        let y = pos.y;
        let row_idx = y * width;

        while x > 0 && grid[row_idx + x - 1] == 0 { x -= 1; }

        let mut span_above = false;
        let mut span_below = false;

        while x < width && grid[row_idx + x] == 0 {
            grid[row_idx + x] = 1;
            if y > 0 {
                if grid[(y - 1) * width + x] == 0 {
                    if !span_above { q.push_back(Pos::new(x, y - 1)); span_above = true; }
                } else { span_above = false; }
            }
            if y < height - 1 {
                if grid[(y + 1) * width + x] == 0 {
                    if !span_below { q.push_back(Pos::new(x, y + 1)); span_below = true; }
                } else { span_below = false; }
            }
            x += 1;
        }
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
        Some(a) => { println!("First half {}", {if a == first_half(true){"passing!".green()} else {"FAILING".bold().red()}})},
    }
}
fn second_half_check() {
    let answer: Option<u128> = SECOND_HALF_ANSWER;
    match answer {
        None => {println!("Second half test answer not given")},
        Some(a) => { println!("Second half {}", {if a == second_half(true){"passing!".green()} else {"FAILING".bold().red()}})},
    }
}