use std::fs;

fn part_one(data: String) {
    let text_data: Vec<&str> = data.split("\n").collect();
    let mut results: i32 = 0;

    for row in text_data {
        let row_parsed: Vec<&str> = row.splitn(2, "|").collect();

        let winning_numbers: Vec<&str> = row_parsed[0]
            .split(":")
            .last()
            .unwrap_or("")
            .trim()
            .split(" ")
            .collect();
        let player_numbers: Vec<i32> = row_parsed
            .last()
            .unwrap_or(&"")
            .trim()
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();

        let mut matches: u32 = 0;
        let base: i32 = 2;

        for number in winning_numbers.into_iter().filter(|x| x != &"") {
            if player_numbers.contains(&number.parse::<i32>().unwrap_or(0)) {
                matches += 1
            }
        }
        if matches != 0 {
            results += base.pow(matches - 1);
        }
    }

    println!("results: {:?}", results);
}

fn main() {
    let data = fs::read("../four/data/data.txt").expect("could not open the file");
    let parsed_data = String::from_utf8(data).unwrap_or(String::from("could not convert file"));

    part_one(parsed_data);
}
