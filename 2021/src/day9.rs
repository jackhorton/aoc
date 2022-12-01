use std::{
    collections::{BinaryHeap, VecDeque},
    iter,
};

fn start_index(index: usize) -> usize {
    match index {
        0 => 0,
        _ => index - 1,
    }
}

fn end_index(length: usize, index: usize) -> usize {
    match index {
        x if length == 0 || x >= length - 1 => x,
        _ => index + 1,
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    pub row: usize,
    pub col: usize,
}

fn surrounding_coords<'a, T>(
    height_map: &'a Vec<Vec<T>>,
    pos: &'a Coord,
) -> impl Iterator<Item = Coord> + 'a {
    (start_index(pos.row)..=end_index(height_map.len(), pos.row))
        .flat_map(|row| {
            (start_index(pos.col)..=end_index(height_map[row].len(), pos.col))
                .map(|col| Coord { row, col })
                .collect::<Vec<_>>()
        })
        .filter(|coord| coord.col != pos.col || coord.row != pos.row)
}

fn basin_coords<'a, T>(
    height_map: &'a Vec<Vec<T>>,
    pos: &'a Coord,
) -> impl Iterator<Item = Coord> + 'a {
    (start_index(pos.row)..=end_index(height_map.len(), pos.row))
        .flat_map(|row| {
            (start_index(pos.col)..=end_index(height_map[row].len(), pos.col))
                .map(|col| Coord { row, col })
                .collect::<Vec<_>>()
        })
        .filter(|coord| (coord.col != pos.col) ^ (coord.row != pos.row))
}

pub fn problem1(height_map: Vec<Vec<u8>>) -> u32 {
    let mut sum_low_point_height = 0u32;
    for (row_index, row) in height_map.iter().enumerate() {
        for (col_index, &height) in row.iter().enumerate() {
            let min_surrounding_value = surrounding_coords(
                &height_map,
                &Coord {
                    row: row_index,
                    col: col_index,
                },
            )
            .map(|coord| height_map[coord.row][coord.col])
            .min()
            .unwrap();

            if min_surrounding_value > height {
                sum_low_point_height += height as u32 + 1;
            }
        }
    }

    sum_low_point_height
}

fn search(
    height_map: &Vec<Vec<u8>>,
    start: Coord,
    basin_id: u32,
    basin_id_map: &mut Vec<Vec<Option<u32>>>,
) -> u32 {
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut basin_size = 0u32;

    while let Some(coord) = q.pop_front() {
        if height_map[coord.row][coord.col] == 9 {
            continue;
        } else if basin_id_map[coord.row][coord.col].is_some() {
            continue;
        }

        basin_id_map[coord.row][coord.col] = Some(basin_id);
        basin_size += 1;

        for candidate in basin_coords(&height_map, &coord) {
            if basin_id_map[candidate.row][candidate.col].is_none()
                && height_map[candidate.row][candidate.col] < 9
            {
                q.push_back(candidate);
            }
        }
    }

    basin_size
}

pub fn problem2(height_map: Vec<Vec<u8>>) -> u32 {
    let mut next_basin_id = 0u32;
    let mut basin_id_map: Vec<Vec<Option<u32>>> =
        iter::repeat(iter::repeat(None).take(height_map[0].len()).collect())
            .take(height_map.len())
            .collect();
    let mut basin_sizes = BinaryHeap::new();

    for row_index in 0..height_map.len() {
        for col_index in 0..height_map[row_index].len() {
            let height = height_map[row_index][col_index];
            assert!(height <= 9);

            match basin_id_map[row_index][col_index] {
                Some(_) => continue,
                None if height == 9 => continue,
                _ => {
                    let basin_size = search(
                        &height_map,
                        Coord {
                            row: row_index,
                            col: col_index,
                        },
                        next_basin_id,
                        &mut basin_id_map,
                    );
                    next_basin_id += 1;
                    basin_sizes.push(basin_size);
                }
            }
        }
    }

    basin_sizes.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day9";

    fn parse_input(input: &str) -> Vec<Vec<u8>> {
        input
            .split('\n')
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect()
    }
    #[test]
    fn problem1_example() {
        let example = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let height_map = parse_input(example);

        assert_eq!(problem1(height_map), 15);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let height_map = parse_input(&content);
        assert_eq!(problem1(height_map), 560);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let height_map = parse_input(&content);
        assert_eq!(problem2(height_map), 959136);
    }
}
