use std::env;
use std::fs;

pub fn day1_1() {
    let contents =
        fs::read_to_string("src/data/1.txt").expect("Should have been able to read the file");
    let count = contents
        .lines()
        .map(|x| x.split_at(1))
        .map(|(dir, num)| (dir, num.parse::<i32>().unwrap() % 100))
        .map(|(dir, num)| if dir.starts_with('L') { -num } else { num })
        .scan(50, |st, item| {
            *st = (*st + item) % 100;
            Some(*st)
        })
        .filter(|x| *x == 0)
        .count();
    println!("Day1_1 {count}");
}

pub fn day1_2() {
    let contents =
        fs::read_to_string("src/data/1.txt").expect("Should have been able to read the file");
    let count = count_all(contents);
    println!("Day1_2 {count}");
}

fn count_all(contents: String) -> usize {
    contents
        .lines()
        .map(|x| x.split_at(1))
        .map(|(dir, num)| (dir, num.parse::<i32>().unwrap()))
        .map(|(dir, num)| if dir.starts_with('L') { -num } else { num })
        //      .inspect(|x| println!("\n=={x}"))
        .flat_map(|num| (0..num.abs()).map(move |_| if num < 0 { -1 } else { 1 }))
        .scan(50, |st, item| {
            *st = (*st + item) % 100;
            Some(*st)
        })
        .map(|x| if x < 0 { x + 100 } else { x })
        //       .inspect(|x| print!("{x} "))
        .filter(|x| *x == 0)
        //        .inspect(|x| print!("PING "))
        .count()

    // 2054 too low
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let test = "L68".to_string();
        assert_eq!(1, count_all(test));
    }

    #[test]
    fn basic2() {
        let test = "L68
R68"
        .to_string();
        assert_eq!(2, count_all(test));
    }

    #[test]
    fn basic_to_zero() {
        let test = "L50
"
        .to_string();
        assert_eq!(1, count_all(test));
    }

    #[test]
    fn basic_to_zero_little() {
        let test = "L50
R1
L1
R1
L1"
        .to_string();
        assert_eq!(3, count_all(test));
    }

    #[test]
    fn basic_to_large() {
        let test = "L50
R100
R1000"
            .to_string();
        assert_eq!(12, count_all(test));
    }

    #[test]
    fn sample() {
        let test = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string();
        assert_eq!(6, count_all(test));
    }
}
