use std::thread;

fn iteration(number: u128)
{
	let start = std::time::Instant::now();
	let mut nb_iterations = 0;
 	let mut current_number = number;

	while current_number != 1
	{
		if current_number % 2 == 0
		{
			break;
		}
		else
		{
			current_number = 3 * current_number + 1;
		}

		nb_iterations += 1;

		// Print if nb_iterations > 100 because it's rare
		if nb_iterations > 100
		{
			println!();
			println!("Number : {}", number);
			println!("Time : {:?}", start.elapsed());
		}
	}
}

fn main()
{
	println!("Syracuse conjecture -- Collatz conjecture -- 3n + 1 problem");
	let mut number: u128 = 1;
	let mut max_number:u128 = 3;

	while max_number < 5
	{
		println!("Enter the max number : ");
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).expect("Failed to read line");
		max_number = input.trim().parse().expect("Please type a number!");
	}

	let start = std::time::Instant::now();

	// 4 threads - 4 iterations per cycle
	let handle = thread::spawn(move || {
		while number <= max_number / 4
		{
			iteration(number);
			iteration(number+1);
			iteration(number+2);
			iteration(number+3);
			number += 4;
		}
	});

	thread::spawn(move || {
		while number + (max_number / 4) <= max_number / 2
		{
			iteration(number);
			iteration(number+1);
			iteration(number+2);
			iteration(number+3);
			number += 4;
		}
	});

	thread::spawn(move || {
		while number + (max_number / 2) <= max_number * 3 / 4
		{
			iteration(number);
			iteration(number+1);
			iteration(number+2);
			iteration(number+3);
			number += 4;
		}
	});

	thread::spawn(move || {

		while number + (max_number * 3 / 4) <= max_number
		{
			iteration(number);
			iteration(number+1);
			iteration(number+2);
			iteration(number+3);
			number += 4;
		}
	});

	handle.join().unwrap();

	println!("Total time : {:?}", start.elapsed());
}
