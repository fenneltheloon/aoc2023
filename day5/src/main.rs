use core::fmt;
use std::{cmp::Ordering, io};

#[derive(Debug, Clone)]
struct OddError;

impl fmt::Display for OddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "odd number of elements supplied")
    }
}

#[derive(Debug, Copy, Clone)]
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

    fn cmp_source(&self, b: &Map) -> Ordering {
        self.source.cmp(&b.source)
    }

    fn cmp_dest(&self, b: &Map) -> Ordering {
        self.dest.cmp(&b.dest)
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
        seed_s.push(' ');
        seed_s.push_str(seed_s.to_owned().as_str());
        seed_s.push_str(seed_r);
        matrix[0].push(Map::from_str(seed_s.as_str()));
    }

    input_iter.by_ref().next();

    for i in 1..8 {
        matrix.push(Vec::new());
        let map_s = input_iter
            .by_ref()
            .skip(1)
            .take_while(|l| !((**l).trim().is_empty()));
        for map in map_s {
            matrix[i].push(Map::from_str(map));
        }
    }
    println!("{matrix:#?}");

    matrix.reverse();
    // Flatten the stack of maps
    while matrix.len() > 1 {
        let source = matrix.pop().unwrap();
        let dest = matrix.pop().unwrap();
        matrix.push(flatten(source, dest));
    }
    println!("{matrix:#?}");

    // Now get the dest value from each map in the vector and print the minimum
    for map in &matrix[0] {
        if map.dest < min {
            min = map.dest;
        }
    }

    println!("{min}");
}

fn flatten(mut source: Vec<Map>, mut dest: Vec<Map>) -> Vec<Map> {
    source.sort_by(|a, b| a.cmp_dest(b));
    dest.sort_by(|a, b| a.cmp_source(b));
    let mut ret: Vec<Map> = Vec::new();
    for dest_map in dest {
        let index = source.partition_point(|m| m.dest + m.range < dest_map.source);
        println!("source: {source:#?}\n index: {index}");
        let mut i = index;
        while i < source.len() {
            let m = source[i];
            //         SSSSSSSSSS
            // DDDDDDDDDDDDDDDDDDDDDDDDD
            if dest_map.source <= m.dest && dest_map.source + dest_map.range >= m.dest + m.range {
                source.remove(index);
                ret.push(Map::from_usize(
                    dest_map.dest + m.dest - dest_map.source,
                    m.source,
                    m.range,
                ));
            }
            // SSSSSSSSSSSSSSSSSSSSSSS
            //           DDDDDDD
            else if dest_map.source + dest_map.range < m.dest + m.range
                && dest_map.source > m.dest
            {
                //remove from source, add union to ret, add non-unions to source
                source.remove(index);
                println!("dest_map: {dest_map:#?}, m: {m:#?}");
                source.insert(
                    index,
                    Map::from_usize(m.dest, m.source, dest_map.source - m.dest),
                );
                ret.push(Map::from_usize(
                    dest_map.dest,
                    m.source + dest_map.source - m.dest,
                    dest_map.range,
                ));
                source.insert(
                    index + 1,
                    Map::from_usize(
                        dest_map.source + dest_map.range,
                        m.source + dest_map.range + dest_map.source - m.dest,
                        m.range - dest_map.range - dest_map.source + m.dest,
                    ),
                );
                break;
            }
            // SSSSSSSSSSSS
            //         DDDDDDDDDDDDD
            else if dest_map.source + dest_map.range > m.dest + m.range
                && dest_map.source > m.dest
            {
                source.remove(index);
                source.insert(
                    index,
                    Map::from_usize(m.dest, m.source, dest_map.source - m.dest),
                );
                ret.push(Map::from_usize(
                    dest_map.dest,
                    m.source + dest_map.source - m.dest,
                    m.range - dest_map.source + m.dest,
                ));
            }
            //          SSSSSSSSSS
            // DDDDDDDDDDDDDD
            else if dest_map.source + dest_map.range > m.dest && dest_map.source < m.dest {
                source.remove(index);
                ret.push(Map::from_usize(
                    dest_map.dest + m.dest - dest_map.source,
                    m.source,
                    dest_map.range - m.dest + dest_map.source,
                ));
                source.insert(
                    index,
                    Map::from_usize(
                        dest_map.source + dest_map.range,
                        m.source + dest_map.range - m.dest + dest_map.source,
                        m.range - dest_map.range - m.dest + dest_map.source,
                    ),
                );
                break;
            } else {
                println!("How did we get here?");
            }
            i += 1;
        }
    }
    ret.append(&mut source);
    ret.sort_by(|a, b| a.cmp_dest(b));
    ret
}
