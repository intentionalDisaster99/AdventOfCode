mod file_reader;
use crate::file_reader::{ get_lines, read_contents, read_csv };


fn main() {

    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = read_csv(&raw_data);

    todo!();

}


fn second_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = read_csv(&raw_data);

    todo!();

}



// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        assert_eq!(1, first_half(true));
    }
    
    #[test]
    fn second_half_check() {
        assert_eq!(1, second_half(true));
    }


}