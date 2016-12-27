extern crate rand;//using the dependency rand
use std::io;//using it for input output purpose
use rand::Rng;//using it for random no. generation
use std::cmp::Ordering;//using it for the comparison purpose
fn main()
{   
	println!("Guessing Game :)");
	let secret_number=rand::thread_rng().gen_range(1,101);
	loop
	{
		println!("Please Input Your Guess:");
		let mut guess=String::new();
		io::stdin().read_line(&mut guess)
		.ok()
		.expect("Failed to read the line");
		let guess:u32=match guess.trim().parse()//type conversion
		{
			Ok(num)=>num,
			Err(_)=>continue,//to avoid the crashing of program due to the illegal input
		};	
		println!("You guessed {}",guess);

		match guess.cmp(&secret_number)
		{
			Ordering::Less=>println!("Too small :( "),
			Ordering::Greater=>println!("Too Big :("),
			Ordering::Equal=>
			{
				println!("You Win :) Congrats gamer!");
				break;//quitting the program once the user wins
			}
		}
	}
}
