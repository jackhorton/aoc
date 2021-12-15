use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum CaveNode {
    Start,
    End,
    Big(u16),
    Small(u16),
}

fn format_base36(mut num: u16) -> String {
    let mut ret = Vec::new();
    while num > 0 {
        let digit = num % 36;
        num /= 36;

        ret.push(char::from_digit(digit as u32, 36).unwrap())
    }

    ret.iter().rev().collect()
}

impl Debug for CaveNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Big(id) => write!(f, "{}", format_base36(*id).to_ascii_uppercase()),
            Self::Small(id) => write!(f, "{}", format_base36(*id)),
        }
    }
}

impl From<&str> for CaveNode {
    fn from(s: &str) -> Self {
        match s.trim() {
            "start" => CaveNode::Start,
            "end" => CaveNode::End,
            v if v.chars().next().unwrap().is_ascii_lowercase() => {
                CaveNode::Small(u16::from_str_radix(v, 36).unwrap())
            }
            v => CaveNode::Big(u16::from_str_radix(v, 36).unwrap()),
        }
    }
}

#[derive(Debug)]
struct CavePath {
    path: Vec<CaveNode>,
    visited_small_nodes: HashSet<u16>,
    duplicate_small_node: Option<u16>,
}

impl CavePath {
    fn current(&self) -> &CaveNode {
        self.path.last().unwrap()
    }

    fn append(&self, next_node: &CaveNode, allow_duplicate_small_node: bool) -> Option<Self> {
        match next_node {
            CaveNode::Small(id)
                if self.visited_small_nodes.contains(id)
                    && allow_duplicate_small_node
                    && self.duplicate_small_node.is_none() =>
            {
                Some(Self {
                    path: self.path.iter().copied().chain([*next_node]).collect(),
                    visited_small_nodes: self
                        .visited_small_nodes
                        .iter()
                        .copied()
                        .chain([*id])
                        .collect(),
                    duplicate_small_node: Some(*id),
                })
            }
            CaveNode::Small(id) if !self.visited_small_nodes.contains(id) => Some(Self {
                path: self.path.iter().copied().chain([*next_node]).collect(),
                visited_small_nodes: self
                    .visited_small_nodes
                    .iter()
                    .copied()
                    .chain([*id])
                    .collect(),
                duplicate_small_node: self.duplicate_small_node,
            }),
            CaveNode::Big(_) | CaveNode::End => Some(Self {
                path: self.path.iter().copied().chain([*next_node]).collect(),
                visited_small_nodes: self.visited_small_nodes.clone(),
                duplicate_small_node: self.duplicate_small_node,
            }),
            _ => None,
        }
    }
}

pub fn count_paths(
    connections: &Vec<(CaveNode, CaveNode)>,
    allow_duplicate_small_node: bool,
) -> u32 {
    let mut paths = 0u32;
    let mut connections_map = HashMap::new();

    for (start, end) in connections {
        match *start {
            s if s == CaveNode::End || *end == CaveNode::Start => {
                connections_map
                    .entry(*end)
                    .or_insert(Vec::new())
                    .push(*start);
            }
            CaveNode::Big(_) | CaveNode::Small(_) if *end != CaveNode::End => {
                connections_map
                    .entry(*start)
                    .or_insert(Vec::new())
                    .push(*end);
                connections_map
                    .entry(*end)
                    .or_insert(Vec::new())
                    .push(*start);
            }
            _ => {
                connections_map
                    .entry(*start)
                    .or_insert(Vec::new())
                    .push(*end);
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back(CavePath {
        path: vec![CaveNode::Start],
        visited_small_nodes: HashSet::new(),
        duplicate_small_node: None,
    });

    while let Some(path) = q.pop_front() {
        match *path.current() {
            CaveNode::End => paths += 1,
            current if connections_map.contains_key(&current) => q.extend(
                connections_map[&current]
                    .iter()
                    .filter_map(|next_node| path.append(next_node, allow_duplicate_small_node)),
            ),
            _ => (),
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day12";
    const EXAMPLE_1: &'static str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const EXAMPLE_2: &'static str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    const EXAMPLE_3: &'static str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    fn parse_input(input: &str) -> Vec<(CaveNode, CaveNode)> {
        input
            .split('\n')
            .map(|line| {
                let mut split_line = line.split('-');
                let src = split_line.next().unwrap();
                let dest = split_line.next().unwrap();
                assert_eq!(split_line.next(), None);

                (src.into(), dest.into())
            })
            .collect()
    }

    #[test]
    fn problem1_example1() {
        assert_eq!(count_paths(&parse_input(EXAMPLE_1), false), 10);
    }

    #[test]
    fn problem1_example2() {
        assert_eq!(count_paths(&parse_input(EXAMPLE_2), false), 19);
    }

    #[test]
    fn problem1_example3() {
        assert_eq!(count_paths(&parse_input(EXAMPLE_3), false), 226);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let connections = parse_input(&content);
        assert_eq!(count_paths(&connections, false), 3563);
    }

    #[test]
    fn problem2_example1() {
        assert_eq!(count_paths(&parse_input(EXAMPLE_1), true), 36);
    }

    #[test]
    fn problem2_example2() {
        assert_eq!(count_paths(&parse_input(EXAMPLE_2), true), 103);
    }

    #[test]
    fn problem2_example3() {
        assert_eq!(count_paths(&parse_input(EXAMPLE_3), true), 3509);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let connections = parse_input(&content);
        assert_eq!(count_paths(&connections, true), 105453);
    }
}
