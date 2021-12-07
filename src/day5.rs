use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn range_to(&self, dest: &Point) -> impl Iterator<Item = Point> {
        let range_x: Vec<u32> = match self.x.cmp(&dest.x) {
            Ordering::Less => (self.x..=dest.x).collect(),
            Ordering::Equal => iter::repeat(self.x)
                .take((dest.y as i32 - self.y as i32).abs() as usize + 1)
                .collect(),
            Ordering::Greater => (dest.x..=self.x).rev().collect(),
        };
        let range_y: Vec<u32> = match self.y.cmp(&dest.y) {
            Ordering::Less => (self.y..=dest.y).collect(),
            Ordering::Equal => iter::repeat(self.y)
                .take((dest.x as i32 - self.x as i32).abs() as usize + 1)
                .collect(),
            Ordering::Greater => (dest.y..=self.y).rev().collect(),
        };

        range_x
            .into_iter()
            .zip(range_y)
            .map(|p| Self { x: p.0, y: p.1 })
    }

    pub fn is_diagonal_to(&self, dest: &Point) -> bool {
        (self.x as i32 - dest.x as i32).abs() == (self.y as i32 - dest.y as i32).abs()
    }
}

pub struct Vent {
    start: Point,
    end: Point,
}

impl Vent {
    pub fn new(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Vent {
            start: Point {
                x: start_x,
                y: start_y,
            },
            end: Point { x: end_x, y: end_y },
        }
    }
}

pub fn problem1(vents: Vec<Vent>) -> usize {
    let vent_points = vents
        .into_iter()
        .filter_map(|v| {
            if v.start.x == v.end.x || v.start.y == v.end.y {
                Some(v.start.range_to(&v.end))
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<Point>>();

    let mut seen_points = HashMap::<Point, u32>::new();
    for point in vent_points {
        *seen_points.entry(point).or_insert(0) += 1;
    }

    seen_points.values().filter(|seen| **seen > 1).count()
}

pub fn problem2(vents: Vec<Vent>) -> usize {
    let vent_points = vents
        .into_iter()
        .filter_map(|v| {
            if v.start.x == v.end.x || v.start.y == v.end.y || v.start.is_diagonal_to(&v.end) {
                Some(v.start.range_to(&v.end))
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<Point>>();

    // for p in vent_points.iter().take(1000) {
    //     println!("{:?}", p);
    // }

    let mut seen_points = HashMap::<Point, u32>::new();
    for point in vent_points {
        *seen_points.entry(point).or_insert(0) += 1;
    }

    seen_points.values().filter(|seen| **seen > 1).count()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use regex::Regex;

    use super::*;

    const DATA_PATH: &'static str = "data/day5";

    #[test]
    fn problem1_example() {
        let vents = vec![
            Vent::new(0, 9, 5, 9),
            Vent::new(8, 0, 0, 8),
            Vent::new(9, 4, 3, 4),
            Vent::new(2, 2, 2, 1),
            Vent::new(7, 0, 7, 4),
            Vent::new(6, 4, 2, 0),
            Vent::new(0, 9, 2, 9),
            Vent::new(3, 4, 1, 4),
            Vent::new(0, 0, 8, 8),
            Vent::new(5, 5, 8, 2),
        ];

        assert_eq!(problem1(vents), 5);
    }

    #[test]
    fn problem1_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);

        let line_re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let vents = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let captures = line_re.captures(line.as_str()).unwrap();
                // println!("{:?}", captures);
                Vent::new(
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                    captures[4].parse().unwrap(),
                )
            })
            .collect();

        assert_eq!(problem1(vents), 5092);
    }

    #[test]
    fn problem2_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);

        let line_re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let vents = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let captures = line_re.captures(line.as_str()).unwrap();
                Vent::new(
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                    captures[4].parse().unwrap(),
                )
            })
            .collect();

        assert_eq!(problem2(vents), 12041);
    }
}
