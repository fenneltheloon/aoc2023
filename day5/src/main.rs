use core::fmt;
use std::{cmp::Ordering, io};

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
        if self.source_contains(a) {
            return Some(a - self.source + self.dest);
        }
        None
    }

    fn source_contains(&self, a: usize) -> bool {
        a >= self.source && a < self.source + self.range
    }

    fn dest_contains(&self, a: usize) -> bool {
        a >= self.dest && a < self.dest + self.range
    }

    fn cmp(&self, b: &Map) -> Ordering {
        self.source.cmp(&b.source)
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

fn flatten(source: Vec<Map>, dest: Vec<Map>) -> Vec<Map> {
    source.sort_by(|a, b| a.cmp(b));
    dest.sort_by(|a, b| a.cmp(b));
    for dest_map in dest {
        let index = source.partition_point(|x| x.dest < dest_map.source);
        let final: Vec<Map> = Vec::new();
        while let Some(m) = source.get(index) {
            // Dest map is completely enclosed by source map, split into three
            if m.dest + m.range > dest_map.source && m.dest+m.range > dest_map.source + dest_map.range {
                //remove from source, add union to final, add non-unions to source
                source.remove(index);
                source.insert(index, Map::from_usize(m.dest, m.source, dest_map.source - m.dest));
                final.push(Map::from_usize(dest_map.dest, m.source + dest_map.source - m.dest, dest_map.range));
                source.insert(index + 1, Map::from_usize(dest_map.source + dest_map.range, dest_map.source + dest_map.range, m.range - dest_map.source + m.dest - dest_map.range));
                break;
            } // Otherwise the destination map applies to other maps in the source
            else if m.dest + m.range > dest_map.source {
                source.remove(index);
                source.insert(index, Map::from_usize(m.dest, m.source, dest_map.source - m.dest));
                final.push(Map::from_usize(dest_map.dest, m.source + dest_map.source - m.dest, m.range - dest_map.source + m.dest));
            } else {
                break;
            }
        }
        // Now we need to check at the index previous
        if let Some(m) = source.get(index - 1) {
            // check for overlap
            // if so, remove m from source, add union to final and remainder to source
            // TODO
        }
    }
}
