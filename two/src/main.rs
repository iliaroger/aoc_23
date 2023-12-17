use regex;
use std::{fs, io};

struct RegexErrorWrapper(regex::Error);

impl From<regex::Error> for RegexErrorWrapper {
    fn from(err: regex::Error) -> Self {
        RegexErrorWrapper(err)
    }
}

impl From<RegexErrorWrapper> for std::io::Error {
    fn from(wrapper: RegexErrorWrapper) -> Self {
        return std::io::Error::new(std::io::ErrorKind::Other, wrapper.0.to_string());
    }
}

fn part_one() -> std::result::Result<(), io::Error> {
    let data = fs::read("../two/data/data.txt").expect("could not read the file");
    let parsed_data: String = match String::from_utf8(data) {
        Ok(text) => text,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    };
    let vec_text: Vec<&str> = parsed_data.split("\n").collect();
    let red_regex = regex::Regex::new(r"(\d+) red").map_err(RegexErrorWrapper)?;
    let blue_regex = regex::Regex::new(r"(\d+) blue").map_err(RegexErrorWrapper)?;
    let green_regex = regex::Regex::new(r"(\d+) green").map_err(RegexErrorWrapper)?;
    let score_regex = regex::Regex::new(r"Game (\d+)").map_err(RegexErrorWrapper)?;
    let mut overall_score: i32 = 0;
    #[derive(Debug)]
    struct AllColors {
        red: bool,
        blue: bool,
        green: bool,
    }

    for item in vec_text {
        let mut green_colors: Vec<&str> = vec![];
        let mut blue_colors: Vec<&str> = vec![];
        let mut red_colors: Vec<&str> = vec![];
        let mut current_game: Vec<&str> = vec![];
        let mut set_colors = AllColors {
            red: false,
            blue: false,
            green: false,
        };

        let all_reds: Vec<&str> = red_regex
            .find_iter(item)
            .map(|x| {
                let first_part: Vec<&str> = x.as_str().splitn(2, " ").collect();
                *first_part.get(0).unwrap_or(&"")
            })
            .collect();
        red_colors.extend(all_reds);

        let all_blues: Vec<&str> = blue_regex
            .find_iter(item)
            .map(|x| {
                let first_part: Vec<&str> = x.as_str().splitn(2, " ").collect();
                *first_part.get(0).unwrap_or(&"")
            })
            .collect();
        blue_colors.extend(all_blues);

        let all_greens: Vec<&str> = green_regex
            .find_iter(item)
            .map(|x| {
                let first_part: Vec<&str> = x.as_str().splitn(2, " ").collect();
                *first_part.get(0).unwrap_or(&"")
            })
            .collect();
        green_colors.extend(all_greens);

        current_game = score_regex
            .find_iter(item)
            .map(|x| {
                let game: Vec<&str> = x.as_str().splitn(2, " ").collect();
                *game.get(1).unwrap_or(&"")
            })
            .collect();

        for color in &red_colors {
            let current_item = color.parse::<i32>().unwrap_or(0);
            if set_colors.red {
                break;
            }
            if current_item > 12 {
                set_colors.red = true;
            }
        }
        for color in &blue_colors {
            let current_item = color.parse::<i32>().unwrap_or(0);
            if set_colors.blue {
                break;
            }
            if current_item > 14 {
                set_colors.blue = true;
            }
        }
        for color in &green_colors {
            let current_item = color.parse::<i32>().unwrap_or(0);
            if set_colors.green {
                break;
            }
            if current_item > 13 {
                set_colors.green = true;
            }
        }

        if !set_colors.red && !set_colors.blue && !set_colors.green {
            overall_score += current_game[0].parse::<i32>().unwrap_or(0);
        }
    }

    println!("score: {:?}", overall_score);

    Ok(())
}

fn main() {
    part_one().expect("something happend while calling the function");
}
