/**
 * Intcode interpreter, used by days 2 and 5.
 */

// The interpreter object
#[allow(dead_code)]
pub struct Interpreter {
	mem : Vec<usize>,
	pc : usize,
}


impl Interpreter {
	// Load the input string into a buffer to be treated as Intcode RAM
	#[allow(dead_code)]
	pub fn load(input :&str) -> Interpreter {
		// Transform the elements into integers
		let mem :Vec<usize> = input
			.trim() // ignore trailing whitespace
			.split(',') // split on ','
			.map(|val| val.parse().unwrap()) // parse each value into a usize
			.collect(); // combine into a Vec
		
		return Interpreter {
			mem,
			pc : 0,
		};
	}
	
	// Set a value in memory
	#[allow(dead_code)]
	pub fn set(self :&mut Interpreter, idx :usize, val :usize) {
		self.mem[idx] = val;
	}
	
	// Get a value from memory
	#[allow(dead_code)]
	pub fn get(self :&Interpreter, idx :usize) -> usize {
		return self.mem[idx];
	}
	
	// Pretty print a program (but not that pretty)
	#[allow(dead_code)]
	pub fn dump(self :&Interpreter)
	{
		println!("Program:");
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
	#[allow(dead_code)]
	pub fn step(self :&mut Interpreter) -> bool {
		// Read off the next instruction
		let ins = self.mem[self.pc];
		
		// Handle the instruction
		match ins
		{
			1 => {
				// Load the sources and dest
				let s0 = self.mem[self.pc + 1];
				let s1 = self.mem[self.pc + 2];
				let d0 = self.mem[self.pc + 3];
				
				// Do the op
				self.mem[d0] = self.mem[s0] + self.mem[s1];
				
				// Increment to the next ins
				self.pc += 4;
			},
			
			2 => {
				// Load the sources and dest
				let s0 = self.mem[self.pc + 1];
				let s1 = self.mem[self.pc + 2];
				let d0 = self.mem[self.pc + 3];
				
				// Do the op
				self.mem[d0] = self.mem[s0] * self.mem[s1];
				
				// Increment to the next ins
				self.pc += 4;
			},
			
			// Break out
			99 => {
				self.pc += 1;
				return true;
			},
			
			// This shouldn't happen
			_ => panic!("Unknown ins: {} at pc: {}", ins, self.pc),
		}
		
		return false;
	}
	
	// Executes the program until a break is encountered
	#[allow(dead_code)]
	pub fn run(self :&mut Interpreter)
	{
		while !self.step() { }
	}
}

