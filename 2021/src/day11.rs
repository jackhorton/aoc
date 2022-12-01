use std::collections::VecDeque;
use std::ops::AddAssign;

use crate::coord::{Coord, CoordIterator};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnergyLevel {
    Dormant(u8),
    Explosive,
}

impl AddAssign<u8> for EnergyLevel {
    fn add_assign(&mut self, rhs: u8) {
        *self = match *self {
            EnergyLevel::Dormant(level) if level + rhs <= 9 => EnergyLevel::Dormant(level + rhs),
            _ => EnergyLevel::Explosive,
        }
    }
}

impl From<char> for EnergyLevel {
    fn from(c: char) -> Self {
        let c_val = c.to_digit(10).unwrap() as u8;
        assert!(c_val <= 9);
        EnergyLevel::Dormant(c_val)
    }
}

pub fn problem1(input: &Vec<Vec<EnergyLevel>>, iterations: u32) -> u32 {
    let mut flash_map = input.to_vec();
    let mut flashes = 0u32;

    for _ in 1..=iterations {
        flashes += do_flash_step(&mut flash_map);
    }

    flashes
}

pub fn problem2(input: &Vec<Vec<EnergyLevel>>) -> u32 {
    let mut flash_map = input.to_vec();
    let mut steps = 0u32;

    while !flash_map
        .iter()
        .all(|row| row.iter().all(|el| *el == EnergyLevel::Dormant(0)))
    {
        do_flash_step(&mut flash_map);
        steps += 1
    }

    steps
}

fn do_flash_step(map: &mut Vec<Vec<EnergyLevel>>) -> u32 {
    let mut q = VecDeque::new();
    let mut flashes = 0u32;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            map[row][col] += 1;
            if map[row][col] == EnergyLevel::Explosive {
                flashes += 1;
                q.push_back(Coord { row, col })
            }
        }
    }

    while let Some(flash_center) = q.pop_front() {
        for coord in map.surrounding_coords(flash_center) {
            if map[coord.row][coord.col] == EnergyLevel::Explosive {
                continue;
            }

            map[coord.row][coord.col] += 1;
            if map[coord.row][coord.col] == EnergyLevel::Explosive {
                flashes += 1;
                q.push_back(coord)
            }
        }
    }

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == EnergyLevel::Explosive {
                map[row][col] = EnergyLevel::Dormant(0);
            }
        }
    }

    flashes
}

#[allow(dead_code)]
fn stringify_flash_map(flash_map: &Vec<Vec<EnergyLevel>>) -> String {
    let mut printed = String::new();
    for row in 0..flash_map.len() {
        for col in 0..flash_map[0].len() {
            match flash_map[row][col] {
                EnergyLevel::Explosive => printed.push_str("*"),
                EnergyLevel::Dormant(level) => printed.push_str(format!("{}", level).as_str()),
            }
        }
        printed.push('\n');
    }

    printed
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day11";

    fn parse_input(input: &str) -> Vec<Vec<EnergyLevel>> {
        input
            .split('\n')
            .map(|line| line.trim().chars().map(|c| c.into()).collect())
            .collect()
    }

    #[test]
    fn problem1_example() {
        let example = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let energy_levels = parse_input(example);

        assert_eq!(problem1(&energy_levels, 10), 204);
        assert_eq!(problem1(&energy_levels, 100), 1656);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let flash_map = parse_input(&content);
        assert_eq!(problem1(&flash_map, 100), 1637);
    }

    #[test]
    fn problem2_example() {
        let example = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let energy_levels = parse_input(example);

        assert_eq!(problem2(&energy_levels), 195);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let flash_map = parse_input(&content);
        assert_eq!(problem2(&flash_map), 242);
    }
}
