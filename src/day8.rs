use std::collections::HashMap;

use itertools::Itertools;

trait Single {
    type Item;
    fn single(&self) -> Result<&Self::Item, ()>;
}

impl<T> Single for Vec<T> {
    type Item = T;

    fn single(&self) -> Result<&Self::Item, ()> {
        match self.as_slice() {
            [x] => Ok(x),
            _ => Err(()),
        }
    }
}

pub fn problem1(entries: HashMap<String, String>) -> usize {
    let is_unique_length: [bool; 8] = [false, false, true, true, true, false, false, true];
    entries
        .values()
        .map(|entry| {
            entry
                .split(' ')
                .filter(|digit| {
                    let digit_length = digit.len();
                    assert!(digit_length < 8);
                    is_unique_length[digit_length]
                })
                .count()
        })
        .sum()
}

pub fn problem2(entries: HashMap<String, String>) -> u32 {
    entries
        .iter()
        .map(|(signal, output)| {
            // Convert each encoded value to a bitfield -- if `a` is in the value, set the 0th bit,
            // if `b` is in the value, set the 1st bit, and so on.
            let bitfield_values_for_lengths = signal
                .split(' ')
                .map(|signal_entry| {
                    let mask = get_bitfield_for_encoded(signal_entry);
                    (mask.count_ones() as u8, mask)
                })
                .into_group_map();

            // the indexes here are the real numbers that the bitfield values represent
            let mut bitfield_values: [u8; 10] = [
                0,
                *bitfield_values_for_lengths[&2].single().unwrap(),
                0,
                0,
                *bitfield_values_for_lengths[&4].single().unwrap(),
                0,
                0,
                *bitfield_values_for_lengths[&3].single().unwrap(),
                *bitfield_values_for_lengths[&7].single().unwrap(),
                0,
            ];

            // to find the rest of the numbers, we can check if the unknown value "contains"
            // known values. For instance, the seven-segment representation of 9 is:
            // ***
            // * *
            // ***
            //   *
            //   *
            // this will be represented as a bitstring where each bit is a segment that is active.
            // the bitstring for 4 should be a strict subset of the bitstring for 9 such that
            // bitstrings[4] & bitstrings[9] == bitstrings[4]. We can use these properties to
            // determine which of the unknown 5 and 6-length bitstrings map to which actual values.

            assert_eq!(bitfield_values_for_lengths[&6].len(), 3);
            for &mask in bitfield_values_for_lengths[&6].iter() {
                if mask & bitfield_values[4] == bitfield_values[4] {
                    bitfield_values[9] = mask;
                } else if mask & bitfield_values[7] == bitfield_values[7] {
                    bitfield_values[0] = mask;
                } else {
                    bitfield_values[6] = mask;
                }
            }

            assert_eq!(bitfield_values_for_lengths[&5].len(), 3);
            for &mask in bitfield_values_for_lengths[&5].iter() {
                if mask & bitfield_values[7] == bitfield_values[7] {
                    bitfield_values[3] = mask;
                } else if (mask ^ bitfield_values[6]).count_ones() == 1 {
                    bitfield_values[5] = mask;
                } else {
                    bitfield_values[2] = mask;
                }
            }

            // at this point, every digit should have a corresponding bitfield value
            assert!(!bitfield_values.iter().any(|&mask| mask == 0));

            // map the output for this entry to a decimal value
            output
                .split(' ')
                .rev()
                .map(get_bitfield_for_encoded)
                .enumerate()
                .map(|(power, bitfield)| {
                    let real_digit = bitfield_values.iter().position(|&d| d == bitfield).unwrap();
                    real_digit as u32 * 10u32.pow(power as u32)
                })
                .sum::<u32>()
        })
        .sum()
}

fn get_bitfield_for_encoded(encoded: &str) -> u8 {
    encoded
        .chars()
        .fold(0, |mask, c| mask | (1 << c as u8 - 'a' as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day8";

    fn parse_input(content: &str) -> HashMap<String, String> {
        content
            .split('\n')
            .map(|line| {
                let separated: Vec<&str> = line.split(" | ").collect();
                assert_eq!(separated.len(), 2, "{}", line);
                (
                    separated[0].trim().to_owned(),
                    separated[1].trim().to_owned(),
                )
            })
            .collect()
    }

    #[test]
    fn problem1_example() {
        let example =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let entries = parse_input(example);

        assert_eq!(problem1(entries), 26);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let entries = parse_input(&content);
        assert_eq!(problem1(entries), 534);
    }

    #[test]
    fn problem2_example() {
        let example =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let entries = parse_input(example);

        assert_eq!(problem2(entries), 61229);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let entries = parse_input(&content);
        assert_eq!(problem2(entries), 1070188);
    }
}
