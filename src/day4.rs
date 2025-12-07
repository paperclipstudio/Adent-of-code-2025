use core::num;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::usize;

pub fn part_1() {
    let contents =
        fs::read_to_string("src/data/4.txt").expect("Should have been able to read the file");
    let count = can_access(&contents);
    println!("Day4_1 {count}");
}

pub fn part_2() {
    let contents =
        fs::read_to_string("src/data/4.txt").expect("Should have been able to read the file");
    let count = can_remove(&contents);
    println!("Day4_2 {count}");
}

fn can_access(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines.first().unwrap().len();
    let mut can_move = 0;

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if lines
                .get(y as usize)
                .unwrap()
                .chars()
                .nth(x as usize)
                .unwrap()
                != '@'
            {
                continue;
            }

            let mut count = 0;
            for x1 in -1..2 {
                for y1 in -1..2 {
                    if (x1, y1) == (0, 0) {
                        continue;
                    }
                    if x + x1 < 0 || y + y1 < 0 {
                        continue;
                    }
                    match lines.get((y + y1) as usize) {
                        None => continue,
                        Some(row) => match row.chars().nth((x + x1) as usize) {
                            Some('@') => {
                                count += 1;
                            }
                            _ => continue,
                        },
                    };
                }
            }

            if count < 4 {
                can_move += 1;
            }
        }
    }
    can_move
}

fn can_remove(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|x| x.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let starting_count = lines.iter().flatten().filter(|b| **b).count();

    fn at(lines: &[Vec<bool>], x: usize, y: usize) -> Option<bool> {
        lines.get(y)?.get(x).copied()
    }

    let mut last = usize::MAX;
    let mut current = lines.iter().flatten().filter(|b| **b).count();
    while last != current {
print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{} {}", last, current);
        for row in &lines {
            for roll in row {
                if *roll {
                    print!("<>");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
        println!();
        sleep(Duration::from_millis(33));

        last = current;
        let counts = (0..width * height)
            .map(|index| (index % width, index / width))
            .filter_map(|(x, y)| Some(((x, y), at(&lines, x, y)?)))
            .map(|(pos, char)| {
                (
                    pos,
                    char,
                    (0..9)
                        .map(|index| (index % 3, index / 3))
                        .map(|(x, y)| (x - 1, y - 1))
                        .filter(|loc| *loc != (0, 0))
                        .map(move |(x, y)| (pos.0 as i32 + x, pos.1 as i32 + y))
                        .filter(|(x, y)| !x.is_negative() && !y.is_negative())
                        .map(|(x, y)| (x as usize, y as usize))
                        .filter_map(|(x, y)| at(&lines, x, y))
                        .filter(|b| *b)
                        .count(),
                )
            })
            .collect::<Vec<_>>();

        counts.iter().for_each(|((x, y), char, count)| {
            *lines.get_mut(*y).unwrap().get_mut(*x).unwrap() = *char && *count >= 4;
        });
        current = lines.iter().flatten().filter(|b| **b).count();
    }
    starting_count - current
}

#[cfg(test)]
mod test {
    use super::*;
    static SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn sample1() {
        assert_eq!(13, can_access(SAMPLE), "Given sample");
    }

    #[test]
    fn sample2() {
        assert_eq!(43, can_remove(SAMPLE), "Given sample2");
    }
}
