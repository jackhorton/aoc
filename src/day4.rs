use std::collections::HashMap;

// the thinking here: an efficient way to check for matches is to compare a bitfield of what has
// been drawn against a bitfield for every bingo case.

const ROWS: usize = 5;
const COLUMNS: usize = 5;

pub fn problem1(boards: Vec<Vec<u8>>, draws: Vec<u8>) -> u32 {
    // key = bingo mask, value = board index
    let mut bingo_masks: HashMap<u128, usize> =
        HashMap::with_capacity(boards.len() * ROWS * COLUMNS);
    let mut board_value_masks: Vec<u128> = Vec::with_capacity(boards.len());

    for (board_index, board) in boards.iter().enumerate() {
        let mut board_value_mask = 0u128;
        assert_eq!(board.len(), ROWS * COLUMNS);

        let row_bingos = board
            .chunks_exact(ROWS)
            .map(|chunk| chunk.iter().fold(0u128, |acc, elem| acc | (1 << elem)));

        for row_bingo in row_bingos {
            assert_eq!(board_value_mask & row_bingo, 0);
            board_value_mask |= row_bingo;
            bingo_masks.insert(row_bingo, board_index);
        }

        let column_bingos = (0..COLUMNS).map(|col| {
            board
                .iter()
                .skip(col)
                .step_by(COLUMNS)
                .fold(0u128, |acc, elem| acc | (1 << elem))
        });

        for column_bingo in column_bingos {
            // we don't need to add these to board_value_mask, since every number on the board
            // has already been added by adding each row above.
            bingo_masks.insert(column_bingo, board_index);
        }

        board_value_masks.push(board_value_mask);
    }

    // now, search for the shortest prefix of |draws| that matches a corresponding bingo mask
    let mut draw_mask = 0u128;
    for draw in draws {
        draw_mask |= 1 << draw;

        let found_bingo = bingo_masks.iter().find_map(|(bingo_mask, board_index)| {
            if draw_mask & bingo_mask == *bingo_mask {
                Some(board_index)
            } else {
                None
            }
        });

        if let Some(winning_board_index) = found_bingo {
            // cool, we found our bingo. now, we need to sum all of the unmarked numbers
            let undrawn_mask = !draw_mask;
            let board_value_mask = board_value_masks[*winning_board_index];

            // take the bitfield of all of the numbers in the board and zero out everything that
            // has been drawn so far
            let board_undrawn_value_mask = board_value_mask & undrawn_mask;
            let unmarked_sum = (0..100).fold(0u32, |acc, elem| {
                if board_undrawn_value_mask & (1 << elem) != 0 {
                    acc + elem
                } else {
                    acc
                }
            });

            return unmarked_sum * draw as u32;
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use super::*;

    const DATA_PATH: &'static str = "data/day4";

    #[test]
    fn problem1_example() {
        let draws: Vec<u8> = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards: Vec<Vec<u8>> = vec![
            vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ],
            vec![
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ],
            vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
        ];

        assert_eq!(problem1(boards, draws), 4512);
    }

    #[test]
    fn problem1_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let draws_line = lines.next().unwrap().unwrap();
        let draws = draws_line
            .split(',')
            .map(|draw| draw.parse().unwrap())
            .collect::<Vec<u8>>();

        // skip a line to get to the first board
        lines.next();

        let mut boards: Vec<Vec<u8>> = Vec::with_capacity(100);
        let mut current_board: Vec<u8> = Vec::with_capacity(ROWS * COLUMNS);
        for line in lines.map(|l| l.unwrap()) {
            if line.trim().len() == 0 {
                boards.push(current_board);
                current_board = Vec::with_capacity(ROWS * COLUMNS);
                continue;
            }

            for value in line
                .split(char::is_whitespace)
                .filter(|v| v.trim().len() > 0)
            {
                current_board.push(value.parse().unwrap());
            }
        }

        assert_eq!(problem1(boards, draws), 38913);
    }
}
