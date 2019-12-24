/**
 * Part 1:
 *
 * Each image actually consists of a series of identically-sized layers that
 * are filled in this way. So, the first digit corresponds to the top-left
 * pixel of the first layer, the second digit corresponds to the pixel to the
 * right of that on the same layer, and so on until the last digit, which
 * corresponds to the bottom-right pixel of the last layer.
 *
 * The image you received is 25 pixels wide and 6 pixels tall.
 * 
 * To make sure the image wasn't corrupted during transmission, the Elves would
 * like you to find the layer that contains the fewest 0 digits. On that layer,
 * what is the number of 1 digits multiplied by the number of 2 digits?
 *
 *
 * Part 2:
 *
 * Now you're ready to decode the image. The image is rendered by stacking the
 * layers and aligning the pixels with the same positions in each layer. The
 * digits indicate the color of the corresponding pixel: 0 is black, 1 is
 * white, and 2 is transparent.
 *
 * The layers are rendered with the first layer in front and the last layer in
 * back. So, if a given position has a transparent pixel in the first and
 * second layers, a black pixel in the third layer, and a white pixel in the
 * fourth layer, the final image would have a black pixel at that position.
 *
 * What message is produced after decoding your image?
 */


use std::error::Error;


struct ImageLayer {
	data : Vec<usize>,
}

impl ImageLayer {
	fn new(data :&str) -> ImageLayer {
		let mut layer :Vec<usize> = Vec::new();
		for ch in data.chars() {
			let ch = (ch as usize) - ('0' as usize);
			layer.push(ch);
		}
		return ImageLayer {
			data : layer,
		};
	}
}

struct Image {
	layers : Vec<ImageLayer>,
	width : usize,
	height : usize,
}

impl Image {
	fn new(data :&str, width :usize, height :usize) -> Image {
		let mut layers = Vec::new();
		
		// Split the data into layers
		let layer_size = width * height;
		assert_eq!(data.len() % layer_size, 0);
		let layer_count = data.len() / layer_size;
		
		for idx in 0..layer_count {
			let start = idx * layer_size;
			let end = start + layer_size;
			let range = &data[start..end];
			let layer = ImageLayer::new(range);
			layers.push(layer);
		}
		
		return Image {
			layers,
			width,
			height,
		};
	}
}


// Load the input as a string
fn load_input() -> String
{
	// Load the input from file
	let input = match std::fs::read_to_string("inputs/day8.txt")
	{
		Err(error) => panic!("Failed to open input: {}", error.description()),
		Ok(string) => string,
	};
	
	return input.trim().to_string();
}


fn run_part1() -> usize
{
	// Create an image from the input
	let input = load_input();
	let image = Image::new(&input, 25, 6);
	
	// Find the layer with the most 0s
	let mut zeros = image.width * image.height;
	let mut product = 0;
	for layer in image.layers {
		// Count the number of 0s, 1s, and 2s
		let mut bins = vec![0; 3];
		for pixel in layer.data {
			bins[pixel] += 1;
		}
		
		// Update the product if this layer has more
		if bins[0] < zeros {
			zeros = bins[0];
			product = bins[1] * bins[2];
		}
	}
	
	return product;
}


fn run_part2()
{
	// Create an image from the input
	let input = load_input();
	let image = Image::new(&input, 25, 6);
	
	// Create the final image as a layer of all transparent
	let mut final_image = ImageLayer{
		data : vec![2; 25 * 6],
	};
	
	// Go through each layer and apply the colouring
	for layer in image.layers {
		for idx in 0..(25*6) {
			let ref mut pixel = final_image.data[idx];
			match pixel {
				// We already have a final colour, so nothing to do
				0 => {},
				1 => {},
				
				// See if this layer has a colour
				2 => {
					if layer.data[idx] != 2 {
						*pixel = layer.data[idx];
					}
				},
				
				_ => panic!("Unexpected pixel colour: {}", pixel),
			}
		}
	}
	
	// I'm not writing text recognition to teach myself rust...
	for y in 0..6 {
		for x in 0..25 {
			let pixel = final_image.data[x + y * 25];
			match pixel {
				0 => print!(" "),
				1 => print!("X"),
				_ => panic!("Unexpected pixel colour: {}", pixel),
			};
		}
		println!("");
	}
}


pub fn main()
{
	println!("Day8:");
	println!("\tPart1 = {}", run_part1());
	run_part2();
}

