use regex::Regex;
use std::io;

#[derive(Debug)]
struct Num {
    n: usize,
    i: Vec<(usize, usize)>,
}

impl Num {
    fn new(line: usize, num: (usize, &str)) -> Num {
        let n: usize = num.1.parse().unwrap();
        let mut i: Vec<(usize, usize)> = Vec::new();
        for (j, _) in (0usize..).zip(num.1.as_bytes().iter()) {
            i.push((line, num.0 + j));
        }
        Num { n, i }
    }
}

fn main() {
    let mut sum: usize = 0;
    let mut gear_list: Vec<(usize, usize)> = Vec::new();
    let mut num_list: Vec<Num> = Vec::new();
    let re_d = Regex::new(r"\d+").unwrap();
    let re_gear = Regex::new(r"\*").unwrap();
    // Parse input: store all numbers as data structs in list and all symbols
    // in 2D grid
    for (line_num, line) in (0usize..).zip(io::stdin().lines()) {
        let line = line.unwrap();
        let nums = re_d.find_iter(line.as_str());
        let gears = re_gear.find_iter(line.as_str());
        for num in nums {
            num_list.push(Num::new(line_num, (num.start(), num.as_str())));
        }
        for gear in gears {
            gear_list.push((line_num, gear.start()));
        }
    }

    // Go through all gears and check to see if they border exactly 2 numbers
    // Only need to look at numbers that are line above, same line, and line
    // below
    for gear in gear_list {
        let mut adjacents: Vec<usize> = Vec::new();
        let nums = num_list.iter().skip_while(|n| n.i[0].0 < gear.0 - 1);
        for num in nums {
            if num.i[0].0 > gear.0 + 1 || adjacents.len() > 2 {
                break;
            }
            // TODO we only need to check the x coords for adjacency now!!
            for digit in num.i.iter() {
                if digit.1 >= gear.1 - 1 && digit.1 <= gear.1 + 1 {
                    adjacents.push(num.n);
                    break;
                }
            }
        }
        if adjacents.len() == 2 {
            sum += adjacents[0] * adjacents[1];
        }
    }
    println!("{sum}");
}
