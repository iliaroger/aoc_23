use std::{
    fmt::Error,
    fs::File,
    io::{self, Read},
    ops::Index,
    thread::current,
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

fn first_part() -> () {
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
    println!("all numbers: {:?}", all_numbers)
}

fn second_part() -> () {
    let data = parse_data().expect("an error occurred while opening the file");
    let parsed_data: Vec<&str> = data.split("\n").into_iter().collect();
    let split_regex =
        Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let mut result_all = 0;
    for item in parsed_data {
        let mut collected_numbers: Vec<String> = vec![];
        for single_char in 0..item.as_bytes().len() {
            let mut complete_char = Vec::new();
            let mut new_iter: bool = true;
            for current_word in single_char..item.as_bytes().len() {
                if let Some(byte) = item.as_bytes().get(current_word) {
                    complete_char.push(*byte);
                }
            }
            if let Ok(word) = String::from_utf8(complete_char) {
                let found_nums: Vec<String> = split_regex
                    .find_iter(&word)
                    .map(|x| x.as_str().to_string())
                    .collect();
                for num in found_nums {
                    if !collected_numbers.contains(&num) && new_iter {
                        collected_numbers.push(num)
                    }
                    new_iter = false;
                }
            } else {
                println!("invalid utf-8 sequence")
            }
        }
        let mut first_num = match collected_numbers[0].parse::<i32>() {
            Ok(val) => val,
            Err(_) => -1,
        };

        if first_num == -1 {
            first_num = match &collected_numbers[0][..] {
                "zero" => 0,
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            }
        }

        let mut last_num = match collected_numbers[collected_numbers.len() - 1].parse::<i32>() {
            Ok(val) => val,
            Err(_) => -1,
        };

        if last_num == -1 {
            last_num = match &collected_numbers[collected_numbers.len() - 1][..] {
                "zero" => 0,
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            }
        }

        result_all += [first_num.to_string(), last_num.to_string()]
            .join("")
            .parse::<i32>()
            .expect("could not parse value");
    }
    println!("result: {:?}", result_all);
}

fn main() {
    // first_part();
    second_part();
}
