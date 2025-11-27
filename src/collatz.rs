/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32, mut len: u32) -> u32 {
	len += 1;
	n = n.abs();
	if n == 1 {
		return len;
	}
	match n % 2 {
		0 => {
			n = n / 2;
			return collatz_length(n, len);
		}
		1 => {
			n = 3 * n + 1;
			return collatz_length(n, len);
		}
		_ => {
			unreachable!();
		}
	}
}

fn collatz_length_test() {
	let start = 11;
	let length = collatz_length(start, 0);
	println!("Collatz length of {} is {}", start, length); // should be 15
}