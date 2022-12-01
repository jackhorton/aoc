use std::{collections::HashMap, fmt::Display};

use itertools::{Itertools, MinMaxResult};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pair(char, char);

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.0.to_ascii_uppercase(),
            self.1.to_ascii_uppercase()
        )
    }
}

const fn char_index(c: &char) -> usize {
    (*c as u8 - 'A' as u8) as usize
}

pub fn expand_and_count(template: &Vec<char>, rules: &HashMap<Pair, char>, iterations: u8) -> u64 {
    let mut letter_counts = [0u64; 26];
    let mut pair_counts = HashMap::new();

    for letter in template.iter() {
        letter_counts[char_index(letter)] += 1;
    }

    for pair in template.windows(2).map(|window| Pair(window[0], window[1])) {
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        let current_pairs = pair_counts.clone();
        for (pair, count) in current_pairs {
            let insertion = rules[&pair];

            letter_counts[char_index(&insertion)] += count;

            *pair_counts.get_mut(&pair).unwrap() -= count;
            *pair_counts.entry(Pair(pair.0, insertion)).or_insert(0) += count;
            *pair_counts.entry(Pair(insertion, pair.1)).or_insert(0) += count;
        }
    }

    match letter_counts
        .into_iter()
        .filter(|count| *count > 0)
        .minmax()
    {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day14";
    const EXAMPLE_1: &'static str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    fn parse_input(input: &str) -> (Vec<char>, HashMap<Pair, char>) {
        let mut rules = HashMap::new();
        let mut lines = input.lines();
        let template = lines.next().unwrap();
        lines.next().unwrap();
        for rule in lines {
            let mut split = rule.split(" -> ");

            let pair_str = split.next().unwrap();
            assert_eq!(pair_str.len(), 2);
            let mut pair_chars = pair_str.chars();

            let insertion = split.next().unwrap();
            assert_eq!(insertion.len(), 1);

            assert_eq!(split.next(), None);

            rules.insert(
                Pair(pair_chars.next().unwrap(), pair_chars.next().unwrap()),
                insertion.chars().next().unwrap(),
            );
        }

        (template.chars().collect(), rules)
    }

    #[test]
    fn problem1_example1() {
        let (template, rules) = parse_input(EXAMPLE_1);
        assert_eq!(expand_and_count(&template, &rules, 10), 1588);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let (template, rules) = parse_input(&content);
        assert_eq!(expand_and_count(&template, &rules, 10), 3230);
    }

    #[test]
    fn problem2_example1() {
        let (template, rules) = parse_input(EXAMPLE_1);
        assert_eq!(expand_and_count(&template, &rules, 40), 2188189693529);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let (template, rules) = parse_input(&content);
        assert_eq!(expand_and_count(&template, &rules, 40), 3542388214529);
    }
}
