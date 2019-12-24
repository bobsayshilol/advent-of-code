/**
 * Part 1:
 *
 * There are five amplifiers connected in series; each one receives an input
 * signal and produces an output signal. They are connected such that the first
 * amplifier's output leads to the second amplifier's input, the second
 * amplifier's output leads to the third amplifier's input, and so on. The
 * first amplifier's input value is 0, and the last amplifier's output leads to
 * your ship's thrusters.
 *
 * When a copy of the program starts running on an amplifier, it will first use
 * an input instruction to ask the amplifier for its current phase setting (an
 * integer from 0 to 4). Each phase setting is used exactly once, but the Elves
 * can't remember which amplifier needs which phase setting.
 *
 * Your job is to find the largest output signal that can be sent to the
 * thrusters by trying every possible combination of phase settings on the
 * amplifiers.
 *
 *
 * Part 2:
 *
 * Most of the amplifiers are connected as they were before; amplifier A's
 * output is connected to amplifier B's input, and so on. However, the output
 * from amplifier E is now connected into amplifier A's input. This creates the
 * feedback loop: the signal will be sent through the amplifiers many times.
 *
 * In feedback loop mode, the amplifiers need totally different phase settings:
 * integers from 5 to 9, again each used exactly once. These settings will
 * cause the Amplifier Controller Software to repeatedly take input and produce
 * output many times before halting. Provide each amplifier its phase setting
 * at its first input instruction; all further input/output instructions are
 * for signals.
 *
 * Eventually, the software on the amplifiers will halt after they have
 * processed the final loop. When this happens, the last output signal from
 * amplifier E is sent to the thrusters. Your job is to find the largest output
 * signal that can be sent to the thrusters using the new phase settings and
 * feedback loop arrangement.
 */


use std::error::Error;
use super::intcode::{Interpreter, StepResult};


struct Amps {
	base : Interpreter,
}


impl Amps {
	fn new() -> Amps {
		let input = load_input();
		return Amps {
			base : Interpreter::load(&input),
		};
	}
	
	fn run(self :&Amps, phase_a :usize, phase_b :usize, phase_c :usize, phase_d :usize, phase_e :usize) -> isize {
		// Create the amps
		let mut a = self.base.clone();
		let mut b = self.base.clone();
		let mut c = self.base.clone();
		let mut d = self.base.clone();
		let mut e = self.base.clone();
		
		let run_prog = |prog :&mut Interpreter, phase :usize, input :isize| {
			// First input is the phase
			prog.set_input(phase as isize);
			let result = prog.run();
			assert_eq!(result, StepResult::Input);
			
			// Second input is the input
			prog.set_input(input);
			let result = prog.run();
			assert_eq!(result, StepResult::Break);
			
			return prog.get_outputs()[0];
		};
		
		// Run each in turn
		let next_input = 0;
		let next_input = run_prog(&mut a, phase_a, next_input);
		let next_input = run_prog(&mut b, phase_b, next_input);
		let next_input = run_prog(&mut c, phase_c, next_input);
		let next_input = run_prog(&mut d, phase_d, next_input);
		let next_input = run_prog(&mut e, phase_e, next_input);
		return next_input;
	}
	
	fn run_loop(self :&Amps, phase_a :usize, phase_b :usize, phase_c :usize, phase_d :usize, phase_e :usize) -> isize {
		// Lambda to create an amp
		let setup_amp = |phase :usize| {
			let mut prog = self.base.clone();
			
			// First input is the phase
			prog.set_input(phase as isize);
			let result = prog.run();
			assert_eq!(result, StepResult::Input);
			
			return prog;
		};
		
		// Lambda to run the next part of the program
		let run_prog = |prog :&mut Interpreter, input :&mut isize| {
			// Input is the input
			prog.set_input(*input);
			let result = prog.run();
			
			// Save back the output
			*input = prog.get_outputs()[0];
			
			// Return the result of the execution
			return result;
		};
		
		// Create the amps
		let mut a = setup_amp(phase_a);
		let mut b = setup_amp(phase_b);
		let mut c = setup_amp(phase_c);
		let mut d = setup_amp(phase_d);
		let mut e = setup_amp(phase_e);
		
		// Run each in turn until it breaks
		let mut next_input = 0;
		loop {
			// We can't add asserts about any of these since they all
			// return break on the last pass
			run_prog(&mut a, &mut next_input);
			run_prog(&mut b, &mut next_input);
			run_prog(&mut c, &mut next_input);
			run_prog(&mut d, &mut next_input);
			let result = run_prog(&mut e, &mut next_input);
			
			// If the final run produced a break then we're done
			if result == StepResult::Break {
				break;
			}
		}
		return next_input;
	}
}


