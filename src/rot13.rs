use std::io::Read;

struct RotDecoder<R: Read> {
	input: R,
	rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.

impl<R: Read> Read for RotDecoder<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let bytes_read = self.input.read(buf)?;
		for byte in &mut buf[..bytes_read] {
			if byte.is_ascii_alphabetic() {
				let base = if byte.is_ascii_lowercase() {
					b'a'
				} else {
					b'A'
				};
				*byte = (((*byte - base + self.rot) % 26) + base) as u8;
			}
		}
		Ok(bytes_read)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn joke() {
		let mut rot = RotDecoder {
			input: "Gb trg gb gur bgure fvqr!".as_bytes(),
			rot: 13,
		};
		let mut result = String::new();
		rot.read_to_string(&mut result).unwrap();
		assert_eq!(&result, "To get to the other side!");
	}

	#[test]
	fn binary() {
		let input: Vec<u8> = (0..=255u8).collect();
		let mut rot = RotDecoder::<&[u8]> {
			input: input.as_slice(),
			rot: 13,
		};
		let mut buf = [0u8; 256];
		assert_eq!(rot.read(&mut buf).unwrap(), 256);
		for i in 0..=255 {
			if input[i] != buf[i] {
				assert!(input[i].is_ascii_alphabetic());
				assert!(buf[i].is_ascii_alphabetic());
			}
		}
	}

	#[test]
	fn chained13() {
		let input = "Uryyb Jbeyq!".as_bytes();
		let mut rot1 = RotDecoder { input, rot: 13 };
		let mut rot2 = RotDecoder {
			input: &mut rot1,
			rot: 13,
		};
		let mut result = String::new();
		rot2.read_to_string(&mut result).unwrap();
		assert_ne!(&result, "Hello World!");
	}
}
