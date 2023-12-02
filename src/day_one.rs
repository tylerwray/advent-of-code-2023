use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn find_calibration_value() -> i32 {
    let file_path = "./src/input/day_one.txt";

    let mut calibration_values: Vec<i32> = Vec::new();

    for line in lines_from_file(file_path) {
        let line_chars: Vec<char> = line.chars().collect();

        let mut first: char = 'e';
        let mut last: char = 'e';

        for c in line_chars {
            if c.is_numeric() {
                if first == 'e' {
                    first = c;
                }
                last = c;
            }
        }

        let calibration = format!("{}{}", first, last);

        let num = calibration.parse::<i32>().unwrap();

        calibration_values.push(num);
    }

    let mut sum: i32 = 0;

    for v in calibration_values {
        sum += v;
    }

    return sum;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}
