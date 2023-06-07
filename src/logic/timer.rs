use macroquad::time::get_time;
#[derive(Default)]
pub struct Timer {
	start_time: Option<f64>,
	state: TimerState,
	old: f64,
}
#[derive(Default)]
enum TimerState {
	#[default]
	Stopped,
	Running,
	Frozen,
}

impl Timer {
	pub fn clear(&mut self) {
		self.start_time = None;
		self.state = TimerState::Stopped;
	}
	pub fn start(&mut self) {
		self.start_time = Some(get_time());
		self.state = TimerState::Running;
	}
	pub fn elapsed(&self) -> Option<f64> {
		if let TimerState::Frozen = self.state {
			return Some(self.old);
		}
		self.start_time.map(|time| get_time() - time)
	}
	pub fn stop(&mut self) {
		self.old = self.elapsed().unwrap_or(0f64);
		self.state = TimerState::Frozen;
	}
}
