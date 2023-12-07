use std::{
    fs::File,
    io::{self, Read},
};

use regex::Regex;

fn parse_data() -> io::Result<String> {
    let mut file = match File::open("data/data.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let data = parse_data().expect("an error occured");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut all_numbers: i32 = 0;
    let num_regex = Regex::new(r"\d").unwrap();
    for x in lines {
        let found_numbers: Vec<&str> = num_regex.find_iter(x).map(|value| value.as_str()).collect();
        let first_num = found_numbers[0];
        let last_num = found_numbers[found_numbers.len() - 1];
        let combined_numbers: i32 = format!("{}{}", first_num, last_num)
            .parse()
            .expect("could not parse the number");
        all_numbers += combined_numbers
    }
    println!("all numbers = {:?}", all_numbers)
}
