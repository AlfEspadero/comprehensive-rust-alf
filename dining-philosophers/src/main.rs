use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick;

struct Philosopher {
	name: String,
	left_chopstick: Arc<Mutex<Chopstick>>,
	right_chopstick: Arc<Mutex<Chopstick>>,
	thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
	fn think(&self) {
		self.thoughts
			.send(format!("Eureka! {} has a new idea!", &self.name))
			.unwrap();
	}

	fn eat(&self) {
		let _left = self.left_chopstick.lock().unwrap();
		let _right = self.right_chopstick.lock().unwrap();
		println!("{} is eating...", &self.name);
		thread::sleep(Duration::from_millis(10));
	}
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
	// Create chopsticks
	let chopsticks = PHILOSOPHERS
		.iter()
		.map(|_| Arc::new(Mutex::new(Chopstick)))
		.collect::<Vec<_>>();

	// Create philosophers
	let (tx, rx) = mpsc::sync_channel(10);

	let philosophers: Vec<Philosopher> = PHILOSOPHERS
		.iter()
		.enumerate()
		.map(|(i, &name)| {
			let mut l_chopstick = Arc::clone(&chopsticks[i]);
			let mut r_chopstick = Arc::clone(&chopsticks[(i + 1) % PHILOSOPHERS.len()]);

			// To avoid a deadlock, we have to break the symmetry
			// somewhere. This will swap the chopsticks without deinitializing
			// either of them.
			if i == chopsticks.len() - 1 {
				std::mem::swap(&mut l_chopstick, &mut r_chopstick);
			}

			Philosopher {
				name: name.to_string(),
				left_chopstick: l_chopstick,
				right_chopstick: r_chopstick,
				thoughts: tx.clone(),
			}
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
				println!("{} is done eating.", philosopher.name);
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
