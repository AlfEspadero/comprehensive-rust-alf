include!("collatz.rs");

fn main() {
	println!("Length: {}", collatz_length(11, 0)); // should be 15
}
