use utils::read_lines;

#[derive(Clone, Debug)]
struct Packet {
    val: Option<u32>,
    packets: Vec<Packet>,
}

fn build_packet(input: &Vec<char>, index_in_input: &mut usize) -> Packet {
    let mut packets = Vec::new();
    while *index_in_input < input.len() - 1 {
        *index_in_input += 1;
        match input[*index_in_input] {
            '[' => {
                packets.push(build_packet(input, index_in_input));
            }
            ']' => {
                return Packet { val: None, packets };
            }
            ',' => {}
            _ => {
                let mut next_non_number_index = 0;
                for i in (*index_in_input + 1)..input.len() {
                    if !input[i].is_numeric() {
                        next_non_number_index = i;
                        break;
                    }
                }
                let num = &input[*index_in_input..next_non_number_index];
                let parsed_num = num.iter().collect::<String>().parse().unwrap();
                packets.push(Packet {
                    val: Some(parsed_num),
                    packets: Vec::new(),
                });
                *index_in_input = next_non_number_index - 1;
            }
        }
    }

    panic!();
}

fn are_packets_ordered(a: &Packet, b: &Packet) -> Option<bool> {
    let mut a_index = 0;
    let mut b_index = 0;
    while a_index < a.packets.len() && b_index < b.packets.len() {
        let a_elem = &a.packets[a_index];
        let b_elem = &b.packets[b_index];

        let mut ordered: Option<bool> = None;

        if let Some(a_val) = a_elem.val {
            if let Some(b_val) = b_elem.val {
                // both numbers
                if a_val < b_val {
                    ordered = Some(true);
                } else if a_val > b_val {
                    ordered = Some(false);
                }
            } else {
                ordered = are_packets_ordered(
                    &Packet {
                        val: None,
                        packets: vec![a_elem.clone()],
                    },
                    b_elem,
                );
            }
        } else if b_elem.val.is_some() {
            ordered = are_packets_ordered(
                a_elem,
                &Packet {
                    val: None,
                    packets: vec![b_elem.clone()],
                },
            );
        } else {
            ordered = are_packets_ordered(a_elem, b_elem);
        }

        if ordered.is_some() {
            return ordered;
        }

        a_index += 1;
        b_index += 1;
    }

    if a_index == a.packets.len() {
        if b_index == b.packets.len() {
            // both packets same length. need to look further in parent packets
            return None;
        } else {
            // a is shorter than b. packets is ordered
            return Some(true);
        }
    } else {
        return Some(false);
    }
}

fn part_a() {
    let ordered_index_sum: usize = read_lines("./day_13/input.txt")
        .filter(|line| line != "")
        .map(|line| build_packet(&line.chars().collect(), &mut 0))
        .collect::<Vec<Packet>>()
        .chunks(2)
        .enumerate()
        .fold(0, |acc, (index, packets)| {
            if are_packets_ordered(&packets[0], &packets[1]).unwrap() {
                acc + index + 1
            } else {
                acc
            }
        });

    println!("{}", ordered_index_sum);
}

fn main() {
    part_a();
}
