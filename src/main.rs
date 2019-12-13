use rand::Rng;
use std::io;

fn main() {
	println!("Guess the Number");
	let secrect_number = rand::thread_rng().gen_range(1, 100);
	println!("The Secrect number is {}", secrect_number);
	println!("Please input your guess");
	let mut guess = String::new();
	io::stdin()
		.read_line(&mut guess)
		.expect("failed to read line");

	println!("You guessed {}", guess);
}
