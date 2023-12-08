use std::{iter, cmp::min};
use std::num;

fn main() {
    // let times = vec![7.0, 15.0, 30.0];
    // let distances = vec![9.0, 40.0, 200.0];
    let times = vec![58.0, 81.0, 96.0, 76.0];
    let distances = vec![434.0, 1041.0, 2219.0, 1218.0];
    // println!("{how_many_ways:?}");
    println!("Part 1: {}", races(&times, &distances).iter().product::<i32>());
    let times = vec![58819676.0];
    let distances = vec![434104122191218.0];
    println!("Part 2: {}", races(&times, &distances).iter().product::<i32>());
}

fn races(times: &[f64], distances: &[f64]) -> Vec<i32> {
    let mut how_many_ways = Vec::new();
    for (time, dist) in times.iter().zip(distances) {
        let discrimnant: f64 = (*time * *time - 4.0_f64 * *dist).sqrt();
        let min = f64::ceil(1.0_f64.max(0.5_f64 * (*time - discrimnant)) + 0.0001) as i32;
        let max = f64::floor((*time - 1.0).min(0.5 * (*time + discrimnant)) - 0.0001) as i32;
        // println!("{min} {max}");
        how_many_ways.push(max - min + 1);
    }
    how_many_ways
}
