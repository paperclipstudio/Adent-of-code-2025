use core::num;
use std::cmp::Ordering;
use std::env;
use std::fs;

pub fn part_1() {
    let contents =
        fs::read_to_string("src/data/3.txt").expect("Should have been able to read the file");
    let count = contents.lines().map(max_volts).sum::<u64>();
    println!("Day3_1 {count}");
}

pub fn part_2() {
    let contents =
        fs::read_to_string("src/data/3.txt").expect("Should have been able to read the file");
    let count = contents.lines().map(max_volts_2).sum::<u64>();
    // 698 too low
    // 16664 too low
    println!("Day3_2 {count}");
}

fn max_volts(input: &str) -> u64 {
    assert!(input.len() >= 2);
    // Find tens
    let inputs = input
        .chars()
        .map(|x| x as u64 - '0' as u64)
        .collect::<Vec<_>>();
    let (max_index, max) = inputs
        .iter()
        .enumerate()
        .take(input.len() - 1)
        .max_by(|(_, value), (_, other)| value.cmp(other))
        .unwrap();

    let max_unit = inputs.iter().skip(max_index + 1).max().unwrap();
    max * 10 + max_unit
}

fn max_volts_2(input: &str) -> u64 {
    // dbg!(input);
    assert!(input.len() >= 13);
    const LEN:usize = 12;
    let digits: [u64; 12];
    // Find tens
    let mut start_index= 0 ;
    let mut output = 0;
    let inputs = input
        .chars()
        .map(|x| x as u64 - '0' as u64)
        .collect::<Vec<_>>();
    for i in 0..12 {
        let max;
        // Digits let to add to number
        let digits_left = LEN - i;
        // charaters left in the input
        let inputs_left = input.len() - start_index + 1;
        // Number of inputs can choose from
        let can_choose = inputs_left - digits_left;
        // dbg!(digits_left, can_choose);
        (start_index, max) = inputs
            .iter()
            .enumerate()
            .skip(start_index)
            .take(can_choose)
            // .inspect(|(x,y)| print!("{x}-{y} "))
            .max_by(|(_, value), (_, other)| if value != other {value.cmp(other)} else {Ordering::Greater})
            .unwrap();
        // println!();
        start_index += 1; 
        output += max * 10_u64.pow(11 - i as u32);
        //dbg!(output, start_index);
    }
    output
}

        // 811111111111 119
        // 811111111111119
        // 811111111111119
        //  2342 34234234278
        // 234 23 4234234278
        // 23 4234 234234278
#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: [&str; 4] = [
        "234234234234278",
        "811111111111119",
        "987654321111111",
        "818181911112111",
    ];

    static MAX_VOLTS: [u64; 4] = [
        78, 
        89, 
        98, 
        92];

    static MAX_VOLTS_2: [u64; 4] = [
        434234234278, 
        811111111119, 
        987654321111, 
        888911112111,
    ];

    #[test]
    fn sample1() {
        for i in 0..4 {
            assert_eq!(MAX_VOLTS[i], max_volts(SAMPLE[i]), "{}", SAMPLE[i]);
        }
    }

    #[test]
    fn sample2() {
        for i in 0..4 {
            assert_eq!(MAX_VOLTS_2[i], max_volts_2(SAMPLE[i]), "{}", SAMPLE[i]);
        }
    }

    #[test]
    fn sample3() {
        assert_eq!(3121910778619_u64, SAMPLE.map(max_volts_2).iter().sum());
    }
}
