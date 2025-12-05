use std::env;
use std::fs;

fn is_repeat(x: u128) -> bool {
    let length = x.ilog10();
    if length.is_multiple_of(2)  {
        return false;
    }
    let sub_length = (length / 2) + 1;
    let other = 10_u128.pow(sub_length);
    //println!("{other}");
    x / other == x % other
}

fn count_invalid(input: &str) -> u128 {
    input.split(",")
        .filter_map(|str| str.split_once('-'))
        .map(|x|(x.0.trim_matches('\n'), x.1.trim_matches('\n')))
        .inspect(|x| println!("<<{} {}>>", x.0, x.1))
        .map(|x|(x.0.parse::<u128>().unwrap(), x.1.trim_end().parse::<u128>().unwrap()))
        .flat_map(|(x,y)| x..y+1)
        .filter(|x| is_repeat(*x))
        .inspect(|x| println!("{x}"))
        .sum()

}

fn is_repeat_2(x: u128) -> bool {
//        println!("{x}");
    let digits = x.ilog10() + 1;
    // dbg!(x, digits);
    for i in 1..digits {
        if !(digits).is_multiple_of(i) {
            continue;
        }
        // dbg!(i);
        let mut test = x;
        let divisor = 10_u128.pow(i);
        // dbg!(divisor);

        let start = test % divisor;
        if start == 0 {
            continue;
        }
      // println!("=== {start}");
        while test > 0 {
            dbg!(test);
      // println!(">>> {test}");
            if test % divisor != start {
                break;
            }
            test /= divisor;
            if test == 0 {
                // println!("TEST is 0");
                return true;
            }
        }
    }
    false
    //println!("{other}");


}

fn count_invalid_2(input: &str) -> u128 {
    input.split(",")
        .filter_map(|str| str.split_once('-'))
        .map(|x|(x.0.trim_matches('\n'), x.1.trim_matches('\n')))
        .inspect(|x| println!("<<{} {}>>", x.0, x.1))
        .map(|x|(x.0.parse::<u128>().unwrap(), x.1.trim_end().parse::<u128>().unwrap()))
        .flat_map(|(x,y)| x..y+1)
        .filter(|x| is_repeat_2(*x))
        .inspect(|x| println!("{x}"))
        .sum()

        // 35367539282 too low
}

pub fn part_1() {
    let contents =
        fs::read_to_string("src/data/2.txt").expect("Should have been able to read the file");
    let count = count_invalid(&contents);
    
    // 698 too low
    println!("Day2_1 {count}");
}

pub fn part_2() {
    let contents =
        fs::read_to_string("src/data/2.txt").expect("Should have been able to read the file");
    let count = count_invalid_2(&contents);
    
    // 698 too low
    println!("Day2_2 {count}");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let repeats = [11,22,33,1212,321321];
        for i in repeats {
            assert!(is_repeat(i), "{i} should be a repeats");
        }
    }

    #[test]
    fn name2() {
        let repeats = [10,4,313,1234,123443321];
        for i in repeats {
            assert!(!is_repeat(i), "{i} shouldn't be a repeats");
        }
    }

    #[test]
    fn name3() {
        let repeats = [12341234, 111,2222,333,123123123,121212];
        for i in repeats {
            assert!(is_repeat_2(i), "{i} should be a repeat 2");
        }
    }

    #[test]
    fn name4() {
        let repeats = [110,100,303,123,123123124];
        for i in repeats {
            assert!(!is_repeat_2(i), "{i} shouldn't be a repeat 2");
        }
    }

    static SAMPLE:&str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn sample1() {
        assert_eq!(1227775554, count_invalid(SAMPLE));
    }

    #[test]
    fn sample2() {
        assert_eq!(4174379265, count_invalid_2(SAMPLE));
    }


}
