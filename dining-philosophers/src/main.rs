use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick;

struct Philosopher {
	name: String,
	left_chopstick: Arc<Mutex<Chopstick>>,
	right_chopstick: Arc<Mutex<Chopstick>>,
	thoughts: mpsc::Sender<String>,
}

impl Philosopher {
	fn think(&self) {
		self.thoughts
			.send(format!("Eureka! {} has a new idea!", &self.name))
			.unwrap();
	}

	fn eat(&self) {
		// To avoid deadlock, always pick up the lower-numbered chopstick first
		let (first, second) = if self.name == PHILOSOPHERS[0] {
			(&self.left_chopstick, &self.right_chopstick)
		} else {
			(&self.right_chopstick, &self.left_chopstick)
		};
		let _first_lock = first.lock().unwrap();
		let _second_lock = second.lock().unwrap();
		println!("{} is eating...", &self.name);
		thread::sleep(Duration::from_millis(10));
	}
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
	// Create chopsticks
	let chopsticks: Vec<Arc<Mutex<Chopstick>>> = (0..PHILOSOPHERS.len())
		.map(|_| Arc::new(Mutex::new(Chopstick)))
		.collect();

	// Create philosophers
	let (tx, rx) = mpsc::channel();
	let philosophers: Vec<Philosopher> = PHILOSOPHERS
		.iter()
		.enumerate()
		.map(|(i, &name)| Philosopher {
			name: name.to_string(),
			left_chopstick: chopsticks[i].clone(),
			right_chopstick: chopsticks[(i + 1) % PHILOSOPHERS.len()].clone(),
			thoughts: tx.clone(),
		})
		.collect();

	// Make each of them think and eat 100 times

	let handles: Vec<_> = philosophers
		.into_iter()
		.map(|philosopher| {
			thread::spawn(move || {
				for _ in 0..100 {
					philosopher.think();
					philosopher.eat();
				}
			})
		})
		.collect();

	// Output their thoughts

	drop(tx); // Close the sending side to avoid deadlock

	for thought in rx {
		println!("{}", thought);
	}
	for handle in handles {
		handle.join().unwrap();
	}
}
