// Allow dead code for elevator
#![allow(dead_code)]

include!("collatz.rs");
include!("transpose.rs");
include!("geometry.rs");
include!("elevator.rs");
include!("arithmetic.rs");
include!("logger.rs");
include!("generics.rs");
include!("counter.rs");
include!("rot13.rs");
include!("builder.rs");
include!("btree.rs");
include!("health.rs");

fn main() {
	println!("\t\t------------------ Google's Comprehensive Rust Course ------------------");
	println!("\t\t\t\t\tAttempted by AlfEspadero\n");

	println!("--- Collatz Sequence Length Test ---");
	collatz_length_test();
	println!("\n--- Matrix Transpose Test ---");
	transpose_test();
	println!("\n--- 3D Geometry Test ---");
	geometry_test();
	println!("\n--- Elevator Simulation Test ---");
	elevator_test();
	println!("\n--- Arithmetic Expression Evaluation Test ---");
	println!("Run cargo test to execute the arithmetic tests.");
	println!("\n--- Logger Functionality Test ---");
	logger_test();
	println!("\n--- Generics Functionality Test ---");
	println!("Run cargo test to execute the generics tests.");
	println!("\n--- Filter Logger Functionality Test ---");
	filter_test();
	println!("\n--- Counter Functionality Test ---");
	counter_test();
	println!("\n--- ROT13 Cipher Test ---");
	println!("Run cargo test to execute the ROT13 tests.");
	println!("\n--- Complex Data Builder Test ---");
	builder_test();
	println!("\n--- Binary Tree Functionality Test ---");
	println!("Run cargo test to execute the binary tree tests.");
	println!("\n--- Health Report System Test ---");
	println!("Run cargo test to execute the health report tests.");
}
