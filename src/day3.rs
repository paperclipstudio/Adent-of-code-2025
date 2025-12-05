use core::num;
use std::env;
use std::fs;

pub fn part_1() {
    let contents =
        fs::read_to_string("src/data/3.txt").expect("Should have been able to read the file");
    let count = contents.lines().map(max_volts).sum::<u32>();
    println!("Day3_1 {count}");
}

pub fn part_2() {
    let contents =
        fs::read_to_string("src/data/3.txt").expect("Should have been able to read the file");
    let count = 0;
    // 698 too low
    println!("Day3_2 {count}");
}

fn max_volts(input: &str) -> u32 {
    assert!(input.len() >= 2);
    // Find tens
    let inputs = input.chars().map(|x| x as u32 - '0' as u32).collect::<Vec<_>>();
    let (max_index, max) = inputs
        .iter()
        .enumerate()
        .take(input.len() - 1)
        .max_by(|(_, value), (_, other)| value.cmp(other))
        .unwrap();

    let max_unit = inputs.iter().skip(max_index + 1).max().unwrap();
    max * 10 + max_unit
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: [&str; 5] = [
        "11",
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    static MAX_VOLTS: [u32; 5] = [11, 98, 89, 78, 92];

    #[test]
    fn sample1() {
        for i in 0..4 {
            assert_eq!(MAX_VOLTS[i], max_volts(SAMPLE[i]), "{}", SAMPLE[i]);
        }
    }

    #[test]
    fn sample2() {}
}
