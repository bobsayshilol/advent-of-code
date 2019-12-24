/**
 * Part 1:
 *
 * In the map data, this orbital relationship is written AAA)BBB, which means
 * "BBB is in orbit around AAA".
 *
 * Before you use your map data to plot a course, you need to make sure it
 * wasn't corrupted during the download. To verify maps, the Universal Orbit
 * Map facility uses orbit count checksums - the total number of direct orbits
 * (like the one shown above) and indirect orbits.
 *
 * Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain
 * can be any number of objects long: if A orbits B, B orbits C, and C orbits
 * D, then A indirectly orbits D.
 */


use std::error::Error;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::cell::RefCell;


struct Orbit {
	parent :Weak<Orbit>,
	name :String,
	children :RefCell<Vec<Rc<Orbit>>>,
}

impl Orbit {
	fn new(parent :Weak<Orbit>, name :&str) -> Orbit {
		return Orbit{
			parent,
			name : name.to_string(),
			children : RefCell::new(Vec::new()),
		};
	}
}


// Load the inputs into orbits
fn load_inputs() -> Rc<Orbit>
{
	// Load the inputs from file
	let input = match std::fs::read_to_string("inputs/day6.txt")
	{
		Err(error) => panic!("Failed to open inputs: {}", error.description()),
		Ok(string) => string,
	};
	
	// Go through the inputs line by line and transform them into parent-child orbits
	let map = {
		let mut map = HashMap::new();
		for line in input.trim().lines()
		{
			let mut parts = line.split(')');
			let ref mut children = map.entry(parts.next().unwrap()).or_insert(Vec::new());
			children.push(parts.next().unwrap());
		}
		map
	};
	
	// Build the base orbit
	let com = Rc::new(Orbit::new(Weak::new(), "COM"));
	
	// Build up the node graph thingy by traversing down from COM
	let mut to_process = Vec::new();
	to_process.push(Rc::downgrade(&com));
	while let Some(node) = to_process.pop() {
		let parent = node.upgrade().unwrap();
		
		// See if this object has any child orbits
		match map.get(parent.name.as_str()) {
			Some(children) => {
				for name in children {
					// Make the new child orbit
					let child = Rc::new(Orbit::new(node.clone(), name));
					
					// Add it to be processed too
					to_process.push(Rc::downgrade(&child));
					
					// Add it as a child of the current node
					parent.children.borrow_mut().push(child);
				}
			},
			
			None => {
				// Nothing to do if this orbit has no children
			},
		}
	}
	
	return com;
}


#[allow(dead_code)]
fn dump(orbit :&Orbit, indent :usize) {
	for _ in 0..indent {
		print!("|");
	}
	println!("{}", orbit.name);
	
	let ref children = *orbit.children.borrow();
	for child in children {
		dump(&child, indent + 1);
	}
}


fn count_orbits() -> usize
{
	let com = load_inputs();
	//dump(&com, 0);
	
	// Walk the graph and add up the number of orbits
	fn count_recursive(node :&Orbit, level :usize) -> usize {
		let mut sum = 0;
		
		let ref children = *node.children.borrow();
		for child in children {
			sum += level;
			sum += count_recursive(&child, level + 1);
		}
		
		return sum;
	};
	return count_recursive(&com, 1);
}


pub fn main()
{
	let part1 = count_orbits();
	//let part2 = get_total_part2();
	println!("Day6:");
	println!("\tPart1 = {}", part1);
	//println!("\tPart2 = {}", part2);
}
