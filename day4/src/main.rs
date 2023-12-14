use std::io;

fn main() {
    let mut sum: usize = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut line = line.split('|');
        let lh = line.next().unwrap();
        let rh = line.next().unwrap();
        let wn: Vec<usize> = lh
            .split_whitespace()
            .skip(2)
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let cn: Vec<usize> = rh
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let matches = check(wn, cn);
        if matches > 0 {
            sum += 2usize.pow(u32::try_from(matches - 1).unwrap());
        }
    }
    println!("{sum}");
}

fn check(mut wn: Vec<usize>, cn: Vec<usize>) -> usize {
    let mut matches: usize = 0;
    for n in cn {
        if wn.is_empty() {
            break;
        }
        for i in 0..wn.len() {
            if n == wn[i] {
                matches += 1;
                wn.remove(i);
                break;
            }
        }
    }
    matches
}
