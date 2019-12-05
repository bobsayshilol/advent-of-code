/**
 * Part 1:
 *
 * Opening the front panel reveals a jumble of wires. Specifically, two wires
 * are connected to a central port and extend outward on a grid. You trace the
 * path each wire takes as it leaves the central port, one wire per line of
 * text (your puzzle input).
 *
 * The wires twist and turn, but the two wires occasionally cross paths. To fix
 * the circuit, you need to find the intersection point closest to the central
 * port. Because the wires are on a grid, use the Manhattan distance for this
 * measurement. While the wires do technically cross right at the central port
 * where they both start, this point does not count, nor does a wire count as
 * crossing with itself.
 *
 * For example, if the first wire's path is R8,U5,L5,D3, then starting from the
 * central port (o), it goes right 8, up 5, left 5, and finally down 3.
 *
 *
 * Part 2:
 */

use std::error::Error;
use std::ops::Add;


// Simple vec2 struct
#[derive(Copy, Clone)]
struct TVec2<T>
{
	x :T,
	y :T,
}

type Vec2 = TVec2<usize>;
type Vec2i = TVec2<isize>;

impl<T> TVec2<T>
{
	fn new(x :T, y :T) -> TVec2<T> {
		return TVec2::<T> {
			x,
			y,
		};
	}
}

impl<T> Add for TVec2<T>
where
	T: Add<Output = T> + Copy
{
	type Output = TVec2<T>;
	fn add(self, other :Self) -> Self::Output {
		return Self::Output {
			x : self.x + other.x,
			y : self.y + other.y,
		};
	}
}


// A grid where the origin may not be at (0,0).
// Use get() and set() to get to the actual data.
struct Grid {
	grid :Vec<bool>,
	size :Vec2,
	origin :Vec2,
}

impl Grid {
	fn build(&mut self) {
		self.grid = vec![false; self.size.x * self.size.y];
	}
	fn set(&mut self, pos :Vec2i) {
		let origin = Vec2i { x : self.origin.x as isize, y : self.origin.y as isize, };
		let pos = origin + pos;
		let pos = Vec2 { x : pos.x as usize, y : pos.y as usize, };
		self.grid[pos.x + pos.y * self.size.x] = true;
	}
	fn get(&self, pos :Vec2i) -> bool {
		let origin = Vec2i { x : self.origin.x as isize, y : self.origin.y as isize, };
		let pos = origin + pos;
		let pos = Vec2 { x : pos.x as usize, y : pos.y as usize, };
		return self.grid[pos.x + pos.y * self.size.x];
	}
}


// A segment of the wire
enum WireSegment {
	Up(usize),
	Down(usize),
	Left(usize),
	Right(usize),
}

// A full wire composed of multiple segments
struct Wire {
	segments :Vec<WireSegment>,
}

impl Wire {
	fn from_string(input :&str) -> Wire
	{
		let to_segment = |segment :&str|
		{
			// Read off the direction and the distance of this segment
			let direction = segment.chars().nth(0).unwrap();
			let distance :usize = segment[1..].parse().unwrap();
			
			// Map it to a WireSegment enum
			match direction
			{
				'U' => WireSegment::Up(distance),
				'D' => WireSegment::Down(distance),
				'L' => WireSegment::Left(distance),
				'R' => WireSegment::Right(distance),
				_ => panic!("Unknown direction: {}", direction),
			}
		};
		
		// Build the input into a vector of segments
		let segments =
			input
				.split(',') // split it on the ','
				.map(&to_segment) // map it to a WireSegment
				.collect(); // collect them into a Vec
		
		return Wire {
			segments,
		}
	}
	
	fn for_each<Func>(&self, lambda :&mut Func)
		where Func : FnMut(&WireSegment)
	{
		// I don't know if this function actually helps at all...
		for segment in &self.segments
		{
			lambda(&segment);
		}
	}
}


