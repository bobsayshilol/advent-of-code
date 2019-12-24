/**
 * Intcode interpreter, used by multiple days.
 */


// The interpreter object
#[derive(Clone)]
pub struct Interpreter {
	mem : Vec<isize>,
	pc : usize,
	next_input : Option<isize>,
	outputs :Vec<isize>,
}


// The result of an execution
#[derive(Debug, PartialEq)]
pub enum StepResult {
	Continue,
	Break,
	Input,
}


impl Interpreter {
	// Load the input string into a buffer to be treated as Intcode RAM
	pub fn load(input :&str) -> Interpreter {
		// Transform the elements into integers
		let mem :Vec<isize> = input
			.trim() // ignore trailing whitespace
			.split(',') // split on ','
			.map(|val| val.parse().unwrap()) // parse each value into an isize
			.collect(); // combine into a Vec
		
		return Interpreter {
			mem,
			pc : 0,
			next_input : None,
			outputs : Vec::new(),
		};
	}
	
	// Set a value in memory
	pub fn set(self :&mut Interpreter, idx :usize, val :isize) {
		self.mem[idx] = val;
	}
	
	// Get a value from memory
	pub fn get(self :&Interpreter, idx :usize) -> isize {
		return self.mem[idx];
	}
	
	// Set the next input value
	pub fn set_input(self :&mut Interpreter, input :isize) {
		self.next_input = Some(input);
	}
	
	// Get and clear the outputs from the program
	pub fn get_outputs(self :&mut Interpreter) -> Vec<isize> {
		let mut output = Vec::new();
		std::mem::swap(&mut output, &mut self.outputs);
		return output;
	}
	
	// Pretty print a program (but not that pretty)
	#[allow(dead_code)]
	pub fn dump(self :&Interpreter)
	{
		println!("Program (pc={}):", self.pc);
		for idx in 0..self.mem.len()
		{
			print!("{},\t", self.mem[idx]);
			if (idx & 7) == 7
			{
				println!();
			}
		}
		println!();
	}
	
	// Step the program
	pub fn step(self :&mut Interpreter) -> StepResult {
		// Read off the next instruction
		let ins = self.mem[self.pc];
		
		#[derive(Debug, PartialEq)]
		enum OpMode { Position, Immediate, };
		let to_op_mode = |val| {
			match val {
				0 => OpMode::Position,
				1 => OpMode::Immediate,
				_ => panic!("Unknown parameter mode: {} for ins: {} at pc: {}", val, ins, self.pc),
			}
		};
		
		// Peel away the parameter modes
		let operation = ins % 100;
		let operand_mode_0 = to_op_mode((ins / 100) % 10);
		let operand_mode_1 = to_op_mode((ins / 1000) % 10);
		let operand_mode_2 = to_op_mode((ins / 10000) % 10);
		
		// Setup read/write methods
		let read_operand = |val, mode| {
			match mode {
				OpMode::Position => self.mem[val as usize],
				OpMode::Immediate => val,
			}
		};
		
		// Handle the instruction
		match operation
		{
			// Add
			1 => {
				// Load the sources and dest
				let s0 = self.mem[self.pc + 1];
				let s1 = self.mem[self.pc + 2];
				assert_eq!(operand_mode_2, OpMode::Position);
				let d0 = self.mem[self.pc + 3] as usize;
				
				// Do the op
				self.mem[d0] =
					read_operand(s0, operand_mode_0) +
					read_operand(s1, operand_mode_1);
				
				// Increment to the next ins
				self.pc += 4;
			},
			
			// Mul
			2 => {
				// Load the sources and dest
				let s0 = self.mem[self.pc + 1];
				let s1 = self.mem[self.pc + 2];
				assert_eq!(operand_mode_2, OpMode::Position);
				let d0 = self.mem[self.pc + 3] as usize;
				
				// Do the op
				self.mem[d0] = 
					read_operand(s0, operand_mode_0) *
					read_operand(s1, operand_mode_1);
				
				// Increment to the next ins
				self.pc += 4;
			},
			
			// Input
			3 => {
				// Read the next queued input
				if let Some(input) = self.next_input {
					self.next_input = None;
					
					// Load the dest location
					assert_eq!(operand_mode_0, OpMode::Position);
					let d0 = self.mem[self.pc + 1] as usize;
					
					// Store it to memory
					self.mem[d0] = input;
					
					// Increment to the next ins
					self.pc += 2;
				} else {
					// No input ready, so return that we need one
					return StepResult::Input;
				}
			},
			
			// Output
			4 => {
				// Load the source location
				let s0 = self.mem[self.pc + 1];
				
				// Get the value requested and save it as an output
				let val = read_operand(s0, operand_mode_0);
				self.outputs.push(val);
				
				// Increment to the next ins
				self.pc += 2;
			},
			
			// JNZ
			5 => {
				// Load the source and dest
				let s0 = self.mem[self.pc + 1];
				let d0 = self.mem[self.pc + 2];
				
				// Update pc to the new address
				if read_operand(s0, operand_mode_0) != 0 {
					self.pc = read_operand(d0, operand_mode_1) as usize;
				} else {
					self.pc += 3;
				}
			},
			
			// JEZ
			6 => {
				// Load the source and dest
				let s0 = self.mem[self.pc + 1];
				let d0 = self.mem[self.pc + 2];
				
				// Update pc to the new address
				if read_operand(s0, operand_mode_0) == 0 {
					self.pc = read_operand(d0, operand_mode_1) as usize;
				} else {
					self.pc += 3;
				}
			},
			
			// Set if less
			7 => {
				// Load the source and dest
				let s0 = self.mem[self.pc + 1];
				let s1 = self.mem[self.pc + 2];
				assert_eq!(operand_mode_2, OpMode::Position);
				let d0 = self.mem[self.pc + 3] as usize;
				
				// Calculate the result
				let val = read_operand(s0, operand_mode_0) < read_operand(s1, operand_mode_1);
				
				// Store it to memory
				self.mem[d0] = if val { 1 } else { 0 };
				
				// Increment to the next ins
				self.pc += 4;
			},
			
			// Set if equal
			8 => {
				// Load the source and dest
				let s0 = self.mem[self.pc + 1];
				let s1 = self.mem[self.pc + 2];
				assert_eq!(operand_mode_2, OpMode::Position);
				let d0 = self.mem[self.pc + 3] as usize;
				
				// Calculate the result
				let val = read_operand(s0, operand_mode_0) == read_operand(s1, operand_mode_1);
				
				// Store it to memory
				self.mem[d0] = if val { 1 } else { 0 };
				
				// Increment to the next ins
				self.pc += 4;
			},
			
			// Break out
			99 => {
				self.pc += 1;
				return StepResult::Break;
			},
			
			// This shouldn't happen
			_ => panic!("Unknown ins: {} at pc: {}", ins, self.pc),
		}
		
		return StepResult::Continue;
	}
	
	// Executes the program until a break is encountered
	pub fn run(self :&mut Interpreter) -> StepResult
	{
		loop {
			let result = self.step();
			if result != StepResult::Continue {
				return result;
			}
		}
	}
}

