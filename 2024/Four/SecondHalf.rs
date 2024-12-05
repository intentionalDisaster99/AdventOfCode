use regex::Regex;

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The formats that we are looking for
    let to_look_for = vec![
        r"M[^;]S.{139}A.{139}M[^;]S",
        r"S[^;]S.{139}A.{139}M[^;]M",
        r"S[^;]M.{139}A.{139}S[^;]M",
        r"M[^;]M.{139}A.{139}S[^;]S",
    ];

    let mut answer = 0;

    // The working pattern
    let mut re;

    // This also takes freaking ages, so I might just remake this at some point - it's O(6n) I think, but n is the number of characters (140^2)
    //                                                                            but that is assuming the regex time complexity is O(1), which it isn't...
    for pattern in to_look_for {
        re = Regex::new(pattern).unwrap();

        // Manually moving because you can't look forward in rust 😭
        for start in 0..data.join(";").len() - (7 + 139 * 2) {
            if start + (7 + 139 * 2 + 1) > data.join(";").len() {
                continue;
            }
            let slice = &data.join(";")[start..start + (7 + 139 * 2)];
            if re.is_match(slice) {
                answer += 1;
            }
        }
    }

    println!("The answer is {}", answer);
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Four\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
