#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    fn to(self, end: Coord) -> impl Iterator<Item = Coord> {
        assert!(self.row < end.row);
        assert!(self.col < end.col);
        (self.row..=end.row)
            .flat_map(move |row| (self.col..=end.col).map(move |col| Coord { row, col }))
            .filter(move |coord| coord.row != self.row || coord.col != self.col)
    }

    pub fn distance_to(&self, end: &Coord) -> f64 {
        let row_distance = if self.row > end.row {
            self.row - end.row
        } else {
            end.row - self.row
        } as f64;
        let col_distance = if self.col > end.col {
            self.col - end.col
        } else {
            end.col - self.col
        } as f64;

        (row_distance.powf(2.0) + col_distance.powf(2.0)).sqrt()
    }
}

pub struct Coords {
    coord_range: Vec<Coord>,
    coord_index: usize,
}

impl Coords {
    pub fn new_surrounding(max: Coord, center: Coord) -> Self {
        let (top_left, bottom_right) = get_bounding_coords(&max, &center);

        Coords {
            coord_range: top_left.to(bottom_right).collect(),
            coord_index: 0,
        }
    }

    pub fn new_neighbors(max: Coord, center: Coord) -> Self {
        let (top_left, bottom_right) = get_bounding_coords(&max, &center);

        Coords {
            coord_range: top_left
                .to(bottom_right)
                .filter(|c| (c.row != center.row) ^ (c.col != center.col))
                .collect(),
            coord_index: 0,
        }
    }
}

fn get_bounding_coords(max: &Coord, center: &Coord) -> (Coord, Coord) {
    let top_left = match center {
        Coord { row: 0, col: 0 } => *center,
        Coord { row: 0, col } => Coord {
            row: 0,
            col: col - 1,
        },
        Coord { row, col: 0 } => Coord {
            row: row - 1,
            col: 0,
        },
        Coord { row, col } => Coord {
            row: row - 1,
            col: col - 1,
        },
    };
    let bottom_right = match *center {
        c if c == *max => *center,
        Coord { row, col } if row == max.row => Coord { row, col: col + 1 },
        Coord { row, col } if col == max.col => Coord { row: row + 1, col },
        Coord { row, col } => Coord {
            row: row + 1,
            col: col + 1,
        },
    };
    (top_left, bottom_right)
}

impl Iterator for Coords {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        match self.coord_range.get(self.coord_index) {
            Some(coord) => {
                self.coord_index += 1;
                Some(*coord)
            }
            _ => None,
        }
    }
}

pub trait CoordIterator<T> {
    fn surrounding_coords(&self, center: Coord) -> Coords;
    fn neighbor_coords(&self, center: Coord) -> Coords;
}

impl<T> CoordIterator<T> for Vec<Vec<T>> {
    fn surrounding_coords(&self, center: Coord) -> Coords {
        Coords::new_surrounding(Coord { row: self.len() - 1, col: self[0].len() - 1 }, center)
    }

    fn neighbor_coords(&self, center: Coord) -> Coords {
        Coords::new_neighbors(Coord { row: self.len() - 1, col: self[0].len() - 1 }, center)
    }
}
