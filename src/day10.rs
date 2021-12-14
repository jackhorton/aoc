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

enum ParseError {
    Corrupted(Token),
    Incomplete(Vec<Token>),
}

fn map_parse_error(expected: Token) -> impl Fn(ParseError) -> ParseError {
    move |err| match err {
        ParseError::Corrupted(_) => err,
        ParseError::Incomplete(mut expected_list) => {
            expected_list.push(expected);
            ParseError::Incomplete(expected_list)
        }
    }
}

fn parse_matching_end(
    expr: &[Token],
    expected_end: Token,
    matching_tokens: &HashMap<Token, Token>,
    next: &mut usize,
) -> Result<(), ParseError> {
    match expr.get(*next) {
        Some(&right_actual) if right_actual == expected_end => {
            *next += 1;
            Ok(())
        }
        Some(right_actual) if matching_tokens.contains_key(right_actual) => {
            parse_matching(expr, matching_tokens, next)?;
            parse_matching_end(expr, expected_end, matching_tokens, next)
        }
        Some(&right_actual) => Err(ParseError::Corrupted(right_actual)),
        None => Err(ParseError::Incomplete(vec![])),
    }
}

fn parse_matching(
    expr: &[Token],
    matching_tokens: &HashMap<Token, Token>,
    next: &mut usize,
) -> Result<(), ParseError> {
    let left = expr[*next];
    *next += 1;
    parse_expr(expr, matching_tokens, next).map_err(map_parse_error(matching_tokens[&left]))?;
    parse_matching_end(expr, matching_tokens[&left], matching_tokens, next)
        .map_err(map_parse_error(matching_tokens[&left]))
}

fn parse_expr(
    expr: &[Token],
    matching_tokens: &HashMap<Token, Token>,
    next: &mut usize,
) -> Result<(), ParseError> {
    match expr.get(*next) {
        None => Ok(()),
        Some(&token) => match token {
            Token::LAngle | Token::LBracket | Token::LCurly | Token::LParen => {
                parse_matching(expr, &matching_tokens, next)
            }
            _ => Ok(()),
        },
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
    let matching_tokens: HashMap<Token, Token> = [
        (Token::LAngle, Token::RAngle),
        (Token::LBracket, Token::RBracket),
        (Token::LCurly, Token::RCurly),
        (Token::LParen, Token::RParen),
    ]
    .into();

    lines
        .iter()
        .filter_map(
            |line| match parse_expr(&line[..], &matching_tokens, &mut 0) {
                Ok(_) => None,
                Err(ParseError::Incomplete(_)) => None,
                Err(ParseError::Corrupted(actual_token)) => Some(actual_token),
            },
        )
        .into_group_map_by(|&expected| expected)
        .iter()
        .map(|(key, value)| error_values[key] * value.len() as u32)
        .sum()
}

pub fn problem2(lines: Vec<Vec<Token>>) -> u64 {
    let matching_tokens: HashMap<Token, Token> = [
        (Token::LAngle, Token::RAngle),
        (Token::LBracket, Token::RBracket),
        (Token::LCurly, Token::RCurly),
        (Token::LParen, Token::RParen),
    ]
    .into();

    let mut incomplete_line_scores: Vec<u64> = lines
        .iter()
        .filter_map(
            |line| match parse_expr(&line[..], &matching_tokens, &mut 0) {
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
            },
        )
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
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '[' => Token::LBracket,
                        ']' => Token::RBracket,
                        '(' => Token::LParen,
                        ')' => Token::RParen,
                        '<' => Token::LAngle,
                        '>' => Token::RAngle,
                        '{' => Token::LCurly,
                        '}' => Token::RCurly,
                        _ => panic!(),
                    })
                    .collect()
            })
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
