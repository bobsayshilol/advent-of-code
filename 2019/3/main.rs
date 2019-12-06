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
 *
 * To do this, calculate the number of steps each wire takes to reach each
 * intersection; choose the intersection where the sum of both wires' steps is
 * lowest. If a wire visits a position on the grid multiple times, use the
 * steps value from the first time it visits that position when calculating the
 * total value of a specific intersection.
 *
 * The number of steps a wire takes is the total number of grid squares the
 * wire has entered to get to that location, including the intersection being
 * considered.
 */

use std::error::Error;
use std::ops::Add;
use std::default::Default;


// Simple vec2 struct
#[derive(Copy, Clone)]
struct TVec2<T>
{
	x :T,
	y :T,
}

type Vec2 = TVec2<isize>;
//type Vec2u = TVec2<usize>; // ended up not needed

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
struct Grid<T> {
	grid :Vec<T>,
	size :Vec2,
	origin :Vec2,
}

impl<T> Grid<T>
where
	T: Copy + Clone + Default
{
	fn new(size :Vec2, origin :Vec2) -> Grid<T> {
		let elms = size.x * size.y;
		return Grid::<T> {
			grid : vec![Default::default(); elms as usize],
			size,
			origin,
		};
	}
	fn set(&mut self, pos :Vec2, value :T) {
		let pos = self.origin + pos;
		let index = pos.x + pos.y * self.size.x;
		self.grid[index as usize] = value;
	}
	fn get(&self, pos :Vec2) -> T {
		let pos = self.origin + pos;
		let index = pos.x + pos.y * self.size.x;
		return self.grid[index as usize];
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


// Calculate the smallest extents required to fit a wire
struct Extents {
	up_right :Vec2,
	down_left :Vec2,
}
fn smallest_extents(wire :&Wire) -> Extents
{
	// Grow the extents in opposite directions
	let mut ur_extents = Vec2::new(0,0);
	let mut dl_extents = Vec2::new(0,0);
	wire.for_each(&mut |segment| {
		match segment
		{
			WireSegment::Up(dist) => ur_extents.y += *dist as isize,
			WireSegment::Down(dist) => dl_extents.y += *dist as isize,
			WireSegment::Left(dist) => ur_extents.x += *dist as isize,
			WireSegment::Right(dist) => dl_extents.x += *dist as isize,
		}
	});
	
	return Extents {
		up_right : ur_extents,
		down_left :dl_extents,
	};
}


// Combine 2 wires to make a grid they can both fit on
fn combine_wires<T>(wires :&Vec<Wire>) -> Grid<T>
where
	T: Copy + Clone + Default
{
	// Calculate the smallest extents required to fit both wires
	let extents1 = smallest_extents(&wires[0]);
	let extents2 = smallest_extents(&wires[1]);
	
	// Find the max extents in both directions
	let ur_extents = Vec2 {
		x : std::cmp::max(extents1.up_right.x, extents2.up_right.x),
		y : std::cmp::max(extents1.up_right.y, extents2.up_right.y),
	};
	let dl_extents = Vec2 {
		x : std::cmp::max(extents1.down_left.x, extents2.down_left.x),
		y : std::cmp::max(extents1.down_left.y, extents2.down_left.y),
	};
	
	// Add the extents together to get the total. down-left will give us the
	// offset of the origin.
	return Grid::new(ur_extents + dl_extents, dl_extents);
}


// A simple lambda to pass over all grid points in a WireSegment
fn for_segment<Func>(lambda :&mut Func, dist :&usize)
	where Func : FnMut()
{
	for _ in 0..*dist {
		lambda();
	}
}


fn part1() -> usize
{
	let wires = load_wires();
	assert_eq!(wires.len(), 2);
	
	// Build a grid that fits both wires
	let mut grid = combine_wires::<bool>(&wires);
	
	// Set all the points on one wire
	let mut pos = Vec2::new(0,0);
	grid.set(pos, true);
	wires[0].for_each(&mut |segment| {
		match segment
		{
			WireSegment::Up(dist) => for_segment(&mut || {
					pos.y += 1;
					grid.set(pos, true);
				}, dist),
			WireSegment::Down(dist) => for_segment(&mut || {
					pos.y -= 1;
					grid.set(pos, true);
				}, dist),
			WireSegment::Left(dist) => for_segment(&mut || {
					pos.x -= 1;
					grid.set(pos, true);
				}, dist),
			WireSegment::Right(dist) => for_segment(&mut || {
					pos.x += 1;
					grid.set(pos, true);
				}, dist),
		}
	});
	
	// Reset the position for the next wire
	pos = Vec2::new(0,0);
	
	// Go through it again, but this time check for any that are already set
	let mut crossovers :Vec<Vec2> = Vec::new();
	wires[1].for_each(&mut |segment| {
		match segment
		{
			WireSegment::Up(dist) => for_segment(&mut || {
					pos.y += 1;
					if grid.get(pos) { crossovers.push(pos); }
				}, dist),
			WireSegment::Down(dist) => for_segment(&mut || {
					pos.y -= 1;
					if grid.get(pos) { crossovers.push(pos); }
				}, dist),
			WireSegment::Left(dist) => for_segment(&mut || {
					pos.x -= 1;
					if grid.get(pos) { crossovers.push(pos); }
				}, dist),
			WireSegment::Right(dist) => for_segment(&mut || {
					pos.x += 1;
					if grid.get(pos) { crossovers.push(pos); }
				}, dist),
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
