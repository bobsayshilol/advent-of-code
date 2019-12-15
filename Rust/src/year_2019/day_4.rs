/**
 * Part 1:
 *
 * However, they do remember a few key facts about the password:
 *    It is a six-digit number.
 *    The value is within the range given in your puzzle input.
 *    Two adjacent digits are the same (like 22 in 122345).
 *    Going from left to right, the digits never decrease; they only ever
 *    increase or stay the same (like 111123 or 135679).
 *
 * Other than the range rule, the following are true:
 *    111111 meets these criteria (double 11, never decreases).
 *    223450 does not meet these criteria (decreasing pair of digits 50).
 *    123789 does not meet these criteria (no double).
 *
 * How many different passwords within the range given in your puzzle input
 * meet these criteria?
 */

use std::error::Error;



#[derive(Copy, Clone)]
struct Password(usize, usize, usize, usize, usize, usize);

impl Password {
	fn from_int(i :usize) -> Password {
		let mut pw = Password (
			(i / 100000) % 10,
			(i / 10000) % 10,
			(i / 1000) % 10,
			(i / 100) % 10,
			(i / 10) % 10,
			(i / 1) % 10,
		);
		
		// Round up to next valid one
		// TODO: how can this be deduplicated?
		if pw.0 > pw.1 { pw.1 = pw.0; pw.2 = pw.1; pw.3 = pw.2; pw.4 = pw.3; pw.5 = pw.4; }
		if pw.1 > pw.2 { pw.2 = pw.1; pw.3 = pw.2; pw.4 = pw.3; pw.5 = pw.4; }
		if pw.2 > pw.3 { pw.3 = pw.2; pw.4 = pw.3; pw.5 = pw.4; }
		if pw.3 > pw.4 { pw.4 = pw.3; pw.5 = pw.4; }
		if pw.4 > pw.5 { pw.5 = pw.4; }
		
		return pw;
	}
	
	fn as_int(&self) -> usize {
		let v = self.0;
		let v = v * 10 + self.1;
		let v = v * 10 + self.2;
		let v = v * 10 + self.3;
		let v = v * 10 + self.4;
		let v = v * 10 + self.5;
		return v;
	}
	
	// Increment to the next valid non-decreasing pw
	fn increment(&self) -> Option<Password> {
		let mut copy = *self;
		
		// Add 1 from the bottom
		copy.5 += 1;
		if copy.5 == 10 { copy.5 = 0; copy.4 += 1; }
		if copy.4 == 10 { copy.4 = 0; copy.3 += 1; }
		if copy.3 == 10 { copy.3 = 0; copy.2 += 1; }
		if copy.2 == 10 { copy.2 = 0; copy.1 += 1; }
		if copy.1 == 10 { copy.1 = 0; copy.0 += 1; }
		if copy.0 == 10 {
			return None;
		}
		
		// Jump over any decreasing
		if copy.0 > copy.1 { copy.1 = copy.0; }
		if copy.1 > copy.2 { copy.2 = copy.1; }
		if copy.2 > copy.3 { copy.3 = copy.2; }
		if copy.3 > copy.4 { copy.4 = copy.3; }
		if copy.4 > copy.5 { copy.5 = copy.4; }
		
		return Some(copy);
	}
}


fn has_adjacents(pw :Password) -> bool {
	return pw.0 == pw.1 || pw.1 == pw.2 || pw.2 == pw.3 || pw.3 == pw.4 || pw.4 == pw.5;
}


fn has_part2_adjacents(pw :Password) -> bool {
	// Brute force check of each pair and their surroundings
	fn check_quad(a :usize, b :usize, c :usize, d :usize) -> bool {
		return b == c && a != b && c != d;
	};
	// A value can never be 10, so use that for the edges
	return
		check_quad(10, pw.0, pw.1, pw.2) ||
		check_quad(pw.0, pw.1, pw.2, pw.3) ||
		check_quad(pw.1, pw.2, pw.3, pw.4) ||
		check_quad(pw.2, pw.3, pw.4, pw.5) ||
		check_quad(pw.3, pw.4, pw.5, 10) ;
}


struct Counts {
	part1 :usize,
	part2 :usize,
}
fn run(min :usize, max :usize) -> Counts {
	// Brute force!
	let mut part1 :usize = 0;
	let mut part2 :usize = 0;
	let mut iter = Some(Password::from_int(min));
	while let Some(pw) = iter {
		if pw.as_int() > max {
			break;
		}
		if has_part2_adjacents(pw) {
			part1 += 1;
			part2 += 1;
		}
		else if has_adjacents(pw) {
			part1 += 1;
		}
		iter = pw.increment();
	}
	return Counts {
		part1,
		part2,
	};
}


fn get_range() -> [usize; 2]
{
	// Load the input from file
	let input = match std::fs::read_to_string("inputs/day4.txt")
	{
		Err(error) => panic!("Failed to open input: {}", error.description()),
		Ok(string) => string,
	};
	
	// Transform the start and end into integers
	let output :Vec<usize> = input
		.trim() // ignore trailing whitespace
		.split('-') // split on '-'
		.map(|val| val.parse().unwrap()) // parse each value into a usize
		.collect(); // combine into a Vec
	
	return [output[0], output[1]];
}

pub fn main() {
	let range = get_range();
	let result = run(range[0], range[1]);
	println!("Day4:");
	println!("\tPart1 = {}", result.part1);
	println!("\tPart2 = {}", result.part2);
}


