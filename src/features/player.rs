use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct Player {
	backend: Backend,
	current_path: Option<PathBuf>,
	base_position_secs: i64,
	started_at: Option<Instant>,
	paused: bool,
}

struct Backend {
	_stream: OutputStream,
	handle: OutputStreamHandle,
	sink: Option<Sink>,
}

impl Player {
	pub fn new() -> Result<Self> {
		let (stream, handle) = OutputStream::try_default().context("failed to init audio output")?;
		Ok(Self {
			backend: Backend {
				_stream: stream,
				handle,
				sink: None,
			},
			current_path: None,
			base_position_secs: 0,
			started_at: None,
			paused: true,
		})
	}

	pub fn play_file(&mut self, path: &Path, start_secs: i64) -> Result<()> {
		let sink = Self::build_sink(&self.backend.handle, path, start_secs.max(0) as u64)?;
		self.stop_current();
		self.backend.sink = Some(sink);
		self.current_path = Some(path.to_path_buf());
		self.base_position_secs = start_secs.max(0);
		self.started_at = Some(Instant::now());
		self.paused = false;
		Ok(())
	}

	pub fn toggle_pause(&mut self) -> bool {
		if let Some(sink) = self.backend.sink.as_ref() {
			if self.paused {
				sink.play();
				self.started_at = Some(Instant::now());
				self.paused = false;
			} else {
				self.base_position_secs = self.current_position_secs();
				sink.pause();
				self.started_at = None;
				self.paused = true;
			}
		}
		self.paused
	}

	pub fn seek_relative(&mut self, delta_secs: i64) -> Result<i64> {
		let next = (self.current_position_secs() + delta_secs).max(0);
		self.seek_to(next)?;
		Ok(next)
	}

	pub fn seek_to(&mut self, position_secs: i64) -> Result<()> {
		let was_paused = self.paused;
		if let Some(path) = self.current_path.clone() {
			let sink = Self::build_sink(&self.backend.handle, &path, position_secs.max(0) as u64)?;
			self.stop_current();
			if was_paused {
				sink.pause();
				self.started_at = None;
				self.paused = true;
			} else {
				self.started_at = Some(Instant::now());
				self.paused = false;
			}
			self.backend.sink = Some(sink);
			self.base_position_secs = position_secs.max(0);
		}
		Ok(())
	}

	pub fn current_position_secs(&self) -> i64 {
		if self.paused || self.started_at.is_none() {
			return self.base_position_secs.max(0);
		}

		let elapsed = self
			.started_at
			.map(|t| t.elapsed().as_secs() as i64)
			.unwrap_or(0);
		(self.base_position_secs + elapsed).max(0)
	}

	pub fn has_active_sink(&self) -> bool {
		self.backend.sink.is_some()
	}

	pub fn consume_track_finished(&mut self) -> bool {
		let finished = self
			.backend
			.sink
			.as_ref()
			.map(|sink| !self.paused && sink.empty())
			.unwrap_or(false);

		if finished {
			self.backend.sink = None;
			self.started_at = None;
			self.base_position_secs = 0;
			self.paused = true;
		}

		finished
	}

	fn stop_current(&mut self) {
		if let Some(old) = self.backend.sink.take() {
			old.stop();
		}
	}

	fn build_sink(handle: &OutputStreamHandle, path: &Path, start_secs: u64) -> Result<Sink> {
		let file = File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
		let reader = BufReader::new(file);
		let decoder = Decoder::new(reader).context("failed to decode audio file")?;
		let source = decoder.skip_duration(Duration::from_secs(start_secs));
		let sink = Sink::try_new(handle).context("failed to create audio sink")?;
		sink.append(source);
		Ok(sink)
	}
}
