use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub mod expenses;
pub mod passwords;

fn main() {
    let problems: Vec<fn()> = vec![
        || print!("day01 pt. 1: {:8}", day01::pt1(day01::input())),
        || print!("day01 pt. 2: {:8}", day01::pt2(day01::input())),
        || print!("day02 pt. 1: {:8}", day02::pt1(day02::input())),
        || print!("day02 pt. 2: {:8}", day02::pt2(day02::input())),
        || print!("day03 pt. 1: {:8}", day03::pt1(day03::input())),
        || print!("day03 pt. 2: {:8}", day03::pt2(day03::input())),
        || print!("day04 pt. 1: {:8}", day04::pt1(day04::input())),
        || print!("day04 pt. 2: {:8}", day04::pt2(day04::input())),
        || print!("day05 pt. 1: {:8}", day05::pt1(day05::input())),
        || print!("day05 pt. 2: {:8}", day05::pt2(day05::input())),
        || print!("day06 pt. 1: {:8}", day06::pt1(day06::input())),
        || print!("day06 pt. 2: {:8}", day06::pt2(day06::input()))
    ];
    for x in problems {
        let start = Instant::now();
        x();
        let elapsed = start.elapsed();
        println!(" (in {:4}ms)", elapsed.as_millis());
    }
}
