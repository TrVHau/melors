use super::*;

impl Player {
    pub fn play_file(&mut self, path: &Path, mtime: i64, start_secs: i64) -> Result<()> {
        self.poll_analysis_results();

        let cache_key = AnalysisCacheKey {
            path: path.to_path_buf(),
            mtime,
        };
        self.current_analysis_key = Some(cache_key.clone());

        if self.current_path.as_deref() != Some(path) || self.current_mtime != Some(mtime) {
            if let Some(analysis) = self.analysis_cache.get(&cache_key).cloned() {
                self.analysis = analysis;
            } else {
                self.analysis = VisualizerAnalysis::default();
                self.spawn_analysis(path.to_path_buf(), cache_key.clone());
            }
        }

        let sink = Self::build_sink(self.backend.stream.mixer(), path, start_secs.max(0) as u64)?;
        sink.set_volume(self.volume);
        self.stop_current();
        self.backend.sink = Some(sink);
        self.current_path = Some(path.to_path_buf());
        self.current_mtime = Some(mtime);
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
            let sink = Self::build_sink(
                self.backend.stream.mixer(),
                &path,
                position_secs.max(0) as u64,
            )?;
            sink.set_volume(self.volume);
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

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn adjust_volume(&mut self, delta: f32) -> u8 {
        let next = (self.volume + delta).clamp(0.0, 1.5);
        self.volume = next;
        if let Some(sink) = self.backend.sink.as_ref() {
            sink.set_volume(self.volume);
        }
        self.volume_percent()
    }

    pub fn volume_percent(&self) -> u8 {
        (self.volume * 100.0).round() as u8
    }

    pub fn visualizer_levels(&self, bars: usize) -> Vec<(f32, f32)> {
        if bars == 0 || self.analysis.frames.is_empty() {
            return vec![(0.0, 0.0); bars];
        }

        let frame_duration = self.analysis.frame_duration_secs.max(0.001);
        let frame_idx = ((self.current_position_secs().max(0) as f32) / frame_duration) as usize;
        let frame_idx = frame_idx.min(self.analysis.frames.len().saturating_sub(1));
        let frame = &self.analysis.frames[frame_idx];

        (0..bars)
            .map(|bar_idx| {
                let start_band = bar_idx * ANALYSIS_BANDS / bars.max(1);
                let end_band = ((bar_idx + 1) * ANALYSIS_BANDS / bars.max(1))
                    .max(start_band + 1)
                    .min(ANALYSIS_BANDS);
                let mut level = 0.0f32;
                let mut peak = 0.0f32;
                for value in frame.iter().take(end_band).skip(start_band) {
                    level += *value;
                    peak = peak.max(*value);
                }
                let width = (end_band - start_band).max(1) as f32;
                let level = (level / width).clamp(0.0, 1.0);
                let peak = peak.clamp(0.0, 1.0);
                (level, peak)
            })
            .collect()
    }

    pub fn poll_analysis_results(&mut self) {
        while let Ok((key, analysis)) = self.analysis_rx.try_recv() {
            self.analysis_pending.remove(&key);
            self.analysis_active_jobs = self.analysis_active_jobs.saturating_sub(1);

            if let Some(analysis) = analysis {
                if self.current_analysis_key.as_ref() == Some(&key) {
                    self.analysis = analysis.clone();
                }
                self.insert_analysis_cache(key, analysis);
            }
        }

        self.try_start_analysis_jobs();
    }

    pub fn stop(&mut self) {
        self.stop_current();
        self.current_path = None;
        self.current_mtime = None;
        self.current_analysis_key = None;
        self.base_position_secs = 0;
        self.started_at = None;
        self.paused = true;
        self.analysis = VisualizerAnalysis::default();
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

    pub(super) fn stop_current(&mut self) {
        if let Some(old) = self.backend.sink.take() {
            old.stop();
        }
    }
}
