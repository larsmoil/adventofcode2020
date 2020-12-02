use std::time::Instant;

mod day01;
mod day02;

pub mod expenses;
pub mod passwords;

fn main() {
    let problems: Vec<fn()> = vec![
        || print!("day01 pt. 1: {:8}", day01::pt1(day01::input())),
        || print!("day01 pt. 2: {:8}", day01::pt2(day01::input())),
        || print!("day02 pt. 1: {:8}", day02::pt1(day02::input())),
        || print!("day02 pt. 2: {:8}", day02::pt2(day02::input()))
    ];
    for x in problems {
        let start = Instant::now();
        x();
        let elapsed = start.elapsed();
        println!(" (in {:4}ms)", elapsed.as_millis());
    }
}
