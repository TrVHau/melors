use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use rustfft::num_complex::Complex32;
use rustfft::{Fft, FftPlanner};

const ANALYSIS_BANDS: usize = 36;
const FFT_WINDOW_SIZE: usize = 1024;
const FFT_HOP_SIZE: usize = 4096;
const MAX_ANALYSIS_CACHE_ITEMS: usize = 24;

pub struct Player {
    backend: Backend,
    current_path: Option<PathBuf>,
    current_mtime: Option<i64>,
    base_position_secs: i64,
    started_at: Option<Instant>,
    paused: bool,
    volume: f32,
    analysis: VisualizerAnalysis,
    analysis_cache: HashMap<AnalysisCacheKey, VisualizerAnalysis>,
    analysis_cache_order: VecDeque<AnalysisCacheKey>,
    analysis_pending: HashSet<AnalysisCacheKey>,
    analysis_tx: Sender<(AnalysisCacheKey, VisualizerAnalysis)>,
    analysis_rx: Receiver<(AnalysisCacheKey, VisualizerAnalysis)>,
    current_analysis_key: Option<AnalysisCacheKey>,
}

#[derive(Clone, Default)]
struct VisualizerAnalysis {
    frames: Vec<Vec<f32>>,
    frame_duration_secs: f32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct AnalysisCacheKey {
    path: PathBuf,
    mtime: i64,
}

struct Backend {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    sink: Option<Sink>,
}

impl Player {
    pub fn new() -> Result<Self> {
        let (stream, handle) =
            OutputStream::try_default().context("failed to init audio output")?;
        let (analysis_tx, analysis_rx) = mpsc::channel();
        Ok(Self {
            backend: Backend {
                _stream: stream,
                handle,
                sink: None,
            },
            current_path: None,
            current_mtime: None,
            base_position_secs: 0,
            started_at: None,
            paused: true,
            volume: 1.0,
            analysis: VisualizerAnalysis::default(),
            analysis_cache: HashMap::new(),
            analysis_cache_order: VecDeque::new(),
            analysis_pending: HashSet::new(),
            analysis_tx,
            analysis_rx,
            current_analysis_key: None,
        })
    }

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

        let sink = Self::build_sink(&self.backend.handle, path, start_secs.max(0) as u64)?;
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
            let sink = Self::build_sink(&self.backend.handle, &path, position_secs.max(0) as u64)?;
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
            if self.current_analysis_key.as_ref() == Some(&key) {
                self.analysis = analysis.clone();
            }
            self.insert_analysis_cache(key, analysis);
        }
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

    fn stop_current(&mut self) {
        if let Some(old) = self.backend.sink.take() {
            old.stop();
        }
    }

    fn build_sink(handle: &OutputStreamHandle, path: &Path, start_secs: u64) -> Result<Sink> {
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

    fn insert_analysis_cache(&mut self, key: AnalysisCacheKey, analysis: VisualizerAnalysis) {
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

    fn spawn_analysis(&mut self, path: PathBuf, key: AnalysisCacheKey) {
        if self.analysis_pending.contains(&key) {
            return;
        }
        self.analysis_pending.insert(key.clone());

        let tx = self.analysis_tx.clone();
        std::thread::spawn(move || {
            if let Ok(analysis) = Self::analyze_file(&path) {
                let _ = tx.send((key, analysis));
            }
        });
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
