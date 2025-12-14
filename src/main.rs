// Allow dead code for elevator
#![allow(dead_code)]

mod arithmetic;
mod btree;
mod builder;
mod collatz;
mod counter;
mod elevator;
mod generics;
mod geometry;
mod health;
mod iterators;
mod logger;
mod protobuf;
mod rot13;
mod transpose;

fn main() {
	println!("\t\t------------------ Google's Comprehensive Rust Course ------------------");
	println!("\t\t\t\t\tAttempted by AlfEspadero\n");

	println!("--- Collatz Sequence Length Test ---");
	collatz::collatz_length_test();
	println!("\n--- Matrix Transpose Test ---");
	transpose::transpose_test();
	println!("\n--- 3D Geometry Test ---");
	geometry::geometry_test();
	println!("\n--- Elevator Simulation Test ---");
	elevator::elevator_test();
	println!("\n--- Arithmetic Expression Evaluation Test ---");
	println!("Run cargo test to execute the arithmetic tests.");
	println!("\n--- Logger Functionality Test ---");
	logger::logger_test();
	println!("\n--- Generics Functionality Test ---");
	println!("Run cargo test to execute the generics tests.");
	println!("\n--- Filter Logger Functionality Test ---");
	logger::filter_test();
	println!("\n--- Counter Functionality Test ---");
	counter::counter_test();
	println!("\n--- ROT13 Cipher Test ---");
	println!("Run cargo test to execute the ROT13 tests.");
	println!("\n--- Complex Data Builder Test ---");
	builder::builder_test();
	println!("\n--- Binary Tree Functionality Test ---");
	println!("Run cargo test to execute the binary tree tests.");
	println!("\n--- Health Report System Test ---");
	println!("Run cargo test to execute the health report tests.");
	println!("\n--- Protobuf Serialization/Deserialization Test ---");
	println!("Run cargo test to execute the protobuf tests.");
	println!("\n--- Iterators Functionality Test ---");
	println!("Run cargo test to execute the iterators tests.");
}
