use core::num;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub fn part_1() {
    let contents =
        fs::read_to_string("src/data/5.txt").expect("Should have been able to read the file");
    let count = count_fresh(&contents);
    println!("Day5_1 {count}");
}

pub fn part_2() {
    let contents =
        fs::read_to_string("src/data/5.txt").expect("Should have been able to read the file");
    let count = count_fresh_2(&contents);
    println!("Day5_2 {count}");
}

fn count_fresh(input: &str) -> usize {
    let ranges = input
        .lines()
        .take_while(|x| !x.is_empty())
        .filter_map(|x| x.split_once('-'))
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let items = input
        .lines()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    dbg!(ranges.len(), items.len());
    let mut count = 0;
    for item in items {
        if ranges.iter().any(|(from, to)| *from <= item && item <= *to) {
            count += 1;
        }
    }
    count
}

    type Num = u128;
fn count_fresh_2(input: &str) -> Num {
    let mut ranges = input
        .lines()
        .take_while(|x| !x.is_empty())
        .filter_map(|x| x.split_once('-'))
        .map(|(a, b)| (a.parse::<Num>().unwrap(), b.parse::<Num>().unwrap()))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|f| f.1);
    ranges.sort_by_key(|f| f.0);
    let mut range_size = usize::MAX;
    let mut count = 0;
    while ranges.len() != range_size {
        count += 1;
        if count > 100 {
            panic!();
        }
        range_size = ranges.len();
        ranges = ranges
            .chunk_by(|a,b| b.0 <= a.1)
            .map(|xs| {
                // print!("{:?}", xs);
                let head = xs.first().unwrap().0;
                let tail = xs.iter().map(|f|f.1).max().unwrap();
                (head, tail)

            })
            .inspect(|f| print!("{} {} -- ", f.0, f.1))
            //    .inspect(|f| drop(dbg!(f)))
            .collect();
        println!();
    }
    // 344378119285261 too low
    // 344378119285354
    ranges.iter().map(|f| f.1 - f.0 + 1).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    static SAMPLE_2: &str = "3-5
10-14
25-30
10-12
10-11
13-14
11-12

";
    static SAMPLE_3: &str = "0-11
20-30
10-20
100-110
20-30
30-40
40-50
";

    #[test]
    fn sample1() {
        assert_eq!(3, count_fresh(SAMPLE), "Given sample");
    }

    #[test]
    fn sample2() {
        assert_eq!(14, count_fresh_2(SAMPLE), "Given sample");
    }

    #[test]
    fn sample3() {
        assert_eq!(14, count_fresh_2(SAMPLE_2), "Given sample");
    }

    #[test]
    fn sample4() {
        assert_eq!(62, count_fresh_2(SAMPLE_3), "Given sample");
    }
}
