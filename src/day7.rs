pub fn problem1(positions: &[u32]) -> u32 {
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    (min_position..=max_position)
        .map(|candidate| {
            positions.iter().fold(0u32, |cur, elem| {
                cur + (*elem as i32 - candidate as i32).abs() as u32
            })
        })
        .min()
        .unwrap()
}

pub fn problem2(positions: &[u32]) -> u32 {
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    (min_position..=max_position)
        .map(|candidate| {
            positions.iter().fold(0u32, |cur, elem| {
                let distance = (*elem as i32 - candidate as i32).abs() as u32;
                cur + ((distance * (distance + 1)) / 2)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    use super::*;

    const DATA_PATH: &'static str = "data/day7";

    #[test]
    fn problem1_example() {
        let initial_state: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(problem1(&initial_state[..]), 37);
    }

    #[test]
    fn problem1_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let mut reader = BufReader::new(file);

        let mut file_content = String::new();
        reader.read_to_string(&mut file_content).unwrap();

        let initial_state: Vec<u32> = file_content
            .split(',')
            .map(|col| col.parse().unwrap())
            .collect();
        assert_eq!(problem1(initial_state.as_slice()), 352331);
    }

    #[test]
    fn problem2_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let mut reader = BufReader::new(file);

        let mut file_content = String::new();
        reader.read_to_string(&mut file_content).unwrap();

        let initial_state: Vec<u32> = file_content
            .split(',')
            .map(|col| col.parse().unwrap())
            .collect();
        assert_eq!(problem2(initial_state.as_slice()), 99266250);
    }
}
