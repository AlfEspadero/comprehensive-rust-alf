trait Logger {
	/// Log a message at the given verbosity level.
	fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
	fn log(&self, verbosity: u8, message: &str) {
		eprintln!("verbosity={verbosity}: {message}");
	}
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter {
	max_verbosity: u8,
	inner: StderrLogger,
}

impl Logger for VerbosityFilter {
	fn log(&self, verbosity: u8, message: &str) {
		if verbosity <= self.max_verbosity {
			self.inner.log(verbosity, message);
		}
	}
}

// Filter messages based on a closure, send those that pass to an inner logger.
struct Filter<L, P> {
	inner: L,
	predicate: P,
}

impl<L, P> Filter<L, P>
where
	L: Logger,
	P: Fn(u8, &str) -> bool,
{
	fn new(inner: L, predicate: P) -> Self {
		Self { inner, predicate }
	}
}
impl<L, P> Logger for Filter<L, P>
where
	L: Logger,
	P: Fn(u8, &str) -> bool,
{
	fn log(&self, verbosity: u8, message: &str) {
		if (self.predicate)(verbosity, message) {
			self.inner.log(verbosity, message);
		}
	}
}

fn logger_test() {
	let logger = VerbosityFilter {
		max_verbosity: 3,
		inner: StderrLogger,
	};
	logger.log(5, "FYI");
	logger.log(2, "Uhoh");
}

fn filter_test() {
	let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
	logger.log(5, "FYI");
	logger.log(1, "yikes, something went wrong");
	logger.log(2, "uhoh");
}
