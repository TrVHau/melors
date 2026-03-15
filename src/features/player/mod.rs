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

mod analysis;
mod control;

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
}
