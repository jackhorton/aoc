use std::vec::Vec;

pub fn day1_problem1(input: Vec<i32>) -> i32 {
    let mut increases = 0;
    for window in input.windows(2) {
        if window[0] < window[1] {
            increases += 1;
        }
    }
    increases
}

pub fn day1_problem2(input: Vec<i32>) -> i32 {
    let window_sums = input
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>();
    day1_problem1(window_sums)
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufRead};
    use std::path::Path;
    use std::fs::File;

    use crate::*;

    #[test]
    fn day1_problem1_example() {
        let result = day1_problem1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(result, 7);
    }

    #[test]
    fn day1_problem1_real() {
        let file = File::open(Path::new("data/day1-1")).unwrap();
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(day1_problem1(input), 1532);
    }

    #[test]
    fn day1_problem2_real() {
        let file = File::open(Path::new("data/day1-1")).unwrap();
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(day1_problem2(input), 1571);
    }
}
