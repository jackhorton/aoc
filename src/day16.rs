#[derive(Clone)]
enum PacketNode {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    GreaterThan(Box<Packet>, Box<Packet>),
    LessThan(Box<Packet>, Box<Packet>),
    Equals(Box<Packet>, Box<Packet>),
}

#[derive(Clone)]
pub struct Packet {
    node: PacketNode,
    version: u64,
}

pub struct ParserState<'a> {
    current: u8,
    current_bits: u8,
    rest: &'a str,
    bit_position: usize,
}

impl ParserState<'_> {
    fn take_bits(&mut self, requested_bits: u8) -> u64 {
        assert!(self.current < 0x10);
        assert!(self.current_bits <= 4 && self.current_bits > 0);
        assert!(requested_bits > 0);

        if requested_bits < self.current_bits {
            let all_mask = 0xF;
            let unrequested_count = self.current_bits - requested_bits;
            let unrequested_mask = (1u8 << unrequested_count) - 1;
            let mask = all_mask ^ unrequested_mask;
            let bits = self.current & mask;

            self.current_bits -= requested_bits;
            self.current &= unrequested_mask;
            self.bit_position += requested_bits as usize;

            bits as u64 >> unrequested_count
        } else if requested_bits == self.current_bits {
            let bits = self.current;

            // We are taking a char to clear out |current|, but we don't
            // actually need to make sure that it worked -- if |requested_bits|
            // is 4 and we have the last four of the string in |current|, then
            // we are at the end of the string and can't take any more characters.
            // Thats fine, as long as |take_bits| is never called again.
            self.take_char().unwrap_or_default();
            self.bit_position += requested_bits as usize;

            bits.into()
        } else {
            let mut remaining_bits = requested_bits - self.current_bits;
            let mut bits = (self.current as u64) << remaining_bits as u64;
            self.bit_position += self.current_bits as usize;

            self.take_char().unwrap();

            while remaining_bits > 0 {
                let remaining_bit_chunk = remaining_bits.min(4);
                remaining_bits -= remaining_bit_chunk;

                let chunk_value = self.take_bits(remaining_bit_chunk);
                bits += chunk_value << remaining_bits;
            }

            bits
        }
    }

    pub fn new<'a>(input: &'a str) -> ParserState<'a> {
        let mut state = ParserState {
            current: 0,
            current_bits: 0,
            rest: input,
            bit_position: 0,
        };
        state.take_char().unwrap();
        state
    }

    fn take_char(&mut self) -> Result<char, ()> {
        if self.rest.len() == 0 {
            return Err(());
        }

        self.current_bits = 4;
        self.current = u8::from_str_radix(&self.rest[0..1], 16).unwrap();
        let ret = self.rest.chars().next().unwrap();

        if self.rest.len() > 1 {
            self.rest = &self.rest[1..];
        } else {
            self.rest = "";
        }

        Ok(ret)
    }
}

fn parse_literal(state: &mut ParserState) -> PacketNode {
    const MASK: u64 = 0xF;
    let mut literal = 0u64;
    loop {
        let segment = state.take_bits(5);
        let segment_value = segment & MASK;
        literal <<= 4;
        literal += segment_value;
        if segment <= MASK {
            break;
        }
    }

    PacketNode::Literal(literal)
}

fn parse_subpackets_with_bit_length(state: &mut ParserState, bit_length: u16) -> Vec<Packet> {
    let final_position = state.bit_position + bit_length as usize;
    let mut subpackets = Vec::new();
    while state.bit_position < final_position {
        subpackets.push(parse_packet(state));
    }

    assert_eq!(state.bit_position, final_position);
    subpackets
}

fn parse_subpackets_with_count(state: &mut ParserState, packet_count: u16) -> Vec<Packet> {
    let mut subpackets = Vec::new();
    for _ in 0..packet_count {
        subpackets.push(parse_packet(state));
    }
    subpackets
}

fn parse_packet(state: &mut ParserState) -> Packet {
    let version = state.take_bits(3);
    let type_id = state.take_bits(3);

    let node = match type_id {
        0b100 => parse_literal(state),
        _ => {
            let length_type_id = state.take_bits(1);
            let subpackets = match length_type_id {
                0 => {
                    let bit_length = state.take_bits(15) as u16;
                    parse_subpackets_with_bit_length(state, bit_length)
                }
                _ => {
                    let packet_count = state.take_bits(11) as u16;
                    parse_subpackets_with_count(state, packet_count)
                }
            };

            match type_id {
                0 => PacketNode::Sum(subpackets),
                1 => PacketNode::Product(subpackets),
                2 => PacketNode::Minimum(subpackets),
                3 => PacketNode::Maximum(subpackets),
                5 => {
                    assert_eq!(subpackets.len(), 2);
                    PacketNode::GreaterThan(
                        Box::new(subpackets[0].clone()),
                        Box::new(subpackets[1].clone()),
                    )
                }
                6 => {
                    assert_eq!(subpackets.len(), 2);
                    PacketNode::LessThan(
                        Box::new(subpackets[0].clone()),
                        Box::new(subpackets[1].clone()),
                    )
                }
                7 => {
                    assert_eq!(subpackets.len(), 2);
                    PacketNode::Equals(
                        Box::new(subpackets[0].clone()),
                        Box::new(subpackets[1].clone()),
                    )
                }
                _ => panic!("Invalid type ID {}", type_id),
            }
        }
    };

    Packet { node, version }
}

