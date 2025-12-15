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
    let data: Vec<String> = get_lines(&raw_data);
    
    
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



fn second_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);
    
    
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

    // We should be able to find every path from svr-dac-fft-out and add it to svr
    // let svr_dac: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
    //     &devices, 
    //     *nodes.get("svr").expect("Can't find svr!"), 
    //     *nodes.get("dac").expect("Can't find out"),
    //     0,
    //     None
    // ).collect();
    // let dac_fft: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
    //     &devices, 
    //     *nodes.get("dac").expect("Can't find dac!"), 
    //     *nodes.get("fft").expect("Can't find fft"),
    //     0,
    //     None
    // ).collect();
    // let fft_out: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
    //     &devices, 
    //     *nodes.get("fft").expect("Can't find fft!"), 
    //     *nodes.get("out").expect("Can't find out"),
    //     0,
    //     None
    // ).collect();
    // println!("Found {} with dac first.", svr_dac.len() * dac_fft.len() * fft_out.len());
    // let svr_fft: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
    //     &devices, 
    //     *nodes.get("svr").expect("Can't find svr!"), 
    //     *nodes.get("fft").expect("Can't find fft"),
    //     0,
    //     None
    // ).collect();
    // let fft_dac: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
    //     &devices, 
    //     *nodes.get("fft").expect("Can't find fft!"), 
    //     *nodes.get("dac").expect("Can't find dac"),
    //     0,
    //     None
    // ).collect();
    // let dac_out: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<NodeIndex>, _, RandomState>(
    //     &devices, 
    //     *nodes.get("dac").expect("Can't find dac!"), 
    //     *nodes.get("out").expect("Can't find out"),
    //     0,
    //     None
    // ).collect();

    let dac = *nodes.get("dac").unwrap();
    let fft = *nodes.get("fft").unwrap();
    let svr = *nodes.get("svr").unwrap();
    let out = *nodes.get("out").unwrap();



    let total = rayon::join(
        || {
            if has_path_connecting(&devices, svr, dac, None) 
                && has_path_connecting(&devices, dac, fft, None) 
                && has_path_connecting(&devices, fft, out, None) {

                let svr_dac: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<_>, _, RandomState>(&devices, svr, dac, 0, None).collect();   
                let dac_fft: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<_>, _, RandomState>(&devices, dac, fft, 0, None).collect();
                let fft_out: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<_>, _, RandomState>(&devices, fft, out, 0, None).collect();
                svr_dac.len() * dac_fft.len() * fft_out.len()

            } else {
                // panic!("Didn't find path with dac first");
                0
            }
        },
        || {
            if has_path_connecting(&devices, svr, fft, None) 
                && has_path_connecting(&devices, fft, dac, None) 
            && has_path_connecting(&devices, dac, out, None) {
                let svr_fft: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<_>, _, RandomState>(&devices, svr, fft, 0, None).collect();
                let fft_dac: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<_>, _, RandomState>(&devices, fft, dac, 0, None).collect();
                let dac_out: Vec<Vec<NodeIndex>> = all_simple_paths::<Vec<_>, _, RandomState>(&devices, dac, out, 0, None).collect();
                svr_fft.len() * fft_dac.len() * dac_out.len()
            } else {
                // panic!("Didn't find path with fft first");
                0
            }
        },
    );

    (total.0 + total.1) as u128   

    // (svr_dac.len() * dac_fft.len() * fft_out.len() + svr_fft.len() * fft_dac.len() * dac_out.len()) as u128


}

fn different_second_half(test: bool) -> u128 {
    
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);
    
    
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

    let dac_node =  nodes.get("dac").unwrap();
    let fft_node =  nodes.get("fft").unwrap();
    let start_node =  nodes.get("svr").unwrap();
    let end_node = nodes.get("out").unwrap();

    
    // I love love memoization
    let mut memo = HashMap::new(); 

    // Going fft then dac
    let svr_to_dac = count_paths_to_target(&devices, *start_node, *dac_node, &mut memo);
    let dac_to_fft = count_paths_to_target(&devices, *dac_node, *fft_node, &mut memo);
    let fft_to_out = count_paths_to_target(&devices, *fft_node, *end_node, &mut memo);
    let count_a = svr_to_dac * dac_to_fft * fft_to_out;
    println!("a: {}", count_a);

    // memo = HashMap::

    // Going dac fft
    let svr_to_fft = count_paths_to_target(&devices, *start_node, *fft_node, &mut memo);
    let fft_to_dac = count_paths_to_target(&devices, *fft_node, *dac_node, &mut memo);
    let dac_to_out = count_paths_to_target(&devices, *dac_node, *end_node, &mut memo);
    let count_b = svr_to_fft * fft_to_dac * dac_to_out;
    println!("b: {}", count_b);

    count_a + count_b


}

// It was taking far too long with the old solution, so 
fn count_paths_to_target(
    graph: &Graph<&str, i32>,
    start_node: NodeIndex,
    end_node: NodeIndex,
    memo: &mut HashMap<NodeIndex, u128>,
) -> u128 {
    // Base case: If we are already at the end node, there is 1 path (the current one)
    if start_node == end_node {
        return 1;
    }
    
    // Check if we've already computed the paths from this node to the end
    if let Some(&count) = memo.get(&start_node) {
        return count;
    }

    let mut total_count = 0;
    // Iterate over all neighbors (children) of the current node
    for edge in graph.edges_directed(start_node, Outgoing) {
        let next_node = edge.target();
        // Recursively sum the counts from all downstream paths
        total_count += count_paths_to_target(graph, next_node, end_node, memo);
    }

    // Memoize the result before returning
    memo.insert(start_node, total_count);
    total_count
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
        Some(a) => { println!("Second half {}", {if a == first_half(true){"passing!"} else {"FAILING"}})},
    }
}