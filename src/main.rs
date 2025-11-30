// Allow dead code for elevator
#![allow(dead_code)]

include!("collatz.rs");
include!("transpose.rs");
include!("geometry.rs");
include!("elevator.rs");
include!("arithmetic.rs");
include!("logger.rs");


fn main() {
	println!("\t\t------------------ Google's Comprehensive Rust Course ------------------");
	println!("\t\t\t\t\tAttempted by AlfEspadero\n");

	println!("--- Collatz Sequence Length Test ---");
	collatz_length_test();
	println!("\n--- Matrix Transpose Test ---");
	transpose_test();
	println!("\n--- 3D Geometry Test ---");
	test_geometry();
	println!("\n--- Elevator Simulation Test ---");
	elevator_test();
	println!("\n--- Arithmetic Expression Evaluation Test ---");
	println!("Run cargo test to execute the arithmetic tests.");
	println!("\n--- Logger Functionality Test ---");
	logger_test();

}
