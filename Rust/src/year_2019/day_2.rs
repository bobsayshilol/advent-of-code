/**
 * Part 1:
 *
 * An Intcode program is a list of integers separated by commas (like 1,0,0,3,
 * 99). To run one, start by looking at the first integer (called position 0).
 * Here, you will find an opcode - either 1, 2, or 99. The opcode indicates
 * what to do; for example, 99 means that the program is finished and should
 * immediately halt. Encountering an unknown opcode means something went wrong.
 *
 * Opcode 1 adds together numbers read from two positions and stores the result
 * in a third position. The three integers immediately after the opcode tell
 * you these three positions - the first two indicate the positions from which
 * you should read the input values, and the third indicates the position at
 * which the output should be stored.
 *
 * For example, if your Intcode computer encounters 1,10,20,30, it should read
 * the values at positions 10 and 20, add those values, and then overwrite the
 * value at position 30 with their sum.
 *
 * Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
 * instead of adding them. Again, the three integers after the opcode indicate
 * where the inputs and outputs are, not their values.
 *
 * Once you're done processing an opcode, move to the next one by stepping
 * forward 4 positions.
 *
 *
 * Part 2:
 *
 * The inputs should still be provided to the program by replacing the values
 * at addresses 1 and 2, just like before. In this program, the value placed in
 * address 1 is called the noun, and the value placed in address 2 is called
 * the verb. Each of the two input values will be between 0 and 99, inclusive.
 *
 * Once the program has halted, its output is available at address 0, also just
 * like before. Each time you try a pair of inputs, make sure you first reset
 * the computer's memory to the values in the program (your puzzle input) - in
 * other words, don't reuse memory from a previous attempt.
 *
 * Find the input noun and verb that cause the program to produce the output
 * 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2,
 * the answer would be 1202.)
 */


use std::error::Error;
use super::intcode::Interpreter;


// Load the input into a buffer to be treated as Intcode RAM
fn load_input() -> String
{
	// Load the input from file
	let input = match std::fs::read_to_string("inputs/day2.txt")
	{
		Err(error) => panic!("Failed to open input: {}", error.description()),
		Ok(string) => string,
	};
	
	return input;
}


fn run_part1() -> isize
{
	// Load the program as provided
	let mut program = Interpreter::load(&load_input());
	
	// Modify it as required
	program.set(1, 12);
	program.set(2, 2);
	
	// Run it
	program.run();
	
	// Return the first value
	return program.get(0);
}


pub fn main()
{
	println!("Day2:");
	println!("\tPart1 = {}", run_part1());
	println!("\tPart2 = {}", run_part2());
}


fn run_part2() -> isize
{
	// Load the program as provided
	let input = load_input();
	
	// Brute force it
	for noun in 0..99
	{
		for verb in 0..99
		{
			// Create a new program from the input
			let mut program = Interpreter::load(&input);
			
			// Mutate the program
			program.set(1, noun);
			program.set(2, verb);
			
			// Run it
			program.run();
			
			// See if that was what we wanted
			if program.get(0) == 19690720
			{
				// Return the combined value
				return noun * 100 + verb;
			}
		}
	}
	
	panic!("Didn't find it?");
}

