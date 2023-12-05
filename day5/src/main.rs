use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;

use indicatif::ProgressBar;

fn main() {
    // Parse input
    let mut seeds = Vec::new();
    let mut maps: Vec<Vec<(Range<i64>, i64)>> = Vec::new();
    if let Ok(mut lines) = read_lines("./input.txt") {
        let seed_line = lines.next().unwrap().unwrap();
        for seed in seed_line.split_whitespace().skip(1) {
            seeds.push(seed.parse::<i64>().unwrap());
        }
        let _blank = lines.next();
        let mut next_map: Vec<(Range<i64>, i64)> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" && next_map.len() > 0 {
                    maps.push(next_map.clone());
                    next_map.clear();
                }
                let fields: Vec<&str> = ip.split_whitespace().collect();
                if fields.len() == 3 {
                    let start = fields[1].parse::<i64>().unwrap();
                    let end = fields[2].parse::<i64>().unwrap() + start;
                    let offset = fields[0].parse::<i64>().unwrap() - start;
                    next_map.push((start..end, offset));
                }
            }
        }
        maps.push(next_map);
    }

    // Part 1
    let locations: Vec<i64> = seeds.iter().map(|s| map_seed(*s, &maps)).collect();
    println!("Part 1: {}", locations.iter().min().unwrap());

    // Part 2
    let mut minimum = i64::MAX;
    for seed_range in seeds.chunks(2) {
        let bar = ProgressBar::new(seed_range[1] as u64);
        for seed in seed_range[0]..(seed_range[0] + seed_range[1]) {
            bar.inc(1);
            let location = map_seed(seed, &maps);
            //println!("{seed} {location}");
            if location < minimum {
                minimum = location;
            }
        }
    }
    println!("Part 2: {}", minimum);
}

fn map_seed(seed: i64, maps: &Vec<Vec<(Range<i64>, i64)>>) -> i64 {
    let mut mapped_value = seed;
    for map in maps {
        for entry in map {
            if entry.0.contains(&mapped_value) {
                mapped_value += entry.1;
                break;
            }
        }
        // println!("{mapped_value}");
    }
    mapped_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
