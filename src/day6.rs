pub fn problem1(initial_fish_timers: &[u8], days: u32) -> u64 {
    let mut fish_per_timer: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish_timer in initial_fish_timers {
        assert!(*fish_timer < 9);
        fish_per_timer[*fish_timer as usize] += 1;
    }

    for _ in 0..days {
        fish_per_timer.rotate_left(1);
        fish_per_timer[6] += fish_per_timer[8];
    }

    fish_per_timer.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    use super::*;

    const DATA_PATH: &'static str = "data/day6";

    #[test]
    fn problem1_example() {
        let initial_state: [u8; 5] = [3, 4, 3, 1, 2];
        assert_eq!(problem1(&initial_state[..], 18), 26);
        assert_eq!(problem1(&initial_state[..], 80), 5934);
    }

    #[test]
    fn problem1_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let mut reader = BufReader::new(file);

        let mut file_content = String::new();
        reader.read_to_string(&mut file_content).unwrap();

        let initial_state: Vec<u8> = file_content
            .split(',')
            .map(|col| col.parse().unwrap())
            .collect();
        assert_eq!(problem1(initial_state.as_slice(), 80), 358214);
    }

    #[test]
    fn problem2_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let mut reader = BufReader::new(file);

        let mut file_content = String::new();
        reader.read_to_string(&mut file_content).unwrap();

        let initial_state: Vec<u8> = file_content
            .split(',')
            .map(|col| col.parse().unwrap())
            .collect();
        assert_eq!(problem1(initial_state.as_slice(), 256), 1622533344325);
    }
}
