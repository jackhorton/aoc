pub fn calories_per_elf(input: &str) -> Vec<u32> {
    let mut calories_per_elf = vec![0u32];
    for line in input.split('\n') {
        if line.len() == 0 {
            calories_per_elf.push(0);
        } else {
            let elf_index = calories_per_elf.len() - 1;
            calories_per_elf[elf_index] += line.parse::<u32>().unwrap();
        }
    }

    calories_per_elf
}

pub fn problem1(input: &str) -> u32 {
    *calories_per_elf(input).iter().max().unwrap()
}

pub fn problem2(input: &str) -> u32 {
    let mut calories = calories_per_elf(input);
    calories.sort_unstable();
    calories.iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day1";

    #[test]
    fn problem1_example() {
        let example = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(problem1(&example), 24000);
    }

    #[test]
    fn problem1_real() {
        let data = std::fs::read_to_string(DATA_PATH).unwrap();
        assert_eq!(problem1(&data), 70509);
    }

    #[test]
    fn problem2_real() {
        let data = std::fs::read_to_string(DATA_PATH).unwrap();
        assert_eq!(problem2(&data), 208567);
    }
}