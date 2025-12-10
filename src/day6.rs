use core::num;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub fn part_1() -> i128 {
    let contents =
        fs::read_to_string("src/data/6.txt").expect("Should have been able to read the file");
    calc(&contents)
}

pub fn part_2() -> i128 {
    let contents =
        fs::read_to_string("src/data/6.txt").expect("Should have been able to read the file");
    calc(&contents)
}

fn calc(input: &str) -> i128 {
    let mut result :i128 = 0;
    let numbers = input
        .lines()
        .take_while(|line| {
            let first = line.chars().next().unwrap();
            first.is_ascii_digit() || first.is_whitespace()
        })
        .map(|line| {
            line.split(' ')
                .map(str::parse::<i128>)
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ops = input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
   // dbg!(&ops);
    let mut i = 0;
    loop {
        if i == 0 {
            let t: Vec<_> =  numbers.iter().filter_map(|xs| xs.get(i)).collect();
            dbg!(t);
        }
        // dbg!(result);
        match ops.get(i) {
            
            None => break,
            Some(&"+") => result += numbers.iter().filter_map(|xs| xs.get(i)).sum::<i128>(),
            Some(&"*") => result += numbers.iter().filter_map(|xs| xs.get(i)).product::<i128>(),
            Some(op) => panic!("Unknown {op}"),
        }
        i += 1;
    }

    //dbg!(&joined);
    //joined.unwrap().0.iter().sum()
    //joined.iter().sum()
    result
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
