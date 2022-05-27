use std::io;

fn read_number() -> i32 {
	// reading string
	let mut input_text = String::new();
	io::stdin()
		.read_line(&mut input_text)
		.expect("ERROR: failed to read number from standart input. Please post issue on our github page.");

	// making number from string we read
	let trimmed = input_text.trim();
	match trimmed.parse::<i32>() {
		Ok(i) => return i,
		Err(..) => println!("ERROR: failed converting string input to number. Please post issue on out github page.")

	};
	return 0;
}

fn main() {
	let first_operand: i32 = read_number();
	let second_operand: i32 = read_number();

	// calculating and writing value on screen
	println!("{} + {} = {}", first_operand, second_operand, first_operand + second_operand);

	println!("Done!");
	println!("Please star our repository on GitHub >w<");
}
