use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	println!("Guess the Number");
	let secrect_number = rand::thread_rng().gen_range(1, 100);

	loop {
		println!("Please input your guess 1 to 100");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("failed to read line");

		// let guess: u32 = guess.trim().parse().expect("Please Enter a number");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		println!("You guessed {}", guess);
		match guess.cmp(&secrect_number) {
			Ordering::Less => println!("Too Small"),
			Ordering::Greater => println!("Too High"),
			Ordering::Equal => {
				println!("You Won");
				break;
			}
		}
	}
}
