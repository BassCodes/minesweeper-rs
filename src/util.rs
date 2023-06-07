#[rustfmt::skip]
pub const ADJACENT_WITH_CENTER: [(isize, isize); 9] =
	[
		(-1, -1), (0, -1), (1, -1),
		(-1,  0), (0,  0), (1,  0),
		(-1,  1), (0,  1), (1,  1)
	];

#[rustfmt::skip]
pub const ADJACENT_WITHOUT_CENTER: [(isize, isize); 8] =
	[
		(-1, -1), (0, -1), (1, -1),
		(-1,  0),          (1,  0),
		(-1,  1), (0,  1), (1,  1)
	];

// Event Queue
pub struct Events<E> {
	events: Vec<E>,
}

impl<E> Events<E> {
	pub fn add(&mut self, event: E) {
		self.events.push(event);
	}

	pub fn next(&mut self) -> Option<E> {
		if self.events.len() > 0 {
			self.events.pop()
		} else {
			None
		}
	}
	pub fn clear(&mut self) {
		self.events.clear();
	}
}
impl<E> Default for Events<E> {
	fn default() -> Self {
		Self {
			events: Vec::<E>::with_capacity(10),
		}
	}
}
