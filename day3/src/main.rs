use std::io;

const LINES: usize = 10;
const LINE_WIDTH: usize = 10;

#[derive(Debug)]
struct Num {
    n: u32,
    i: Vec<(u32, u32)>,
}

impl Num {
    fn new(line: u32, num: (usize, &str)) -> Num {
        let n: u32 = num.1.parse().unwrap();
        let mut i: Vec<(u32, u32)> = Vec::new();
        for (j, _) in (0usize..).zip(num.1.as_bytes().iter()) {
            i.push((line, u32::try_from(num.0 + j).unwrap()));
        }
        Num { n, i }
    }
}

fn main() {
    let mut sum: u32 = 0;
    let mut num_list: Vec<Num> = Vec::new();
    let mut sym_grid: [[bool; LINE_WIDTH]; LINES] = [[false; LINE_WIDTH]; LINES];

    // Parse input: store all numbers as data structs in list and all symbols
    // in 2D grid
    for (line_num, line) in (0u32..).zip(io::stdin().lines()) {
        let line = line.unwrap();
        let nums: Vec<&str> = line.split(|c: char| !(c.is_ascii_digit())).collect();
        let mut index_list: Vec<usize> = Vec::new()
        for num in nums {
            index_list.push(line.find(num).unwrap());
        }

        for (num, index) in nums.iter().zip(index_list) {
            let i: Vec<(u32, u32)> = Vec::new();
            for j in 0..num.len() {
                i.push
            }
            num_list.push(Num {n: num.parse::<u32>().unwrap(), i: index});
        }
        
        let match_symbols: Vec<(usize, &str)> = line
            .match_indices(|c: char| !(c.is_ascii_digit() || c == '.'))
            .collect();

        for sym in match_symbols {
            sym_grid[line_num as usize][sym.0] = true;
        }
    }

    println!("{num_list:?}");
    println!("{sym_grid:?}");

    // Now go through list of all numbers and see if any digit borders a symbol
    for num in num_list {
        for coord in num.i {
            if check(coord, &sym_grid) {
                sum += num.n;
            }
        }
    }

    println!("{sum}");
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
