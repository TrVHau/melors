use super::*;

impl Player {
    pub(super) fn build_sink(handle: &OutputStreamHandle, path: &Path, start_secs: u64) -> Result<Sink> {
        let file =
            File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).context("failed to decode audio file")?;
        let source = decoder.skip_duration(Duration::from_secs(start_secs));
        let sink = Sink::try_new(handle).context("failed to create audio sink")?;
        sink.append(source);
        Ok(sink)
    }

    fn analyze_file(path: &Path) -> Result<VisualizerAnalysis> {
        let file =
            File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).context("failed to decode audio file for analysis")?;
        let channels = decoder.channels().max(1) as usize;
        let sample_rate = decoder.sample_rate().max(1) as usize;

        let hann_window: Vec<f32> = (0..FFT_WINDOW_SIZE)
            .map(|idx| {
                0.5 - 0.5
                    * ((2.0 * std::f32::consts::PI * idx as f32) / FFT_WINDOW_SIZE as f32).cos()
            })
            .collect();
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(FFT_WINDOW_SIZE);
        let mut samples = VecDeque::with_capacity(FFT_WINDOW_SIZE * 2);
        let mut frames = Vec::new();
        let mut channel_accum = 0.0f32;
        let mut channel_count = 0usize;

        for sample in decoder.convert_samples::<f32>() {
            channel_accum += sample;
            channel_count += 1;

            if channel_count == channels {
                samples.push_back(channel_accum / channels as f32);
                channel_accum = 0.0;
                channel_count = 0;

                if samples.len() >= FFT_WINDOW_SIZE {
                    frames.push(Self::fft_frame(&samples, &hann_window, fft.as_ref()));
                    for _ in 0..FFT_HOP_SIZE.min(samples.len()) {
                        samples.pop_front();
                    }
                }
            }
        }

        if frames.is_empty() && !samples.is_empty() {
            while samples.len() < FFT_WINDOW_SIZE {
                samples.push_back(0.0);
            }
            frames.push(Self::fft_frame(&samples, &hann_window, fft.as_ref()));
        }

        let max_value = frames
            .iter()
            .flat_map(|frame| frame.iter().copied())
            .fold(0.0f32, f32::max);
        if max_value > 0.0 {
            for frame in &mut frames {
                for band in frame {
                    *band /= max_value;
                }
            }
        }

        Ok(VisualizerAnalysis {
            frames,
            frame_duration_secs: FFT_HOP_SIZE as f32 / sample_rate as f32,
        })
    }

    pub(super) fn insert_analysis_cache(&mut self, key: AnalysisCacheKey, analysis: VisualizerAnalysis) {
        if let Some(cached) = self.analysis_cache.get_mut(&key) {
            *cached = analysis;
            return;
        }

        if self.analysis_cache_order.len() >= MAX_ANALYSIS_CACHE_ITEMS
            && let Some(oldest) = self.analysis_cache_order.pop_front()
        {
            self.analysis_cache.remove(&oldest);
        }

        self.analysis_cache_order.push_back(key.clone());
        self.analysis_cache.insert(key, analysis);
    }

    pub(super) fn spawn_analysis(&mut self, path: PathBuf, key: AnalysisCacheKey) {
        if self.analysis_pending.contains(&key) {
            return;
        }
        self.analysis_pending.insert(key.clone());
        self.analysis_queue.push_back((path, key));

        while self.analysis_queue.len() > MAX_QUEUED_ANALYSIS_JOBS {
            if let Some((_, dropped_key)) = self.analysis_queue.pop_front() {
                self.analysis_pending.remove(&dropped_key);
            }
        }

        self.try_start_analysis_jobs();
    }

    pub(super) fn try_start_analysis_jobs(&mut self) {
        while self.analysis_active_jobs < MAX_CONCURRENT_ANALYSIS_JOBS {
            let Some((path, key)) = self.analysis_queue.pop_front() else {
                break;
            };

            self.analysis_active_jobs += 1;
            let tx = self.analysis_tx.clone();
            std::thread::spawn(move || {
                let analysis = Self::analyze_file(&path).ok();
                let _ = tx.send((key, analysis));
            });
        }
    }

    fn fft_frame(samples: &VecDeque<f32>, hann_window: &[f32], fft: &dyn Fft<f32>) -> Vec<f32> {
        let mut buffer: Vec<Complex32> = samples
            .iter()
            .take(FFT_WINDOW_SIZE)
            .zip(hann_window.iter())
            .map(|(sample, weight)| Complex32::new(sample * weight, 0.0))
            .collect();
        buffer.resize(FFT_WINDOW_SIZE, Complex32::new(0.0, 0.0));
        fft.process(&mut buffer);

        let nyquist = FFT_WINDOW_SIZE / 2;
        let mut bands = vec![0.0f32; ANALYSIS_BANDS];
        for (band_idx, band) in bands.iter_mut().enumerate() {
            let start = Self::band_start(band_idx, nyquist);
            let end = Self::band_start(band_idx + 1, nyquist)
                .max(start + 1)
                .min(nyquist);
            let sum: f32 = buffer[start..end].iter().map(|bin| bin.norm()).sum();
            *band = if end > start {
                (sum / (end - start) as f32).sqrt()
            } else {
                0.0
            };
        }

        bands
    }

    fn band_start(band_idx: usize, nyquist: usize) -> usize {
        if band_idx == 0 {
            return 1;
        }

        let min_ln = 1.0f32.ln();
        let max_ln = (nyquist.max(2) as f32).ln();
        let ratio = band_idx as f32 / ANALYSIS_BANDS as f32;
        (min_ln + (max_ln - min_ln) * ratio).exp().round() as usize
    }
}
