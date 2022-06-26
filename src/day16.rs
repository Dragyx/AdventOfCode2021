use std::fmt::{Binary, Debug};
use std::ops::{Add, AddAssign, Shl, ShlAssign};

use crate::helper::load_input_for_day;

#[derive(Debug)]
struct IsNotHexError;

fn hex_to_bin(c: char) -> Result<[bool; 4], IsNotHexError> {
    let [a, b, c, d] = match c {
        '0' => [0u8, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => Err(IsNotHexError)?,
    };
    Ok([a != 0, b != 0, c != 0, d != 0])
}

enum Packet {
    LiteralValue {
        version: u8,
        bits: Vec<bool>,
    },
    Operator {
        version: u8,
        packets: Vec<Box<Packet>>,
        length: LengthType,
    },
    TopLevel {
        packets: Vec<Box<Packet>>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LengthType {
    TotalInBits(u16),
    NumSubPackets(u16),
}

fn bits_to_num<T>(bits: impl Iterator<Item = bool>) -> T
where
    T: ShlAssign + AddAssign + Default + From<u8>,
{
    let mut num = T::default();
    bits.for_each(|b| {
        num <<= T::from(1u8);
        if b {
            num += T::from(1u8)
        }
    });
    num
}

#[derive(Debug)]
enum PacketError {
    Invalid,
    EmptyInput,
}

/// parses the bits for a packet.
///
/// if the packet contains other packets, the bits belonging to
/// the remaining packets will be returned as well
fn parse_packet(bit_iter: &mut impl Iterator<Item = bool>) -> Result<Packet, PacketError> {
    let first_three_bits: Vec<bool> = bit_iter.take(3).collect();
    if first_three_bits.len() != 3 {
        return Err(PacketError::EmptyInput);
    }
    let version: u8 = bits_to_num(first_three_bits.into_iter());
    dbg!(version);
    let type_id: u8 = bits_to_num(bit_iter.take(3));
    dbg!(type_id);
    #[allow(clippy::single_match)]
    match type_id {
        4 => {
            // literal packet
            let mut literal_bits = Vec::<bool>::new();
            let mut last_bit = false;
            while !last_bit {
                last_bit = !bit_iter.next().unwrap_or(false);
                println!("TEst");
                // not the last 5 bit segment
                literal_bits.extend(bit_iter.take(4))
            }
            Ok(Packet::LiteralValue {
                version,
                bits: literal_bits,
            })
        }
        _ => {
            // Operator
            let length_type_id: bool = bit_iter.next().ok_or(PacketError::Invalid)?;
            let length = match length_type_id {
                false => {
                    // the next 15 bits are the total
                    // length of bits of the subpackets
                    let len_subpackets: u16 = bits_to_num(bit_iter.take(15));
                    LengthType::TotalInBits(len_subpackets)
                }
                true => {
                    // the next 11 bits are the number of sub-packets
                    // contained by this packet
                    let num_packets: u16 = bits_to_num(bit_iter.take(11));
                    LengthType::NumSubPackets(num_packets)
                }
            };
            Ok(Packet::Operator {
                version,
                packets: Vec::new(),
                length,
            })
        }
    }
}

fn parse_protocol(mut stream: impl Iterator<Item = bool>) -> Vec<Packet> {
    // packet / subpacket-count pairs
    let mut packet_stack = vec![(
        Packet::TopLevel {
            packets: Vec::new(),
        },
        0u32,
    )];
    while let Ok(next_packet) = parse_packet(&mut stream) {
        // let is_operator = matches!(next_packet, Packet::Operator { .. });
        let mut packet_complete = false;
        if let Packet::Operator { length, .. } = next_packet {
            // the operator could have no children at all
            if let LengthType::NumSubPackets(0) = length {
                if let LengthType::TotalInBits(0) = length {
                    packet_complete = true;
                }
            }
        } else {
            packet_complete = true;
        }
        match packet_complete {
            true => {
                // traverse the stack in a reverse manner and collect
                // all children until an operator is reached
                let mut parent = packet_stack.last_mut().unwrap();

                // let mut collected_children = Vec::new();
                // let (last_packet, count): &mut (Packet, u32) = match packet_stack.last_mut() {
                //     Some(package_and_count) => package_and_count,
                //     None => { break; }
                // };
                // let index_of_parent_operator = packet_stack.iter().enumerate().rev().find(| (i, &(packet, count)) |  {
                //     match packet {
                //         Packet::Operator { .. } => {
                //             true
                //         }
                //         _ => false
                //     }
                // });
            }
            false => todo!(),
        }
        // search for the last operator
    }
    todo!()
}

pub fn run() {
    let input = load_input_for_day(16);
    let input_bits = input.chars().flat_map(|c| hex_to_bin(c).unwrap());
}

mod tests {
    use crate::day16::LengthType;

    #[allow(unused_imports)]
    use super::{hex_to_bin, parse_packet, Packet};

    #[allow(unused)]
    fn assert_eq_to_bitstring(bitstring: &str, bits: &Vec<bool>) {
        let mut all_equal = bitstring.len() == bits.len();
        for (i, bit_char) in bitstring.chars().enumerate() {
            if !all_equal {
                break;
            }
            let bit = bit_char.to_digit(2).unwrap() != 0;
            all_equal = bit == bits[i];
        }
        if !all_equal {
            let bitvec_str: String = bits
                .iter()
                .map(|bit| match bit {
                    true => '1',
                    false => '0',
                })
                .collect();
            panic!("Bitstring not correct: bitstring: {bitstring}, bitvec: {bitvec_str}")
        }
    }

    #[test]
    fn literal_packet() {
        let input = "D2FE28";
        let mut bits = input.chars().flat_map(|c| hex_to_bin(c).unwrap());
        let packet = parse_packet(&mut bits).unwrap();
        match packet {
            Packet::LiteralValue { version, bits } => {
                assert_eq!(version, 6);
                assert_eq_to_bitstring("011111100101", &bits)
            }
            _ => panic!("Expected a LiteralValue!"),
        }
    }

    #[test]
    fn operator_packet_len_bit() {
        let input = "38006F45291200";
        let mut bits = input.chars().flat_map(|c| hex_to_bin(c).unwrap());
        let packet = parse_packet(&mut bits).unwrap();
        match packet {
            Packet::Operator {
                version, length, ..
            } => {
                assert_eq!(version, 1);
                assert_eq!(length, LengthType::TotalInBits(27));
            }
            _ => panic!("Expected a LiteralValue!"),
        }
        let remaining_bits: Vec<bool> = bits.collect();
        assert_eq_to_bitstring("1101000101001010010001001000000000", &remaining_bits);
    }

    #[test]
    fn operator_packet_num_subpackets() {
        let input = "EE00D40C823060";
        let mut bits = input.chars().flat_map(|c| hex_to_bin(c).unwrap());
        let packet = parse_packet(&mut bits).unwrap();
        match packet {
            Packet::Operator {
                version, length, ..
            } => {
                assert_eq!(version, 7);
                assert_eq!(length, LengthType::NumSubPackets(3));
            }
            _ => panic!("Expected a LiteralValue!"),
        }
        let remaining_bits: Vec<bool> = bits.collect();
        assert_eq_to_bitstring("01010000001100100000100011000001100000", &remaining_bits);
    }
}
