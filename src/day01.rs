pub fn run() -> String {
    format!("{}\n{}\n", part1(INPUT), part2(INPUT))
}
fn part1(input: &str) -> String {
    let num = input
        .lines()
        .map(|l| {
            l.chars().find_map(|c| c.to_digit(10)).unwrap() * 10
                + l.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
        })
        .sum::<u32>();
    format!("{num}")
}

fn part2(input: &str) -> String {
    let num: usize = input
        .lines()
        .map(|l| {
            let mut tens = None;
            let mut ones = None;
            for (n, s_num) in NUMBERS.iter().enumerate() {
                if let Some(idx) = l.find(s_num) {
                    match tens {
                        Some((old_idx, _)) => {
                            if idx < old_idx {
                                tens = Some((idx, n / 2 + 1));
                            }
                        }
                        None => tens = Some((idx, n / 2 + 1)),
                    }
                }
                if let Some(idx) = l.rfind(s_num) {
                    match ones {
                        Some((old_idx, _)) => {
                            if idx > old_idx {
                                ones = Some((idx, n / 2 + 1))
                            }
                        }
                        None => ones = Some((idx, n / 2 + 1)),
                    }
                }
            }

            tens.unwrap().1 * 10 + ones.unwrap().1
        })
        .sum();
    format!("{num}")
}

const NUMBERS: [&str; 18] = [
    "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8",
    "eight", "9", "nine",
];

const INPUT: &str = include_str!("inputs/day01.txt");
const TEST: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
