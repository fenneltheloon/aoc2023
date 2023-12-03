use regex::Regex;
use std::io;

fn main() {
    let mut sum: u32 = 0;
    let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();
    for line in io::stdin().lines() {
        // 0 = red
        // 1 = green
        // 2 = blue
        let mut mins = [0u32; 3];

        let line = line.unwrap();
        let reads = re.captures_iter(line.as_str());
        for read in reads {
            let read: (&str, [&str; 2]) = read.extract();
            let read: Vec<&str> = read.1.into();
            update_mins(read, &mut mins).unwrap();
        }
        let power: u32 = mins[0] * mins[1] * mins[2];
        sum += power;
    }
    println!("{sum}");
}

// Takes an individual color and determines if it meets criteria
// fn parse_item(a: Vec<&str>) -> Result<bool, io::Error> {
//     let n: u32 = a[0].parse().unwrap();
//     match a[1] {
//         "red" => Ok(n <= RED_MAX),
//         "green" => Ok(n <= GREEN_MAX),
//         "blue" => Ok(n <= BLUE_MAX),
//         _ => Err(io::Error::other("color not found")),
//     }
// }

// For each color, check to see if the min possible value needs to be increased
fn update_mins(a: Vec<&str>, mins: &mut [u32; 3]) -> Result<(), io::Error> {
    let n: u32 = a[0].parse().unwrap();
    match a[1] {
        "red" => {
            if mins[0] < n {
                mins[0] = n;
            }
        }
        "green" => {
            if mins[1] < n {
                mins[1] = n;
            }
        }
        "blue" => {
            if mins[2] < n {
                mins[2] = n;
            }
        }
        _ => return Err(io::Error::other("color not found")),
    }
    Ok(())
}
