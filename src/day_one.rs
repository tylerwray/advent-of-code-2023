extern crate regex;

use regex::Regex;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn find_calibration_with_digits() -> u32 {
    let file_path = "./src/input/day_one.txt";

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in lines_from_file(file_path) {
        calibration_values.push(calibration_value(line));
    }

    let mut sum: u32 = 0;

    for v in calibration_values {
        sum += v;
    }

    return sum;
}

pub fn find_calibration_with_all() -> u32 {
    let file_path = "./src/input/day_one.txt";

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in lines_from_file(file_path) {
        let formatted = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        calibration_values.push(calibration_value(formatted));
    }

    let mut sum: u32 = 0;

    for v in calibration_values {
        sum += v;
    }

    return sum;
}

const MATCH_DIGIT: &str = r#"(\d)"#;

fn calibration_value(line: String) -> u32 {
    let input_re = Regex::new(MATCH_DIGIT).unwrap();

    let captures: Vec<&str> = input_re.find_iter(&line).map(|m| m.as_str()).collect();

    let calibration = format!("{}{}", captures.first().unwrap(), captures.last().unwrap());

    return calibration.parse::<u32>().unwrap();
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}
