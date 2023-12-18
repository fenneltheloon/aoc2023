use std::io;

struct Map {
    dest: usize,
    source: usize,
    range: usize,
}

impl Map {
    fn new(input: &str) -> Map {
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

    let seeds: Vec<&str> = seeds.split_whitespace().skip(1).collect();
    let mut seeds_int: Vec<usize> = Vec::new();

    for seed in seeds {
        let seed = seed.parse::<usize>().unwrap();
        seeds_int.push(seed);
    }

    let seeds = seeds_int;

    // Now get each of the different maps and add them to a Vec of Vec of maps
    let mut matrix: Vec<Vec<Map>> = Vec::new();

    for i in 0..7 {
        matrix.push(Vec::new());
        let map_s = input_iter.by_ref().skip(1).take_while(|l| !(l.is_empty()));
        for map in map_s {
            matrix[i].push(Map::new(map));
        }
    }

    //Now conert each seed into its final mapping
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
