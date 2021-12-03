use std::cmp::Ordering;

pub fn problem1(readings: &[u32], bits: u32) -> u32 {
    if bits > 32 {
        panic!("Only support up to 32bit power readings");
    } else if let Some(index) = readings.iter().position(|r| *r > (1 << bits)) {
        panic!("Element {} at index {} uses more than {} bits", readings[index], index, bits);
    }

    let mut gamma = 0u32;
    for bit in 0..bits {
        let count_set = readings
            .iter()
            .fold(0, |acc, elem| {
                if elem & (1 << bit) > 0 {
                    acc + 1
                } else {
                    acc
                }
            });

        let half = readings.len() / 2;
        if count_set == half && readings.len() % 2 == 0 {
            panic!("Equal number of 0s and 1s");
        }
        if count_set > half {
            gamma |= 1 << bit;
        }
    }
    
    let epsilon_mask = (2u32.pow(bits)) - 1;
    let epsilon = !gamma & epsilon_mask;
    gamma * epsilon
}

fn problem2_impl(readings: &[u32], bits: u32, filter_to_bit: impl Fn(Option<u32>) -> u32) -> u32 {
    let mut valid_set = readings.to_vec();
    for bit in (0..bits).rev() {
        let count_set = valid_set
            .iter()
            .fold(0, |acc, elem| {
                if elem & (1 << bit) > 0 {
                    acc + 1
                } else {
                    acc
                }
            });
        let most_common_bit = match valid_set.len().cmp(&(count_set * 2)) {
            Ordering::Less => Some(1),
            Ordering::Equal => None,
            Ordering::Greater => Some(0),
        };
        valid_set = valid_set.into_iter().filter(|r| (r >> bit) & 1 == filter_to_bit(most_common_bit)).collect();

        if valid_set.len() == 1 {
            return valid_set[0];
        }
    }

    panic!("Single valid value not found. {} items remain", valid_set.len());
}

fn ogr_filter(most_common_bit: Option<u32>) -> u32 {
    match most_common_bit {
        Some(bit) => bit,
        None => 1
    }
}

fn scrubber_filter(most_common_bit: Option<u32>) -> u32 {
    match most_common_bit {
        Some(bit) => bit ^ 1,
        None => 0
    }
}

pub fn problem2(readings: &[u32], bits: u32) -> u32 {
    if bits > 32 {
        panic!("Only support up to 32bit power readings");
    } else if let Some(index) = readings.iter().position(|r| *r > (1 << bits)) {
        panic!("Element {} at index {} uses more than {} bits", readings[index], index, bits);
    }

    let ogr = problem2_impl(readings, bits, ogr_filter);
    let scrubber = problem2_impl(readings, bits, scrubber_filter);

    ogr * scrubber
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufRead};
    use std::path::Path;
    use std::fs::File;

    use super::*;

    const DATA_PATH: &'static str = "data/day3";

    #[test]
    fn problem1_example() {
        let input = [0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
        assert_eq!(problem1(&input, 5), 198);
    }

    #[test]
    fn problem1_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Vec<Result<String, _>>>();
        let bits = u32::try_from(lines[0].as_ref().unwrap().len()).unwrap();
        let readings = lines
            .iter()
            .map(|line| {
                u32::from_str_radix(line.as_ref().unwrap().as_str(), 2).unwrap()
            })
            .collect::<Vec<u32>>();

        assert_eq!(problem1(readings.as_slice(), bits), 1540244)
    }

    #[test]
    fn problem2_real() {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Vec<Result<String, _>>>();
        let bits = u32::try_from(lines[0].as_ref().unwrap().len()).unwrap();
        let readings = lines
            .iter()
            .map(|line| {
                u32::from_str_radix(line.as_ref().unwrap().as_str(), 2).unwrap()
            })
            .collect::<Vec<u32>>();

        assert_eq!(problem2(readings.as_slice(), bits), 4203981)
    }
}