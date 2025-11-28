/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
	Add,
	Sub,
	Mul,
	Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
	/// An operation on two subexpressions.
	Op {
		op: Operation,
		left: Box<Expression>,
		right: Box<Expression>,
	},

	/// A literal value
	Value(i64),
}

fn eval(e: Expression) -> i64 {
	match e {
		Expression::Value(v) => v,
		Expression::Op { op, left, right } => {
			let l_val = eval(*left);
			let r_val = eval(*right);
			match op {
				Operation::Add => l_val + r_val,
				Operation::Sub => l_val - r_val,
				Operation::Mul => l_val * r_val,
				Operation::Div => if r_val != 0 { l_val / r_val } else {panic!("Division by zero")},
			}
		}
	}
}

#[test]
fn test_value() {
	assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
	assert_eq!(
		eval(Expression::Op {
			op: Operation::Add,
			left: Box::new(Expression::Value(10)),
			right: Box::new(Expression::Value(20)),
		}),
		30
	);
}

#[test]
fn test_recursion() {
	let term1 = Expression::Op {
		op: Operation::Mul,
		left: Box::new(Expression::Value(10)),
		right: Box::new(Expression::Value(9)),
	};
	let term2 = Expression::Op {
		op: Operation::Mul,
		left: Box::new(Expression::Op {
			op: Operation::Sub,
			left: Box::new(Expression::Value(3)),
			right: Box::new(Expression::Value(4)),
		}),
		right: Box::new(Expression::Value(5)),
	};
	assert_eq!(
		eval(Expression::Op {
			op: Operation::Add,
			left: Box::new(term1),
			right: Box::new(term2),
		}),
		85
	);
}

#[test]
fn test_zeros() {
	assert_eq!(
		eval(Expression::Op {
			op: Operation::Add,
			left: Box::new(Expression::Value(0)),
			right: Box::new(Expression::Value(0))
		}),
		0
	);
	assert_eq!(
		eval(Expression::Op {
			op: Operation::Mul,
			left: Box::new(Expression::Value(0)),
			right: Box::new(Expression::Value(0))
		}),
		0
	);
	assert_eq!(
		eval(Expression::Op {
			op: Operation::Sub,
			left: Box::new(Expression::Value(0)),
			right: Box::new(Expression::Value(0))
		}),
		0
	);
}

#[test]
fn test_div() {
	assert_eq!(
		eval(Expression::Op {
			op: Operation::Div,
			left: Box::new(Expression::Value(10)),
			right: Box::new(Expression::Value(2)),
		}),
		5
	)
}
