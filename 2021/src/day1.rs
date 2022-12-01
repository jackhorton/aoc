use std::vec::Vec;

pub fn problem1(input: Vec<i32>) -> i32 {
    let mut increases = 0;
    for window in input.windows(2) {
        if window[0] < window[1] {
            increases += 1;
        }
    }
    increases
}

pub fn problem2(input: Vec<i32>) -> i32 {
    let window_sums = input
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>();
    problem1(window_sums)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    const DATA_PATH: &'static str = "data/day1";

    #[test]
    fn problem1_example() {
        let result = problem1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(result, 7);
    }

    #[test]
    fn problem1_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(problem1(input), 1532);
    }

    #[test]
    fn problem2_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(problem2(input), 1571);
    }
}
