use core::num;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub fn part_1() -> i32 {
    let contents =
        fs::read_to_string("src/data/6.txt").expect("Should have been able to read the file");
    let count = calc(&contents);
    count
}

pub fn part_2() -> i32 {
    let contents =
        fs::read_to_string("src/data/6.txt").expect("Should have been able to read the file");
    let count = calc(&contents);
    count
}

fn calc(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    #[test]
    fn sample1() {
        assert_eq!(4277556, calc(SAMPLE), "Given sample");
    }
}
