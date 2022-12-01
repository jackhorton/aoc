use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Token {
    LBracket,
    RBracket,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LAngle,
    RAngle,
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '<' => Ok(Token::LAngle),
            '>' => Ok(Token::RAngle),
            '{' => Ok(Token::LCurly),
            '}' => Ok(Token::RCurly),
            _ => Err(()),
        }
    }
}

impl Token {
    fn try_get_matching(&self) -> Option<Token> {
        match self {
            Token::LAngle => Some(Token::RAngle),
            Token::LBracket => Some(Token::RBracket),
            Token::LCurly => Some(Token::RCurly),
            Token::LParen => Some(Token::RParen),
            _ => None,
        }
    }

    fn is_left(&self) -> bool {
        self.try_get_matching().is_some()
    }
}

enum ParseError {
    Corrupted(Token),
    Incomplete(Vec<Token>),
}

fn parse_matching_end(
    expr: &[Token],
    expected_end: Token,
    next: &mut usize,
) -> Result<(), ParseError> {
    match expr.get(*next) {
        Some(&end) if end == expected_end => {
            *next += 1;
            Ok(())
        }
        Some(&end) if end.is_left() => {
            parse_matching(expr, next)?;
            parse_matching_end(expr, expected_end, next)
        }
        Some(&end) => Err(ParseError::Corrupted(end)),
        None => Err(ParseError::Incomplete(vec![])),
    }
}

fn parse_matching(expr: &[Token], next: &mut usize) -> Result<(), ParseError> {
    let left = expr[*next];
    let expected_right = left.try_get_matching().unwrap();
    *next += 1;
    parse_expr(expr, next)
        .and_then(|_| parse_matching_end(expr, expected_right, next))
        .map_err(|err| match err {
            ParseError::Incomplete(mut expected_list) => {
                expected_list.push(expected_right);
                ParseError::Incomplete(expected_list)
            }
            _ => err,
        })
}

fn parse_expr(expr: &[Token], next: &mut usize) -> Result<(), ParseError> {
    match expr.get(*next) {
        Some(&token) if token.is_left() => parse_matching(expr, next),
        _ => Ok(()),
    }
}

pub fn problem1(lines: Vec<Vec<Token>>) -> u32 {
    let error_values: HashMap<Token, u32> = [
        (Token::RAngle, 25137),
        (Token::RBracket, 57),
        (Token::RCurly, 1197),
        (Token::RParen, 3),
    ]
    .into();

    lines
        .iter()
        .filter_map(|line| match parse_expr(&line[..], &mut 0) {
            Ok(_) => None,
            Err(ParseError::Incomplete(_)) => None,
            Err(ParseError::Corrupted(actual_token)) => Some(actual_token),
        })
        .into_group_map_by(|&expected| expected)
        .iter()
        .map(|(key, value)| error_values[key] * value.len() as u32)
        .sum()
}

pub fn problem2(lines: Vec<Vec<Token>>) -> u64 {
    let mut incomplete_line_scores: Vec<u64> = lines
        .iter()
        .filter_map(|line| match parse_expr(&line[..], &mut 0) {
            Ok(_) => None,
            Err(ParseError::Corrupted(_)) => None,
            Err(ParseError::Incomplete(expected_tokens)) => {
                let score = expected_tokens.iter().fold(0u64, |cur_score, &token| {
                    let token_value = match token {
                        Token::RParen => 1,
                        Token::RBracket => 2,
                        Token::RCurly => 3,
                        Token::RAngle => 4,
                        _ => panic!(),
                    };
                    (cur_score * 5) + token_value
                });
                Some(score)
            }
        })
        .collect();
    incomplete_line_scores.sort();
    incomplete_line_scores[incomplete_line_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day10";

    fn parse_input(input: &str) -> Vec<Vec<Token>> {
        input
            .split('\n')
            .map(|line| line.trim().chars().map(|c| c.try_into().unwrap()).collect())
            .collect()
    }
    #[test]
    fn problem1_example() {
        let example = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let lines = parse_input(example);

        assert_eq!(problem1(lines), 26397);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let lines = parse_input(&content);
        assert_eq!(problem1(lines), 216297);
    }

    #[test]
    fn problem2_example() {
        let example = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let lines = parse_input(example);
        assert_eq!(problem2(lines), 288957);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let lines = parse_input(&content);
        assert_eq!(problem2(lines), 2165057169);
    }
}
