use std::iter;

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

#[derive(Debug)]
struct Coord {
    pub row: usize,
    pub col: usize,
}

fn surrounding_coords<'a, T>(height_map: &'a Vec<Vec<T>>, pos: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
    (start_index(pos.row)..=end_index(height_map.len(), pos.row))
        .flat_map(|row| {
            (start_index(pos.col)..=end_index(height_map[row].len(), pos.col))
                .map(|col| Coord { row, col })
                .collect::<Vec<_>>()
        })
        .filter(|coord| coord.col != pos.col || coord.row != pos.row)
}

fn basin_coords<'a, T>(height_map: &'a Vec<Vec<T>>, pos: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
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

fn problem2(height_map: Vec<Vec<u8>>) -> u32 {
    let mut next_group_id = 0u32;
    let mut group_id_map = Vec::<Vec<Option<u32>>>::with_capacity(height_map.len());

    for row_index in 0..height_map.len() {
        group_id_map.push(Vec::with_capacity(height_map[row_index].len()));

        for col_index in 0..height_map[row_index].len() {
            let height = height_map[row_index][col_index];
            assert!(height <= 9);

            let group_id_map_value = if height == 9 {
                None
            } else {
                let max_surrounding_value = basin_coords(&group_id_map, &Coord { row: row_index, col: col_index })
                    .filter_map(|coord| group_id_map[coord.row][coord.col])
                    .max();
                
                match max_surrounding_value {
                    Some(contiguous_group_id) => Some(contiguous_group_id),
                    None => {
                        let new_group_id = next_group_id;
                        next_group_id += 1;
                        Some(new_group_id)
                    }
                }
            };
            group_id_map[row_index].push(group_id_map_value);
        }
    }

    for row in group_id_map.iter() {
        for col in row.iter() {
            match col {
                Some(id) => print!("{:03} ", id),
                None => print!("    "),
            }
        }
        println!();
    }

    // |group_id_map| will have some cases where there are contiguous basins with different IDs.
    // We can eliminate these by backtracking through the ID map (from bottom-right to top-left)
    // and always preferring the highest group ID of all of our neighbors
    for row_index in (0..group_id_map.len()).rev() {
        for col_index in (0..group_id_map[row_index].len()).rev() {
            if let Some(current_group_id) = group_id_map[row_index][col_index] {
                let max_adjacent_group_id = basin_coords(&group_id_map, &Coord { row: row_index, col: col_index })
                    .filter_map(|coord| group_id_map[coord.row][coord.col])
                    .max();
                group_id_map[row_index][col_index] = match max_adjacent_group_id {
                    Some(max_adjacent) => Some(max_adjacent),
                    None => Some(current_group_id),
                };
            }
            
        }
    }
    
    println!("After backtracking:");

    for row in group_id_map.iter() {
        for col in row.iter() {
            match col {
                Some(id) => print!("{:03} ", id),
                None => print!("    "),
            }
        }
        println!();
    }

    0
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
        assert_eq!(problem2(height_map), 560);
    }
}
