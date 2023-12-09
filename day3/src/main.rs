use regex::Regex;
use std::io;

const LINES: usize = 140;
const LINE_WIDTH: usize = 140;

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
    let mut sum: u32 = 0;
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
        let nums = num_list.iter().skip_while(|n| n.i[0].0 < gear.0 - 1);
        for num in nums {
            if num.i[0].0 > gear.0 + 1 {
                break;
            }
            // TODO we only need to check the x coords for adjacency now!!
        }
    }
}

fn check(coords: (u32, u32), grid: &[[bool; LINE_WIDTH]; LINES]) -> bool {
    let coords: (usize, usize) = (
        usize::try_from(coords.0).unwrap(),
        usize::try_from(coords.1).unwrap(),
    );
    // tl, tm, tr, ml, mr, bl, bm, br
    let mut directions: [bool; 8] = [false; 8];
    let (xs, ys, xl, yl) = (
        coords.0 == 0,
        coords.1 == 0,
        coords.0 == LINE_WIDTH - 1,
        coords.1 == LINES - 1,
    );
    if !(xs || ys) {
        directions[0] = grid[coords.0 - 1][coords.1 - 1];
    }
    if !ys {
        directions[1] = grid[coords.0][coords.1 - 1];
    }
    if !(xl || ys) {
        directions[2] = grid[coords.0 + 1][coords.1 - 1];
    }
    if !xs {
        directions[3] = grid[coords.0 - 1][coords.1];
    }
    if !xl {
        directions[4] = grid[coords.0 + 1][coords.1];
    }
    if !(xs || yl) {
        directions[5] = grid[coords.0 - 1][coords.1 + 1];
    }
    if !yl {
        directions[6] = grid[coords.0][coords.1 + 1];
    }
    if !(xl || yl) {
        directions[7] = grid[coords.0 + 1][coords.1 + 1];
    }

    let mut result = false;
    for d in directions {
        result = result || d;
    }

    result
}
