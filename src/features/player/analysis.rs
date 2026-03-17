use super::*;

impl Player {
    pub(super) fn build_sink(
        handle: &OutputStreamHandle,
        path: &Path,
        start_secs: u64,
    ) -> Result<Sink> {
        let file =
            File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).context("failed to decode audio file")?;
        let source = decoder.skip_duration(Duration::from_secs(start_secs));
        let sink = Sink::try_new(handle).context("failed to create audio sink")?;
        sink.append(source);
        Ok(sink)
    }

    fn analyze_file(_path: &Path) -> Result<VisualizerAnalysis> {
        Ok(VisualizerAnalysis {
            frames: vec![vec![0.0; ANALYSIS_BANDS]],
            frame_duration_secs: 0.2,
        })
    }

    pub(super) fn insert_analysis_cache(
        &mut self,
        key: AnalysisCacheKey,
        analysis: VisualizerAnalysis,
    ) {
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
}
