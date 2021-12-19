use std::fs;

#[derive(Clone)]
struct Packet {
    version: u8,
    mode: PacketType,
}

#[derive(Clone)]
enum PacketType {
    Literal(u64),
    Operator(Vec<Packet>, u8),
}

fn main() {
    let input = fs::read_to_string("input/day_16.txt").expect("Unable to read input file");
    let input = input.trim();
    let mut parsed_input: Vec<bool> = Vec::with_capacity(input.len() * 4);
    for char in input.chars() {
        let parsed = char.to_digit(16).expect("Invalid input");
        for i in (0..4).rev() {
            parsed_input.push(((parsed >> i) & 1) == 1);
        }
    }

    // Parsing packets
    let (packets, _) = read_packets(&parsed_input, 0);

    // Part 1
    let first = packets[0].clone();
    println!("{}", get_packet_versions(packets));
    // Part 2
    println!("{}", process_operator(first));
}

fn read_packets(raw_data: &[bool], max_packets: u64) -> (Vec<Packet>, usize) {
    let mut packets = Vec::new();

    let mut packets_read = 0;
    let mut pos = 0;
    // Keep looping until there isn't enough left to be valid
    while raw_data.len() > pos + 8 && (max_packets == 0 || max_packets > packets_read) {
        let version = slice_to_num(&raw_data[pos..(pos + 3)]) as u8;
        let type_id = slice_to_num(&raw_data[(pos + 3)..(pos + 6)]) as u8;
        if type_id == 4 {
            // Literal type
            // Literals have a fixed length of 3 + 3 + 15
            // Looking for the last literal bit
            // Moving pos forward 6 to rest on the first bit
            let mut digits = Vec::new();
            pos += 6;
            while raw_data[pos] {
                for i in 1..=4 {
                    digits.push(raw_data[pos + i]);
                }
                pos += 5;
            }
            for i in 1..=4 {
                digits.push(raw_data[pos + i]);
            }
            pos += 5;
            packets.push(Packet {
                version,
                mode: PacketType::Literal(slice_to_num(digits.as_slice())),
            });
        } else {
            // Operator type
            if raw_data[pos + 6] {
                // Next 11 bits indicate the number of sub packets
                let sub_num = slice_to_num(&raw_data[(pos + 7)..(pos + 18)]);
                let (sub_packets, dist) = read_packets(&raw_data[(pos + 18)..], sub_num);
                packets.push(Packet {
                    version,
                    mode: PacketType::Operator(sub_packets, type_id),
                });
                pos += 18 + dist;
            } else {
                // Next 15 bits indicate the number of bits in the sub-packets
                let sub_len = slice_to_num(&raw_data[(pos + 7)..(pos + 22)]);
                let (sub_packets, _) =
                    read_packets(&raw_data[(pos + 22)..(pos + 22 + sub_len as usize)], 0);
                packets.push(Packet {
                    version,
                    mode: PacketType::Operator(sub_packets, type_id),
                });
                pos += 22 + sub_len as usize;
            }
        }

        packets_read += 1;
    }

    (packets, pos)
}

fn slice_to_num(slice: &[bool]) -> u64 {
    let max = slice.len();
    slice
        .iter()
        .enumerate()
        .map(|(i, val)| 2u64.pow((max - i - 1) as u32) * *val as u64)
        .sum()
}

fn get_packet_versions(packets: Vec<Packet>) -> u64 {
    packets
        .into_iter()
        .map(|pack| {
            let mut total = pack.version as u64;
            if let PacketType::Operator(subs, _) = pack.mode {
                total += get_packet_versions(subs);
            }
            total
        })
        .sum()
}

fn process_operator(operator: Packet) -> u64 {
    // Processes an operator.
    match operator.mode {
        PacketType::Literal(num) => num,
        PacketType::Operator(sub_vals, opcode) => {
            match opcode {
                0 => {
                    // Sum
                    sub_vals.iter().cloned().map(process_operator).sum()
                }
                1 => {
                    // Product
                    sub_vals.iter().cloned().map(process_operator).product()
                }
                2 => {
                    // Minimum
                    sub_vals
                        .iter()
                        .cloned()
                        .map(process_operator)
                        .min()
                        .unwrap()
                }
                3 => {
                    // Maximum
                    sub_vals
                        .iter()
                        .cloned()
                        .map(process_operator)
                        .max()
                        .unwrap()
                }
                5 => {
                    // Greater than
                    (process_operator(sub_vals[0].clone()) > process_operator(sub_vals[1].clone()))
                        as u64
                }
                6 => {
                    // Less than
                    (process_operator(sub_vals[0].clone()) < process_operator(sub_vals[1].clone()))
                        as u64
                }
                7 => {
                    // Equal to
                    (process_operator(sub_vals[0].clone()) == process_operator(sub_vals[1].clone()))
                        as u64
                }
                x => panic!("Unrecognized opcode: {}", x),
            }
        }
    }
}
