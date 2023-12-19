use std::{fs, io};

fn part_one(data: &Vec<&str>) {
    #[derive(Debug)]
    struct PointData {
        number: u32,
        start: u32,
        end: u32,
        is_tagged: bool,
    }

    struct PositionIndex(u32, u32);

    let parsed_data: Vec<&str> = data.clone();
    let symbols = "*+#$%&/@=-";
    let mut result: u32 = 0;
    let numbers_regex = regex::Regex::new(r"(\d+)").unwrap();

    for y in 0..parsed_data.len() {
        for x in parsed_data[y].chars().enumerate() {
            if symbols.contains(x.1) {
                {
                    if y > 1 {
                        let left_border = if x.0 >= 4 { x.0 - 4 } else { 0 };
                        let right_border = if x.0 + 5 <= parsed_data[y].len() {
                            x.0 + 5
                        } else {
                            parsed_data[y].len()
                        };
                        let top_row = &parsed_data[y - 1][left_border..right_border];
                        let found_numbers: Vec<&str> = numbers_regex
                            .find_iter(top_row)
                            .map(|x| x.as_str())
                            .collect();

                        let mut numbers_data: Vec<PointData> = vec![];

                        for number in found_numbers {
                            let item_position: Option<(usize, usize)> =
                                match parsed_data[y - 1].find(number) {
                                    Some(position) => Some((position, position + number.len())),
                                    None => {
                                        println!("an error occurred parsing the position");
                                        None
                                    }
                                };
                            numbers_data.push(PointData {
                                number: number.parse::<u32>().unwrap_or(0),
                                start: item_position.unwrap_or((0, 0)).0 as u32,
                                end: item_position.unwrap_or((0, 0)).1 as u32,
                                is_tagged: false,
                            });

                            for numbers in &mut numbers_data {
                                let top_left_position = (x.0 - 1) as u32;
                                let top_mid_position = (x.0) as u32;
                                let top_right_position = (x.0 + 1) as u32;

                                if (numbers.start..numbers.end).contains(&top_left_position) {
                                    if !numbers.is_tagged {
                                        numbers.is_tagged = true;
                                        result += numbers.number;
                                    }
                                }
                                if (numbers.start..numbers.end).contains(&top_mid_position) {
                                    if !numbers.is_tagged {
                                        numbers.is_tagged = true;
                                        result += numbers.number;
                                    }
                                }
                                if (numbers.start..numbers.end).contains(&top_right_position) {
                                    if !numbers.is_tagged {
                                        numbers.is_tagged = true;
                                        result += numbers.number;
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    let position_left_border = if x.0 >= 4 { x.0 - 4 } else { 0 };
                    let position_right_border = if x.0 + 4 <= parsed_data[y].len() {
                        x.0 + 4
                    } else {
                        parsed_data[y].len()
                    };

                    let left_number = &parsed_data[y][position_left_border..x.0];
                    let right_number = &parsed_data[y][x.0 + 1..position_right_border];

                    if let Some(last_char) = left_number.chars().last() {
                        if last_char.is_numeric() {
                            let parsed_number: String =
                                left_number.chars().filter(|x| x.is_numeric()).collect();
                            result += parsed_number.parse::<u32>().unwrap_or(0);
                        }
                    }

                    if let Some(first_char) = right_number.chars().next() {
                        if first_char.is_numeric() {
                            let parsed_number: String =
                                right_number.chars().filter(|x| x.is_numeric()).collect();
                            result += parsed_number.parse::<u32>().unwrap_or(0);
                        }
                    }
                }
                {
                    if y + 1 < parsed_data.len() {
                        let left_border = if x.0 >= 4 { x.0 - 4 } else { 0 };
                        let right_border = if x.0 + 5 <= parsed_data[y].len() {
                            x.0 + 5
                        } else {
                            parsed_data[y].len()
                        };
                        let bottom_row = &parsed_data[y + 1][left_border..right_border];
                        let found_numbers: Vec<&str> = numbers_regex
                            .find_iter(bottom_row)
                            .map(|x| x.as_str())
                            .collect();

                        let mut numbers_data: Vec<PointData> = vec![];

                        for number in found_numbers {
                            let item_position: Option<(usize, usize)> =
                                match parsed_data[y + 1].find(number) {
                                    Some(position) => Some((position, position + number.len())),
                                    None => {
                                        println!("an error occurred parsing the position");
                                        None
                                    }
                                };
                            numbers_data.push(PointData {
                                number: number.parse::<u32>().unwrap_or(0),
                                start: item_position.unwrap_or((0, 0)).0 as u32,
                                end: item_position.unwrap_or((0, 0)).1 as u32,
                                is_tagged: false,
                            });

                            for numbers in &mut numbers_data {
                                let bottom_left_position = (x.0 - 1) as u32;
                                let bottom_mid_position = (x.0) as u32;
                                let bottom_right_position = (x.0 + 1) as u32;

                                if (numbers.start..numbers.end).contains(&bottom_left_position) {
                                    if !numbers.is_tagged {
                                        numbers.is_tagged = true;
                                        result += numbers.number;
                                    }
                                }
                                if (numbers.start..numbers.end).contains(&bottom_mid_position) {
                                    if !numbers.is_tagged {
                                        numbers.is_tagged = true;
                                        result += numbers.number;
                                    }
                                }
                                if (numbers.start..numbers.end).contains(&bottom_right_position) {
                                    if !numbers.is_tagged {
                                        numbers.is_tagged = true;
                                        result += numbers.number;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("result: {:?}", result);
}

fn main() -> std::result::Result<(), io::Error> {
    let data = fs::read("../three/data/data.txt").expect("could not parse the file");
    let parsed_string = String::from_utf8(data).unwrap_or("".to_string());
    let parsed_data: Vec<&str> = parsed_string.split("\n").collect();

    part_one(&parsed_data);
    Ok(())
}
