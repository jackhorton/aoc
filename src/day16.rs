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

            self.take_char();
            self.bit_position += requested_bits as usize;

            bits.into()
        } else {
            let mut remaining_bits = requested_bits - self.current_bits;
            let mut bits = (self.current as u64) << remaining_bits as u64;
            self.bit_position += self.current_bits as usize;

            self.take_char();

            while remaining_bits > 0 {
                let remaining_bit_chunk = remaining_bits.min(4);
                remaining_bits -= remaining_bit_chunk;

                // if remaining_bit_chunk < 4 {
                //     // if we have less than 4 bits remaining, we are on our last segment. We need
                //     // to shift |bits| to the right to account for the fact that we will be getting
                //     // effectively a shorter segment in return from the next call to |take_bits|.
                //     bits >>= 4 - remaining_bit_chunk;
                // }

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
        state.take_char();
        state
    }

    fn take_char(&mut self) {
        assert!(self.rest.len() > 0);

        self.current_bits = 4;
        self.current = u8::from_str_radix(&self.rest[0..1], 16).unwrap();

        if self.rest.len() > 1 {
            self.rest = &self.rest[1..];
        } else {
            self.rest = "";
        }
    }
}

fn parse_literal(state: &mut ParserState) -> u64 {
    let packet = state.take_bits(5);
    if packet & 0x10 > 0 {
        parse_literal(state)
    } else {
        0
    }
}

fn parse_subpackets_with_bit_length(state: &mut ParserState, bit_length: u16) -> u64 {
    let final_position = state.bit_position + bit_length as usize;
    let mut version_sum = 0u64;
    while state.bit_position < final_position {
        version_sum += parse_packet(state);
    }

    assert_eq!(state.bit_position, final_position);
    version_sum
}

fn parse_subpackets_with_count(state: &mut ParserState, packet_count: u16) -> u64 {
    let mut version_sum = 0u64;
    for _ in 0..packet_count {
        version_sum += parse_packet(state);
    }
    version_sum
}

fn parse_operator(state: &mut ParserState) -> u64 {
    let length_type_id = state.take_bits(1);
    match length_type_id {
        0 => {
            let bit_length = state.take_bits(15) as u16;
            parse_subpackets_with_bit_length(state, bit_length)
        }
        _ => {
            let packet_count = state.take_bits(11) as u16;
            parse_subpackets_with_count(state, packet_count)
        }
    }
}

pub fn parse_packet(state: &mut ParserState) -> u64 {
    let version = state.take_bits(3);
    let type_id = state.take_bits(3);

    version
        + match type_id {
            0b100 => parse_literal(state),
            _ => parse_operator(state),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &'static str = "data/day16";

    #[test]
    fn problem1_example1() {
        let mut state = ParserState::new("38006F45291200");
        assert_eq!(parse_packet(&mut state), 9);
    }

    #[test]
    fn problem1_example2() {
        let mut state = ParserState::new("EE00D40C823060");
        assert_eq!(parse_packet(&mut state), 14);
    }

    #[test]
    fn problem1_example3() {
        let mut state = ParserState::new("8A004A801A8002F478");
        assert_eq!(parse_packet(&mut state), 16);
    }

    #[test]
    fn problem1_example4() {
        let mut state = ParserState::new("C0015000016115A2E0802F182340");
        assert_eq!(parse_packet(&mut state), 23);
    }

    #[test]
    fn problem1_example5() {
        let mut state = ParserState::new("620080001611562C8802118E34");
        assert_eq!(parse_packet(&mut state), 12);
    }

    #[test]
    fn problem1_example6() {
        let mut state = ParserState::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(parse_packet(&mut state), 31);
    }

    #[test]
    fn problem1_real() {
        let content = std::fs::read_to_string(DATA_PATH).unwrap();
        let mut state = ParserState::new(&content);
        assert_eq!(parse_packet(&mut state), 897);
    }
}
