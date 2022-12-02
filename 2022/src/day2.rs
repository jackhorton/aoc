pub fn solve(input: &str, table: &[u32; 9]) -> u32 {
    input
        .lines()
        .map(|line| line.chars())
        .fold(0u32, |score, mut chars| {
            let row_offset = (chars.nth(0).unwrap() as u8 - 'A' as u8) * 3;
            // nth() consumes characters, so even though its index 2 in the line, its index 1 after consuming
            // the 0th character on the previous line
            let col_offset = chars.nth(1).unwrap() as u8 - 'X' as u8;
            score + table[(row_offset + col_offset) as usize]
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day2";

    const ROCK: u32 = 1;
    const PAPER: u32 = 2;
    const SCISSORS: u32 = 3;

    const WIN: u32 = 6;
    const DRAW: u32 = 3;
    const LOSE: u32 = 0;

    const PROBLEM1_LOOKUP: [u32; 9] = [
        ROCK + DRAW, PAPER + WIN,  SCISSORS + LOSE,
        ROCK + LOSE, PAPER + DRAW, SCISSORS + WIN,
        ROCK + WIN,  PAPER + LOSE, SCISSORS + DRAW,
    ];

    const PROBLEM2_LOOKUP: [u32; 9] = [
        LOSE + SCISSORS, DRAW + ROCK,     WIN + PAPER,
        LOSE + ROCK,     DRAW + PAPER,    WIN + SCISSORS,
        LOSE + PAPER,    DRAW + SCISSORS, WIN + ROCK,
    ];

    #[test]
    fn problem1_example() {
        let example = "\
A Y
B X
C Z";

        assert_eq!(solve(&example, &PROBLEM1_LOOKUP), 15);
    }

    #[test]
    fn problem1_real() {
        let data = std::fs::read_to_string(DATA_PATH).unwrap();
        assert_eq!(solve(&data, &PROBLEM1_LOOKUP), 11603);
    }

    #[test]
    fn problem2_example() {
        let example = "\
A Y
B X
C Z";

        assert_eq!(solve(&example, &PROBLEM2_LOOKUP), 12);
    }

    #[test]
    fn problem2_real() {
        let data = std::fs::read_to_string(DATA_PATH).unwrap();
        assert_eq!(solve(&data, &PROBLEM2_LOOKUP), 12725);
    }
}