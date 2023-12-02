use regex::Regex;
use std::io;
use std::str;

fn main() {
    let stdin = io::stdin();
    let re_f = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_b = Regex::new(r"\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let mut sum = 0u32;
    for line in stdin.lines() {
        let line = line.unwrap();
        let line = line.as_str();
        let line_rev: String = line.chars().rev().collect();
        let line_rev = line_rev.as_str();
        let l = re_f.find(line).unwrap().as_str();
        let r: String = re_b
            .find(line_rev)
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect();
        let r = r.as_str();
        let l = str_to_u32(l).unwrap();
        let r = str_to_u32(r).unwrap();
        let num = 10 * l + r;
        sum += num;
    }
    println!("{sum}");
}

fn str_to_u32(a: &str) -> Option<u32> {
    match a {
        "0" | "zero" => Some(0),
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}