fn sum_versions(packet: &Packet) -> u64 {
    let mut sum = packet.version;
    match &packet.node {
        PacketNode::Sum(subpackets)
        | PacketNode::Product(subpackets)
        | PacketNode::Minimum(subpackets)
        | PacketNode::Maximum(subpackets) => {
            for subpacket in subpackets.iter() {
                sum += sum_versions(subpacket);
            }
        }
        PacketNode::GreaterThan(left, right)
        | PacketNode::LessThan(left, right)
        | PacketNode::Equals(left, right) => {
            sum += sum_versions(left);
            sum += sum_versions(right);
        }
        _ => (),
    }
    sum
}

pub fn problem1(state: &mut ParserState) -> u64 {
    let root = parse_packet(state);
    sum_versions(&root)
}

fn evaluate_packet(packet: &Packet) -> u64 {
    match &packet.node {
        PacketNode::Sum(subpackets) => subpackets.iter().map(evaluate_packet).sum(),
        PacketNode::Product(subpackets) => subpackets.iter().map(evaluate_packet).product(),
        PacketNode::Minimum(subpackets) => subpackets.iter().map(evaluate_packet).min().unwrap(),
        PacketNode::Maximum(subpackets) => subpackets.iter().map(evaluate_packet).max().unwrap(),
        PacketNode::Literal(literal) => *literal,
        PacketNode::LessThan(left, right) => {
            if evaluate_packet(left) < evaluate_packet(right) {
                1
            } else {
                0
            }
        }
        PacketNode::GreaterThan(left, right) => {
            if evaluate_packet(left) > evaluate_packet(right) {
                1
            } else {
                0
            }
        }
        PacketNode::Equals(left, right) => {
            if evaluate_packet(left) == evaluate_packet(right) {
                1
            } else {
                0
            }
        }
    }
}

pub fn problem2(state: &mut ParserState) -> u64 {
    let root = parse_packet(state);
    evaluate_packet(&root)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day16";

    #[test]
    fn problem1_example1() {
        let mut state = ParserState::new("38006F45291200");
        assert_eq!(problem1(&mut state), 9);
    }

    #[test]
    fn problem1_example2() {
        let mut state = ParserState::new("EE00D40C823060");
        assert_eq!(problem1(&mut state), 14);
    }

    #[test]
    fn problem1_example3() {
        let mut state = ParserState::new("8A004A801A8002F478");
        assert_eq!(problem1(&mut state), 16);
    }

    #[test]
    fn problem1_example4() {
        let mut state = ParserState::new("C0015000016115A2E0802F182340");
        assert_eq!(problem1(&mut state), 23);
    }

    #[test]
    fn problem1_example5() {
        let mut state = ParserState::new("620080001611562C8802118E34");
        assert_eq!(problem1(&mut state), 12);
    }

    #[test]
    fn problem1_example6() {
        let mut state = ParserState::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(problem1(&mut state), 31);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let mut state = ParserState::new(&content);
        assert_eq!(problem1(&mut state), 897);
    }

    #[test]
    fn problem2_example1() {
        let mut state = ParserState::new("C200B40A82");
        assert_eq!(problem2(&mut state), 3);
    }

    #[test]
    fn problem2_example2() {
        let mut state = ParserState::new("04005AC33890");
        assert_eq!(problem2(&mut state), 54);
    }

    #[test]
    fn problem2_example3() {
        let mut state = ParserState::new("880086C3E88112");
        assert_eq!(problem2(&mut state), 7);
    }

    #[test]
    fn problem2_example4() {
        let mut state = ParserState::new("CE00C43D881120");
        assert_eq!(problem2(&mut state), 9);
    }

    #[test]
    fn problem2_example5() {
        let mut state = ParserState::new("D8005AC2A8F0");
        assert_eq!(problem2(&mut state), 1);
    }

    #[test]
    fn problem2_example6() {
        let mut state = ParserState::new("F600BC2D8F");
        assert_eq!(problem2(&mut state), 0);
    }

    #[test]
    fn problem2_example7() {
        let mut state = ParserState::new("9C005AC2F8F0");
        assert_eq!(problem2(&mut state), 0);
    }

    #[test]
    fn problem2_example8() {
        let mut state = ParserState::new("9C0141080250320F1802104A08");
        assert_eq!(problem2(&mut state), 1);
    }

    #[test]
    fn problem2_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let mut state = ParserState::new(&content);
        assert_eq!(problem2(&mut state), 9485076995911);
    }
}
