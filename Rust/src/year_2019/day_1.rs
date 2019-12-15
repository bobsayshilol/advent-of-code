/**
 * Part 1:
 *
 * Fuel required to launch a given module is based on its mass. Specifically,
 * to find the fuel required for a module, take its mass, divide by three,
 * round down, and subtract 2.
 *
 * For example:
 *
 *    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to
 *    get 2.
 *    For a mass of 14, dividing by 3 and rounding down still yields 4, so the
 *    fuel required is also 2.
 *    For a mass of 1969, the fuel required is 654.
 *    For a mass of 100756, the fuel required is 33583.
 *
 * The Fuel Counter-Upper needs to know the total fuel requirement. To find it,
 * individually calculate the fuel needed for the mass of each module (your
 * puzzle input), then add together all the fuel values.
 *
 *
 * Part 2:
 *
 * Fuel itself requires fuel just like a module - take its mass, divide by
 * three, round down, and subtract 2. However, that fuel also requires fuel,
 * and that fuel requires fuel, and so on. Any mass that would require negative
 * fuel should instead be treated as if it requires zero fuel; the remaining
 * mass, if any, is instead handled by wishing really hard, which has no mass
 * and is outside the scope of this calculation.
 *
 * So, for each module mass, calculate its fuel and add it to the total. Then,
 * treat the fuel amount you just calculated as the input mass and repeat the
 * process, continuing until a fuel requirement is zero or negative.
 */


use std::error::Error;


// Load the inputs into a vector
fn load_inputs() -> Vec<u64>
{
	// Load the inputs from file
	let input = match std::fs::read_to_string("inputs/day1.txt")
	{
		Err(error) => panic!("Failed to open inputs: {}", error.description()),
		Ok(string) => string,
	};
	
	// Go through the inputs line by line and transform them into u64s
	let mut output = Vec::new();
	for line in input.lines()
	{
		output.push(line.parse().unwrap());
	}
	
	return output;
}


// Get the total fuel required for a list of masses
fn get_total_fuel<Func>(masses :&Vec<u64>, calculate_fuel :&Func) -> u64
	where Func :Fn(u64) -> u64
{
	// Go through the inputs one by one and accumulate the results
	let mut total_fuel :u64 = 0;
	for mass in masses
	{
		total_fuel += calculate_fuel(*mass);
	}
	
	return total_fuel;
}


// Simple fuel calculation as described above
fn calculate_fuel_simple(mass :u64) -> u64
{
	let fuel = mass / 3;
	// Don't let it go below 0
	return std::cmp::max(fuel, 2) - 2;
}


fn get_total_part1() -> u64
{
	// Read in the masses and calculate the total fuel required for it
	let masses = load_inputs();
	return get_total_fuel(&masses, &calculate_fuel_simple);
}


fn get_total_part2() -> u64
{
	// Read in the masses
	let masses = load_inputs();
	
	// Slightly more complex sum, as described above
	let calculate_fuel_complex = |mass|
	{
		// Nothing fancy, just keep looping until there's no changes in the
		// amount of fuel required.
		let mut next_fuel = calculate_fuel_simple(mass);
		let mut total_fuel = next_fuel;
		while next_fuel != 0
		{
			// Recalculate it for the smaller amount of fuel
			next_fuel = calculate_fuel_simple(next_fuel);
			total_fuel += next_fuel;
		}
		
		return total_fuel;
	};
	
	return get_total_fuel(&masses, &calculate_fuel_complex);
}


pub fn main()
{
	let part1 = get_total_part1();
	let part2 = get_total_part2();
	println!("Day1:");
	println!("\tPart1 = {}", part1);
	println!("\tPart2 = {}", part2);
}
