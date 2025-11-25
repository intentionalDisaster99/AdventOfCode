

/*

This is a fun implementation of A*. Unfortunately, it only gives approximations due to how A* works. 
It was fun to build, but I will be switching to dijkstra's algorithm for my final answer

*/


use std::{cmp::Reverse, collections::HashMap}; 
use priority_queue::PriorityQueue;


fn main() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(true);
    let data: Vec<Vec<char>> = get_words(&raw_data);
    let board = make_board(data);

    println!("The smallest found path has a score of {}", a_star(board));
}


fn a_star(board: Vec<Vec<Node>>) -> usize {

    // Making priority queue <(x    , y    , dir      , cost ), score >
    let mut pq: PriorityQueue<(usize, usize, Direction, usize), Reverse<usize>> = PriorityQueue::new();

    // Making the hashmap     <(x    , y    , dir      ), cost > 
    let mut hashbrown: HashMap<(usize, usize, Direction), usize> = HashMap::new();

    // Finding the goal
    let goal: (usize, usize) = find_goal_location(&board);

    // Pushing the first location
    let start: (usize, usize) = find_starting_location(&board);
    pq.push(
        (start.0, start.1, Direction::EAST, 0),
        Reverse(((goal.0 as isize - start.0 as isize).abs() + (goal.1 as isize - start.1 as isize).abs()) as usize)
    );
    hashbrown.insert((start.0, start.1, Direction::EAST), 0);

    while let Some((current_state, Reverse(score))) = pq.pop() {

        // Checking to see if we have made it to the goal
        if board[current_state.1][current_state.0].marker == 'E' { return current_state.3; }

        // Trying to go forwards
        if let Some(new_pos) = check_forwards(&board, &current_state) {

            // Calculating what we know about where we are
            let new_cost = current_state.3 + 1;
            let new_score = new_cost + (((goal.0 as isize - new_pos.0 as isize).abs() + (goal.1 as isize - new_pos.1 as isize).abs()) as usize);
            
            // Checking to see if we have already been here in the hashmap
            if hashbrown.contains_key(&(new_pos.0, new_pos.1, current_state.2)) {
                // Replacing the old value if it is worse
                if let Some(old_cost) = hashbrown.get_mut(&(new_pos.0, new_pos.1, current_state.2)) {
                    if new_cost < *old_cost {
                        *old_cost = new_cost;

                        // Adding the updated node to the priority queue
                        pq.push((new_pos.0, new_pos.1, current_state.2, new_cost), Reverse(new_score));
                    }
                }
            } else {
                // Adding the new value
                hashbrown.insert((new_pos.0, new_pos.1, current_state.2), new_cost);

                // Adding to the priority queue
                pq.push((new_pos.0, new_pos.1, current_state.2, new_cost), Reverse(new_score));
            }

        }
        

        // Trying to go right
        let mut new_dir = current_state.2.turn_right();
        if hashbrown.contains_key(&(current_state.0, current_state.1, new_dir)) {
            // Replacing the old value if it is worse
            if let Some(old_cost) = hashbrown.get_mut(&(current_state.0, current_state.1, new_dir)) {
                if current_state.3 + 1000 < *old_cost {
                    *old_cost = current_state.3 + 1000;

                    // Adding the updated node to the priority queue
                    pq.push((current_state.0, current_state.1, new_dir, current_state.3 + 1000), Reverse(score + 1000));
                }
            }
        } else {
            // Adding the new value
            hashbrown.insert((current_state.0, current_state.1, new_dir), current_state.3 + 1000);

            // Adding to the priority queue
            pq.push((current_state.0, current_state.1, new_dir, current_state.3 + 1000), Reverse(score + 1000));
        }



        // Trying to go left
        new_dir = current_state.2.turn_left();
        if hashbrown.contains_key(&(current_state.0, current_state.1, new_dir)) {
            // Replacing the old value if it is worse
            if let Some(old_cost) = hashbrown.get_mut(&(current_state.0, current_state.1, new_dir)) {
                if current_state.3 + 1000 < *old_cost {
                    *old_cost = current_state.3 + 1000;

                    // Adding the updated node to the priority queue
                    pq.push((current_state.0, current_state.1, new_dir, current_state.3 + 1000), Reverse(score + 1000));
                }
            }
        } else {
            // Adding the new value
            hashbrown.insert((current_state.0, current_state.1, new_dir), current_state.3 + 1000);

            // Adding to the priority queue
            pq.push((current_state.0, current_state.1, new_dir, current_state.3 + 1000), Reverse(score + 1000));
        }

    }

   panic!("Could not find the path!!");

}




// #####################################
// ######### Helper Functions ##########
// #####################################
fn find_starting_location(board: &Vec<Vec<Node>>) -> (usize, usize) {
    for (y, data_row) in board.iter().enumerate() {
        for (x, &node) in data_row.iter().enumerate() {
            if node.clone().marker == 'S' {
                return (x, y);
            }
        }
    }
    panic!("Start not found!");
}


fn find_goal_location(board: &Vec<Vec<Node>>) -> (usize, usize) {
    for (y, data_row) in board.iter().enumerate() {
        for (x, &node) in data_row.iter().enumerate() {
            if node.clone().marker == 'E' {
                return (x, y);
            }
        }
    }
    panic!("Start not found!");
}


fn check_forwards(board: &Vec<Vec<Node>>, current_state: &(usize, usize, Direction, usize)) -> Option<(usize, usize)> {
    let out = match current_state.2 {
        Direction::NORTH => if current_state.1 > 0 {(current_state.0, current_state.1 - 1)} else { return None; },
        Direction::EAST => if current_state.0 < board[0].len() - 1 { (current_state.0 + 1, current_state.1) } else { return None; },
        Direction::SOUTH => if current_state. 1 < board.len() - 1 { (current_state.0, current_state.1 + 1) } else {return None;},
        Direction::WEST => if current_state.0 > 0 {(current_state.0 - 1, current_state.1)} else { return None; },
    };

    match board[out.1][out.0].marker {
        'E' => Some(out),
        '.' => Some(out),
        _ => None,
    }

    
} 


// ############################
// ######### Structs ##########
// ############################
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    marker: char,
}

// Makes the board that holds the nodes
fn make_board(input: Vec<Vec<char>>) -> Vec<Vec<Node>> {
    let mut out: Vec<Vec<Node>> = vec![];
    for (y, data_row) in input.iter().enumerate() {
        let mut row: Vec<Node> = vec![];
        for (x, &marker) in data_row.iter().enumerate() {
            row.push(
                Node {x, y, marker}
            );  
        }
        out.push(row);
    }
    out
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
// Just a simple abstraction of direction
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {

    fn turn_right(self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST  => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST  => Direction::NORTH,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::NORTH => Direction::WEST,
            Direction::WEST  => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST  => Direction::NORTH,
        }
    }
}




// Stuff to read the input
use std::{fs, thread::current};


fn read_contents(test: bool) -> String {
    // This loads BOTH files into the executable when you compile.
    // It finds them because they are right next to this .rs file.
    if test {
        include_str!("Test.txt").to_string()
    } else {
        include_str!("Input.txt").to_string()
    }
}

fn get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}


