use std::{fs, io};

fn part_one() -> std::result::Result<(), io::Error> {
    let data = fs::read("../two/data/data.txt").expect("could not read the file");
    let parsed_data: String = match String::from_utf8(data) {
        Ok(text) => text,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    };
    let vec_text: Vec<&str> = parsed_data.split("\n").collect();
    for item in vec_text {
        let mut green_colors: Vec<&str> = vec![];
        let mut blue_colors: Vec<&str> = vec![];
        let mut red_colors: Vec<&str> = vec![];
        let mut current_game: i32 = 0;
    }

    Ok(())
}

fn main() {
    part_one().expect("something happend while calling the function");
}
