use itertools::Itertools;

fn read_bits_opt<'a>(bits: &mut impl Iterator<Item = &'a char>, mut n: usize) -> Option<u64> {
	let chunk = bits.take(n).collect_vec();

	if chunk.len() == 0 {
		return None
	}

	let mut result: u64 = 0;
	for ch in &chunk {
		if *ch == &'1' {
			result += 1 << (n -1 );
		}
		n -= 1;
	}

	return Some(result);
}

fn read_bits<'a>(bits: &mut impl Iterator<Item = &'a char>, mut n: usize) -> u64 {
	let chunk = bits.take(n);
	let mut result: u64 = 0;
	for ch in chunk {
		if ch == &'1' {
			result += 1 << (n -1 );
		}
		n -= 1;
	}

	return result;
}

fn read_literal<'a>(mut bits: &mut impl Iterator<Item = &'a char>) -> u64 {
	let mut result: u64 = 0;

	loop {
		result <<= 4;
		let prefix = bits.next().unwrap();
		let num = read_bits(&mut bits, 4);

		result += num;

		if prefix == &'0' {
			break;
		}
	}

	return result;
}

#[derive(Debug)]
enum Payload {
	Literal(u64),
	Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
	version: u8,
	ptype: u8,
	payload: Payload,
}

fn read_packet<'a>(mut bits: &mut impl Iterator<Item = &'a char>) -> Option<Packet> {
	let version = read_bits_opt(&mut bits, 3);
	if version.is_none() {
		return None;
	}

	let version = version.unwrap() as u8;
	let ptype = read_bits(&mut bits, 3) as u8;

	if ptype == 4 {
		let literal = read_literal(&mut bits);
		return Some(Packet{ version, ptype, payload: Payload::Literal(literal)});
	} else {
		// Operator
		let length_type = bits.next().unwrap();
		let packets = if length_type == &'0' {
			let length_in_bits = read_bits(&mut bits, 15);
			let sub_bits = bits.take(length_in_bits as usize).cloned().collect_vec();
			read_packets(&mut sub_bits.iter())
		} else {
			let num_sub_packets = read_bits(&mut bits, 11);
			read_packets_n(bits, num_sub_packets as usize)
		};
		return Some(Packet{version, ptype, payload: Payload::Operator(packets)});
	}
}

fn read_packets<'a>(bits: &mut impl Iterator<Item = &'a char>) -> Vec<Packet> {
	let mut result = Vec::new();
	loop {
		if let Some(packet) = read_packet(bits) {
			result.push(packet);
		} else {
			break;
		}
	}

	return result;
}

fn read_packets_n<'a>(bits: &mut impl Iterator<Item = &'a char>, num: usize) -> Vec<Packet> {
	let mut result = Vec::new();
	for _ in 0..num {
		result.push(read_packet(bits).unwrap());
	}

	return result;
}

fn sum_versions(packet: &Packet) -> usize {
	return (packet.version as usize) +
	match &packet.payload {
		Payload::Operator(sub_packets) => sub_packets.iter().map(|p| sum_versions(p)).sum::<usize>(),
		Payload::Literal(_) => 0,
	}
}

fn packet_value(packet: &Packet) -> u64 {
	match &packet.payload {
		Payload::Literal(val) => *val,
		Payload::Operator(sub_packets) => {
			let mut sub_iter = sub_packets.iter().map(|p| packet_value(p));
			match packet.ptype {
				0 => sub_iter.sum::<u64>(),
				1 => sub_iter.product::<u64>(),
				2 => sub_iter.min().unwrap(),
				3 => sub_iter.max().unwrap(),
				5 | 6 | 7 => {
					let p1 = sub_iter.next().unwrap();
					let p2 = sub_iter.next().unwrap();
					let condition = match packet.ptype {
						5 => p1 > p2,
						6 => p1 < p2,
						7 => p1 == p2,
						_ => unreachable!(),
					};
					if condition { 1 } else { 0 }
				},
				_ => unreachable!("Unexpected type = {}", packet.ptype),
			}
		}
	}

}

pub fn solve(inputs: Vec<String>) {
	let mut bits: Vec<char> = Vec::new();

	for ch in inputs[0].chars() {
		let digit = match ch {
			'0'..='9' => format!("{:04b}", ch as u8 - '0' as u8),
			'A'..='F' => format!("{:04b}", ch as u8 - 'A' as u8 + 10),
			_ => unreachable!(),
		}.chars().collect_vec();

		bits.extend(digit);
	}

	let mut iter = bits.iter();
	let packet = read_packet(&mut iter).unwrap();

	println!("Part 1: {}", sum_versions(&packet));
	println!("Part 2: {}", packet_value(&packet));
}