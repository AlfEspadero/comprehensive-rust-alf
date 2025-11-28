// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(point: &[f64; 3]) -> f64 {
	let mut sum_of_squares = 0.0;
	for p in point {
		sum_of_squares += p * p;
	}
	sum_of_squares.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(point: &mut [f64; 3]) {
	let mag = magnitude(point);
	for p in point {
		*p /= mag;
	}
}

// Use the following `main` to test your work.

fn test_geometry() {
	println!(
		"Magnitude of a unit vector: {}",
		magnitude(&[0.0, 1.0, 0.0])
	);

	let mut v = [1.0, 2.0, 9.0];
	println!("Magnitude of {v:?}: {}", magnitude(&v));
	normalize(&mut v);
	println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
