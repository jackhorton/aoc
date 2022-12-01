use std::collections::HashSet;

#[derive(Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Coord {
    y: usize,
    x: usize,
}

fn fold_index(index: usize, fold: usize) -> usize {
    if index > fold {
        let offset = index % (fold + 1);
        fold - offset - 1
    } else {
        index
    }
}

impl Coord {
    pub fn fold(&self, folds: &[Fold]) -> Self {
        folds.iter().fold(*self, |coord, fold| match fold {
            Fold::X(index) => Coord {
                x: fold_index(coord.x, *index),
                y: coord.y,
            },
            Fold::Y(index) => Coord {
                x: coord.x,
                y: fold_index(coord.y, *index),
            },
        })
    }
}

pub fn count_dots(dot_list: &Vec<Coord>, folds: &[Fold]) -> usize {
    dot_list
        .iter()
        .map(|dot| dot.fold(folds))
        .collect::<HashSet<Coord>>()
        .len()
}

pub fn decode_dots(dot_list: &Vec<Coord>, folds: &[Fold]) -> String {
    let mut ret = String::new();
    let mut folded: Vec<_> = dot_list
        .iter()
        .map(|dot| dot.fold(folds))
        .collect::<HashSet<Coord>>()
        .into_iter()
        .collect();
    folded.sort();

    let mut x_cursor = 0;
    let mut y_cursor = 0;
    for coord in folded.iter() {
        if coord.y > 0 && y_cursor < coord.y {
            ret.push('\n');
            x_cursor = 0;
        }
        y_cursor = coord.y;
        for _ in x_cursor..coord.x {
            ret.push(' ');
        }
        ret.push('X');
        x_cursor = coord.x + 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day13";
    const EXAMPLE_1: &'static str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    const FOLD_PREFIX: &'static str = "fold along ";

    fn parse_input(input: &str) -> (Vec<Coord>, Vec<Fold>) {
        let coords: Vec<_> = input
            .lines()
            .take_while(|line| line.trim().len() != 0)
            .map(|line| {
                let mut split = line.split(',').map(|index| index.parse::<usize>().unwrap());
                Coord {
                    x: split.next().unwrap(),
                    y: split.next().unwrap(),
                }
            })
            .collect();

        let folds = input.lines().skip(coords.len() + 1).map(|line| {
            match line.chars().nth(FOLD_PREFIX.len()).unwrap() {
                'x' => Fold::X(line.split('=').nth(1).unwrap().parse().unwrap()),
                'y' => Fold::Y(line.split('=').nth(1).unwrap().parse().unwrap()),
                _ => panic!(),
            }
        });

        (coords, folds.collect())
    }

    #[test]
    fn problem1_example1() {
        let (coords, folds) = parse_input(EXAMPLE_1);
        assert_eq!(count_dots(&coords, &folds[0..1]), 17);
    }

    #[test]
    fn problem1_example2() {
        let (coords, folds) = parse_input(EXAMPLE_1);
        assert_eq!(count_dots(&coords, &folds[..]), 16);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let (coords, folds) = parse_input(&content);
        assert_eq!(count_dots(&coords, &folds[0..1]), 842);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let (coords, folds) = parse_input(&content);
        assert_eq!(decode_dots(&coords, &folds[..]), "\
XXX  XXXX X  X XXX   XX    XX XXXX X  X
X  X X    X X  X  X X  X    X    X X  X
XXX  XXX  XX   X  X X       X   X  X  X
X  X X    X X  XXX  X       X  X   X  X
X  X X    X X  X X  X  X X  X X    X  X
XXX  X    X  X X  X  XX   XX  XXXX  XX");
    }
}
