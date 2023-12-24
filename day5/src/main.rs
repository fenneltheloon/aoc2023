use core::fmt;
use std::io;

#[derive(Debug, Clone)]
struct OddError;

impl fmt::Display for OddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "odd number of elements supplied")
    }
}

#[derive(Debug)]
struct Map {
    dest: usize,
    source: usize,
    range: usize,
}

impl Map {
    fn from_str(input: &str) -> Map {
        let mut split = input.split_whitespace();
        let dest = split.next().unwrap().parse::<usize>().unwrap();
        let source = split.next().unwrap().parse::<usize>().unwrap();
        let range = split.next().unwrap().parse::<usize>().unwrap();

        Map {
            dest,
            source,
            range,
        }
    }

    fn from_usize(dest: usize, source: usize, range: usize) -> Map {
        Map {
            dest,
            source,
            range,
        }
    }

    fn map(&self, a: usize) -> Option<usize> {
        if self.is_mappable(a) {
            return Some(a - self.source + self.dest);
        }
        None
    }

    fn is_mappable(&self, a: usize) -> bool {
        a >= self.source && a < self.source + self.range
    }
}

fn main() {
    let mut min: usize = usize::MAX;
    let mut input: Vec<String> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        input.push(line);
    }

    let mut input_iter = input.iter();
    let seeds = input_iter.by_ref().next().unwrap().to_owned();

    let mut seeds = seeds.split_whitespace().skip(1).collect::<Vec<_>>();

    // Now get each of the different maps and add them to a Vec of Vec of maps
    let mut matrix: Vec<Vec<Map>> = Vec::new();

    seeds.reverse();
    matrix.push(Vec::new());
    while !seeds.is_empty() {
        let mut seed_s = seeds.pop().unwrap().to_string();
        let seed_r = seeds.pop().unwrap();
        seed_s.push_str(" ");
        seed_s.push_str(seed_s.to_owned().as_str());
        seed_s.push_str(seed_r);
        matrix[0].push(Map::from_str(seed_s.as_str()));
    }

    input_iter.by_ref().next();

    for i in 0..7 {
        matrix.push(Vec::new());
        let map_s = input_iter
            .by_ref()
            .skip(1)
            .take_while(|l| !((**l).trim().is_empty()));
        for map in map_s {
            matrix[i].push(Map::from_str(map));
        }
    }

    //Now convert each seed into its final mapping
    // First map in each step will be selected and all others in that step will
    // be ignored
    for mut seed in seeds {
        for step in matrix.iter() {
            for map in step {
                if let Some(new) = map.map(seed) {
                    seed = new;
                    break;
                }
            }
        }
        if seed < min {
            min = seed;
        }
    }
    println!("{min}");
}

fn flatten(a: Vec<Map>, b: Vec<Map>) -> Vec<Map> {
    let mut to_return: Vec<Map> = Vec::new();
    for source_map in a {
        let sm1 = source_map.dest;
        let sm2 = sm1 + source_map.range;
        for dest_map in b {
            let dm1 = dest_map.source;
            let dm2 = dm1 + dest_map.range;
            // dm is fully less than sm
            // top end of dm overlaps with sm
            if dm2 >= sm1 && dm1 < sm1 {
                let 
                // TODO uhhh we need to merge these properly idk idk get all of
                // the boundaries into a list and set a flag for the
                // pattern of alternation?
            }
            // dm fully contained in sm
            // bottom of dm overlaps with sm
            // dm fully greater than sm
        }
    }
}
