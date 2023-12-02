use regex::Regex;
use std::io;
use std::str;

fn main() {
    let stdin = io::stdin();
    let re_f = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_b = Regex::new(r"\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let mut _sum: u32 = 0;
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
        let l = str_to_u8(l).unwrap();
        let r = str_to_u8(r).unwrap();
        let num: u32 = 10u32 * (l as u32) + (r as u32);
        _sum += num;
    }
    println!("{_sum}");
}

fn str_to_u8(a: &str) -> Option<u8> {
    match a {
        "0" | "zero" => Some(0u8),
        "1" | "one" => Some(1u8),
        "2" | "two" => Some(2u8),
        "3" | "three" => Some(3u8),
        "4" | "four" => Some(4u8),
        "5" | "five" => Some(5u8),
        "6" | "six" => Some(6u8),
        "7" | "seven" => Some(7u8),
        "8" | "eight" => Some(8u8),
        "9" | "nine" => Some(9u8),
        _ => None,
    }
}
