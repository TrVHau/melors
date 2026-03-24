use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player as RodioPlayer, Source};

mod analysis;
mod control;

const ANALYSIS_BANDS: usize = 36;
const MAX_ANALYSIS_CACHE_ITEMS: usize = 24;
const MAX_CONCURRENT_ANALYSIS_JOBS: usize = 2;
const MAX_QUEUED_ANALYSIS_JOBS: usize = 32;

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
    analysis_queue: VecDeque<(PathBuf, AnalysisCacheKey)>,
    analysis_active_jobs: usize,
    analysis_tx: Sender<(AnalysisCacheKey, Option<VisualizerAnalysis>)>,
    analysis_rx: Receiver<(AnalysisCacheKey, Option<VisualizerAnalysis>)>,
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
    stream: MixerDeviceSink,
    sink: Option<RodioPlayer>,
}

impl Player {
    pub fn new() -> Result<Self> {
        let stream = DeviceSinkBuilder::open_default_sink().context("failed to init audio output")?;
        let (analysis_tx, analysis_rx) = mpsc::channel();
        Ok(Self {
            backend: Backend {
                stream,
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
            analysis_queue: VecDeque::new(),
            analysis_active_jobs: 0,
            analysis_tx,
            analysis_rx,
            current_analysis_key: None,
        })
    }
}