// Load the input into a buffer to be treated as Intcode RAM
fn load_input() -> String
{
	// Load the input from file
	let input = match std::fs::read_to_string("inputs/day7.txt")
	{
		Err(error) => panic!("Failed to open input: {}", error.description()),
		Ok(string) => string,
	};
	
	return input;
}


fn run_part1() -> isize
{
	// Load the program as provided into the amp
	let amps = Amps::new();
	
	// Find the max
	let mut max_output = 0;
	let mut _max_phase = 0;
	
	// 5! = 120 combos, but we'll go through all of them to make the logic a
	// little bit easier (ie so I don't have to add permutations)
	for phase in 0..3125 {
		// Peel the phases for each
		let phase_a = (phase / 1) % 5;
		let phase_b = (phase / 5) % 5;
		let phase_c = (phase / 25) % 5;
		let phase_d = (phase / 125) % 5;
		let phase_e = (phase / 625) % 5;
		
		// Check that no digit appears twice
		// Urgh, permutations might have been easier
		let mut bins = vec![0;5];
		bins[phase_a] += 1;
		bins[phase_b] += 1;
		bins[phase_c] += 1;
		bins[phase_d] += 1;
		bins[phase_e] += 1;
		if !bins.iter().all(|&val| val == 1) {
			continue;
		}
		
		// Run it
		let output = amps.run(phase_a, phase_b, phase_c, phase_d, phase_e);
		
		// See if that was bigger
		if output > max_output {
			max_output = output;
			
			// Convert it back to a readable number
			let mut max_phase = 0;
			max_phase += phase_a * 10000;
			max_phase += phase_b * 1000;
			max_phase += phase_c * 100;
			max_phase += phase_d * 10;
			max_phase += phase_e * 1;
			_max_phase = max_phase
		}
	}
	
	return max_output;
}


fn run_part2() -> isize
{
	// Load the program as provided into the amp
	let amps = Amps::new();
	
	// Find the max
	let mut max_output = 0;
	
	// 5! = 120 combos, but we'll go through all of them to make the logic a
	// little bit easier (ie so I don't have to add permutations)
	for phase in 0..3125 {
		// Peel the phases for each
		let phase_a = (phase / 1) % 5;
		let phase_b = (phase / 5) % 5;
		let phase_c = (phase / 25) % 5;
		let phase_d = (phase / 125) % 5;
		let phase_e = (phase / 625) % 5;
		
		// Check that no digit appears twice
		// Urgh, permutations might have been easier
		let mut bins = vec![0;5];
		bins[phase_a] += 1;
		bins[phase_b] += 1;
		bins[phase_c] += 1;
		bins[phase_d] += 1;
		bins[phase_e] += 1;
		if !bins.iter().all(|&val| val == 1) {
			continue;
		}
		
		// Run it, adding 5 to each phase as the numbers are now [5,9)
		let output = amps.run_loop(phase_a + 5, phase_b + 5, phase_c + 5, phase_d + 5, phase_e + 5);
		
		// See if that was bigger
		if output > max_output {
			max_output = output;
		}
	}
	
	return max_output;
}


pub fn main()
{
	println!("Day7:");
	println!("\tPart1 = {}", run_part1());
	println!("\tPart2 = {}", run_part2());
}

