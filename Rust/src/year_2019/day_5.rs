/**
 * Part 1:
 *
 * First, you'll need to add two new instructions:
 *
 * Opcode 3 takes a single integer as input and saves it to the position given
 * by its only parameter. For example, the instruction 3,50 would take an input
 * value and store it at address 50.
 * Opcode 4 outputs the value of its only parameter. For example, the
 * instruction 4,50 would output the value at address 50.
 *
 * Now, your ship computer will also need to handle parameters in mode 1,
 * immediate mode. In immediate mode, a parameter is interpreted as a value -
 * if the parameter is 50, its value is simply 50.
 *
 * Parameter modes are stored in the same value as the instruction's opcode.
 * The opcode is a two-digit number based only on the ones and tens digit of
 * the value, that is, the opcode is the rightmost two digits of the first
 * value in an instruction. Parameter modes are single digits, one per
 * parameter, read right-to-left from the opcode: the first parameter's mode is
 * in the hundreds digit, the second parameter's mode is in the thousands
 * digit, the third parameter's mode is in the ten-thousands digit, and so on.
 * Any missing modes are 0.
 *
 * Part 2:
 *
 * More opcodes are introduced.
 */


use std::error::Error;
use super::intcode::Interpreter;


// Load the input into a buffer to be treated as Intcode RAM
fn load_input() -> String
{
	// Load the input from file
	let input = match std::fs::read_to_string("inputs/day5.txt")
	{
		Err(error) => panic!("Failed to open input: {}", error.description()),
		Ok(string) => string,
	};
	
	return input;
}

fn run_part1()
{
	// Load the program as provided
	let mut program = Interpreter::load(&load_input());
	
	// TODO: should set the input to 1
	
	// Run it
	program.run();
}

fn run_part2()
{
	// Load the program as provided
	let mut program = Interpreter::load(&load_input());
	
	// TODO: should set the input to 5
	
	// Run it
	program.run();
}


pub fn main()
{
	println!("Day5:");
	run_part1();
	run_part2();
}