// Load the input as a vector of Wire structs
// TODO: I'd prefer a [Wire; 2] here really (at least for Part 1)
fn load_wires() -> Vec<Wire>
{
	// Load the input from file
	let input = match std::fs::read_to_string("input.txt")
	{
		Err(error) => panic!("Failed to open input: {}", error.description()),
		Ok(string) => string,
	};
	
	// Read in each line as a wire
	let mut wires = Vec::new();
	for line in input.lines()
	{
		wires.push(Wire::from_string(line));
	}
	
	return wires;
}


// Calculate the smallest grid required to fit a wire
fn smallest_grid(wire :&Wire) -> Grid
{
	// Note: this doesn't actually calculate the smallest due to unsigned
	// warnings in rust
	let mut ur_extents = Vec2::new(0,0);
	let mut dl_extents = Vec2::new(0,0);
	wire.for_each(&mut |segment| {
		match segment
		{
			WireSegment::Up(dist) => ur_extents.y += dist,
			WireSegment::Down(dist) => dl_extents.y += dist,
			WireSegment::Left(dist) => ur_extents.x += dist,
			WireSegment::Right(dist) => dl_extents.x += dist,
		}
	});
	
	// Add the extents together to get the total. down-left will give us the
	// offset of the origin.
	return Grid {
		grid :Vec::new(),
		size : ur_extents + dl_extents,
		origin :dl_extents,
	};
}


fn part1() -> usize
{
	let wires = load_wires();
	assert_eq!(wires.len(), 2);
	
	// Calculate the smallest grid required to fit both wires
	let grid1 = smallest_grid(&wires[0]);
	let grid2 = smallest_grid(&wires[1]);
	let size = Vec2 {
		x : std::cmp::max(grid1.size.x, grid2.size.x),
		y : std::cmp::max(grid1.size.y, grid2.size.y),
	};
	let origin = Vec2 {
		x : std::cmp::max(grid1.origin.x, grid2.origin.x),
		y : std::cmp::max(grid1.origin.y, grid2.origin.y),
	};
	let mut grid = Grid {
		grid :Vec::new(),
		size,
		origin,
	};
	
	// Create the grid we'll use
	grid.build();
	
	// Set all the points on one wire
	let mut pos = Vec2i::new(0,0);
	grid.set(pos);
	wires[0].for_each(&mut |segment| {
		// TODO: there must be a neater way of repeating these loops...
		match segment
		{
			WireSegment::Up(dist) => {
				for _ in 0..*dist {
					pos.y += 1;
					grid.set(pos);
				}
			},
			WireSegment::Down(dist) => {
				for _ in 0..*dist {
					pos.y -= 1;
					grid.set(pos);
				}
			},
			WireSegment::Left(dist) => {
				for _ in 0..*dist {
					pos.x -= 1;
					grid.set(pos);
				}
			},
			WireSegment::Right(dist) => {
				for _ in 0..*dist {
					pos.x += 1;
					grid.set(pos);
				}
			},
		}
	});
	
	// Go through it again, but this time check for any that are already set
	let mut crossovers :Vec<Vec2i> = Vec::new();
	pos = Vec2i::new(0,0);
	wires[1].for_each(&mut |segment| {
		// TODO: there must be a neater way of repeating these loops...
		match segment
		{
			WireSegment::Up(dist) => {
				for _ in 0..*dist {
					pos.y += 1;
					if grid.get(pos) { crossovers.push(pos); }
				}
			},
			WireSegment::Down(dist) => {
				for _ in 0..*dist {
					pos.y -= 1;
					if grid.get(pos) { crossovers.push(pos); }
				}
			},
			WireSegment::Left(dist) => {
				for _ in 0..*dist {
					pos.x -= 1;
					if grid.get(pos) { crossovers.push(pos); }
				}
			},
			WireSegment::Right(dist) => {
				for _ in 0..*dist {
					pos.x += 1;
					if grid.get(pos) { crossovers.push(pos); }
				}
			},
		}
	});
	
	// Find the smallest distance
	let mut dist :usize = 100000;
	for pt in &crossovers
	{
		let len = pt.x.abs() + pt.y.abs();
		dist = std::cmp::min(dist, len as usize);
	}
	
	return dist;
}


fn main()
{
	println!("Part1 = {}", part1());
}
