use std::collections::BinaryHeap;

use crate::coord::{Coord, Coords};

struct OpenSetValue {
    coord: Coord,
    estimated_cost: f64,
}

impl PartialEq for OpenSetValue {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.estimated_cost == other.estimated_cost
    }
}

impl Eq for OpenSetValue {}

impl PartialOrd for OpenSetValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // we want reverse ordering for open_set values -- that is, we want
        // ourselves to return |Ordering::Greater| if our estimated cost is less
        // than |other|'s. This is to work with the default BinaryHeap behavior,
        // which is a max-heap rather than a min-heap
        other.estimated_cost.partial_cmp(&self.estimated_cost)
    }
}

impl Ord for OpenSetValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.partial_cmp(self).unwrap()
    }
}

const ORIGIN: Coord = Coord { row: 0, col: 0 };

pub struct RiskMap {
    map: Vec<Vec<u8>>,
    goal: Coord,
    rows: usize,
    cols: usize,
}

impl RiskMap {
    pub fn new(map: Vec<Vec<u8>>, tiles: usize) -> Self {
        assert!(tiles > 0);
        assert!(map.len() > 0);
        assert!(map[0].len() > 0);

        let rows = map.len();
        let cols = map[0].len();
        let goal = Coord {
            row: (rows * tiles) - 1,
            col: (cols * tiles) - 1,
        };
        Self {
            map,
            goal,
            rows,
            cols,
        }
    }

    pub fn get(&self, coord: &Coord) -> u8 {
        let real_row = coord.row % self.rows;
        let real_col = coord.col % self.cols;
        let shift = (coord.row / self.rows) + (coord.col / self.cols);
        let shifted_real_value = self.map[real_row][real_col] + shift as u8;
        if shifted_real_value > 9 {
            (shifted_real_value % 10) + 1
        } else {
            shifted_real_value
        }
    }
}

pub fn a_star(map: &RiskMap) -> u32 {
    let mut open_set = BinaryHeap::from([OpenSetValue {
        coord: ORIGIN,
        estimated_cost: ORIGIN.distance_to(&map.goal),
    }]);

    // There is *probably* a way to keep track of the best weight for each position without
    // making the matrix 5x larger in both dimensions. However, that's above my pay grade.
    let mut best_weights = vec![vec![u32::MAX; map.goal.col + 1]; map.goal.row + 1];
    best_weights[ORIGIN.row][ORIGIN.col] = 0;

    while let Some(current) = open_set.pop() {
        if current.coord == map.goal {
            return best_weights[current.coord.row][current.coord.col];
        }

        let current_weight = best_weights[current.coord.row][current.coord.col];
        for neighbor in Coords::new_neighbors(map.goal, current.coord) {
            let new_neighbor_weight = current_weight + map.get(&neighbor) as u32;
            let current_neighbor_weight = best_weights[neighbor.row][neighbor.col];
            if new_neighbor_weight < current_neighbor_weight {
                best_weights[neighbor.row][neighbor.col] = new_neighbor_weight;

                open_set.push(OpenSetValue {
                    coord: neighbor,
                    estimated_cost: new_neighbor_weight as f64 + neighbor.distance_to(&map.goal),
                });
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day15";
    const EXAMPLE_1: &'static str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    fn parse_input(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn problem1_example() {
        let map = parse_input(EXAMPLE_1);
        assert_eq!(a_star(&RiskMap::new(map, 1)), 40);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let map = parse_input(&content);
        assert_eq!(a_star(&RiskMap::new(map, 1)), 429);
    }

    #[test]
    fn problem2_example() {
        let map = parse_input(EXAMPLE_1);
        assert_eq!(a_star(&RiskMap::new(map, 5)), 315);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let map = parse_input(&content);
        assert_eq!(a_star(&RiskMap::new(map, 5)), 2844);
    }
}
