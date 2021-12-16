use itertools::Itertools;

fn to_number(bits: &[u8]) -> u64 {
	bits.iter().fold(0, |a, b| a << 1 | (*b as u64))
}

struct BitReader<'a> {
	bits: &'a Vec<u8>,
	pos: usize,
}

impl<'a> BitReader<'a> {
	fn read_bits(&mut self, n: usize) -> &'a [u8] {
		self.pos += n;
		&self.bits[self.pos - n..self.pos]
	}

	fn read_number(&mut self, n: usize) -> u64 {
		to_number(self.read_bits(n))
	}

	fn read_literal(&mut self) -> u64 {
		let mut result = 0;
		loop {
			let chunk = self.read_bits(5);
			result = result << 4 | to_number(&chunk[1..]);
			if chunk[0] == 0 {
				break;
			}
		}
		return result;	
	}
}

#[derive(Debug)]
enum Payload {
	Literal(u64),
	Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
	version: u64,
	ptype: u64,
	payload: Payload,
}

fn read_packet(bits: &mut BitReader) -> Packet {
	let version = bits.read_number(3);
	let ptype = bits.read_number(3);

	if ptype == 4 {
		let literal = bits.read_literal();
		return Packet{ version, ptype, payload: Payload::Literal(literal)};
	} else {
		// Operator
		let length_type = bits.read_bits(1);
		let mut packets = Vec::new();
		if length_type[0] == 0 {
			let length_in_bits = bits.read_number(15);
			let buffer_end_pos = bits.pos + length_in_bits as usize;
			while bits.pos < buffer_end_pos {
				packets.push(read_packet(bits));
			}
		} else {
			let num_sub_packets = bits.read_number(11);
			for _ in 0..num_sub_packets {
				packets.push(read_packet(bits));
			}
		};
		return Packet{version, ptype, payload: Payload::Operator(packets)};
	}
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
	let bits = inputs[0].chars()
		.flat_map(|c| {
			let n = c.to_digit(16).unwrap() as u8;
			[n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]
		}).collect_vec();

	let mut reader = BitReader{ bits: &bits, pos: 0 };

	let packet = read_packet(&mut reader);
	println!("Part 1: {}", sum_versions(&packet));
	println!("Part 2: {}", packet_value(&packet));
}