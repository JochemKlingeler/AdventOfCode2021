#![warn(clippy::pedantic)]

mod day_02;
mod day_03;

fn main() {
    println!("Day 2, part 1: {}", day_02::part1());
    println!("Day 2, part 2: {}", day_02::part2());
    println!("Day 3, part 1: {}", day_03::part1());
    println!("Day 3, part 2: {}", day_03::part2());
}
