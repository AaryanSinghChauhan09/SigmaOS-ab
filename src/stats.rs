//! Statistics for ApacheBench.

use std::collections::BTreeMap;

/// Benchmark stats
#[derive(Debug, Clone)]
pub struct BenchStats {
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Status code counts
    pub status_codes: BTreeMap<u16, u64>,
    /// Response times in ms
    pub response_times: Vec<u64>,
    /// Total bytes transferred
    pub total_bytes: u64,
    /// Stats created timestamp
    pub created_at: u64,
    /// Stats updated timestamp
    pub updated_at: u64,
}

impl BenchStats {
    /// Create new benchmark stats
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            status_codes: BTreeMap::new(),
            response_times: vec![],
            total_bytes: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Record request
    pub fn record_request(&mut self, response_time_ms: u64, status_code: u16, success: bool) {
        self.total_requests += 1;
        self.response_times.push(response_time_ms);
        *self.status_codes.entry(status_code).or_insert(0) += 1;
        
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        
        self.update_timestamp();
    }
    
    /// Record bytes
    pub fn record_bytes(&mut self, bytes: u64) {
        self.total_bytes += bytes;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Calculate average response time
    pub fn avg_response_time(&self) -> f64 {
        if self.response_times.is_empty() {
            return 0.0;
        }
        self.response_times.iter().sum::<u64>() as f64 / self.response_times.len() as f64
    }
    
    /// Get min response time
    pub fn min_response_time(&self) -> f64 {
        self.response_times.iter().min().copied().unwrap_or(0) as f64
    }
    
    /// Get max response time
    pub fn max_response_time(&self) -> f64 {
        self.response_times.iter().max().copied().unwrap_or(0) as f64
    }
    
    /// Get percentile response time
    pub fn percentile_response_time(&self, percentile: f64) -> f64 {
        if self.response_times.is_empty() {
            return 0.0;
        }
        
        let mut sorted = self.response_times.clone();
        sorted.sort();
        
        let index = ((percentile / 100.0) * (sorted.len() - 1) as f64) as usize;
        sorted.get(index).copied().unwrap_or(0) as f64
    }
    
    /// Get 50th percentile
    pub fn p50_response_time(&self) -> f64 {
        self.percentile_response_time(50.0)
    }
    
    /// Get 95th percentile
    pub fn p95_response_time(&self) -> f64 {
        self.percentile_response_time(95.0)
    }
    
    /// Get 99th percentile
    pub fn p99_response_time(&self) -> f64 {
        self.percentile_response_time(99.0)
    }
    
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        }
    }
    
    /// Get error rate
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.failed_requests as f64 / self.total_requests as f64) * 100.0
        }
    }
    
    /// Get requests per second
    pub fn requests_per_second(&self, duration_seconds: f64) -> f64 {
        if duration_seconds == 0.0 {
            0.0
        } else {
            self.total_requests as f64 / duration_seconds
        }
    }
    
    /// Get bytes per second
    pub fn bytes_per_second(&self, duration_seconds: f64) -> f64 {
        if duration_seconds == 0.0 {
            0.0
        } else {
            self.total_bytes as f64 / duration_seconds
        }
    }
    
    /// Get status code count
    pub fn get_status_code_count(&self, status_code: u16) -> u64 {
        self.status_codes.get(&status_code).copied().unwrap_or(0)
    }
    
    /// Get all status codes
    pub fn get_status_codes(&self) -> &BTreeMap<u16, u64> {
        &self.status_codes
    }
    
    /// Reset stats
    pub fn reset(&mut self) {
        self.total_requests = 0;
        self.successful_requests = 0;
        self.failed_requests = 0;
        self.status_codes.clear();
        self.response_times.clear();
        self.total_bytes = 0;
        self.update_timestamp();
    }
    
    /// Get stats age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.updated_at
    }
    
    /// Check if stats have requests
    pub fn has_requests(&self) -> bool {
        self.total_requests > 0
    }
    
    /// Check if stats have successful requests
    pub fn has_successful_requests(&self) -> bool {
        self.successful_requests > 0
    }
    
    /// Check if stats have failed requests
    pub fn has_failed_requests(&self) -> bool {
        self.failed_requests > 0
    }
}

impl Default for BenchStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Latency stats
#[derive(Debug, Clone)]
pub struct LatencyStats {
    /// Latency samples in ms
    pub samples: Vec<u64>,
    /// Min latency
    pub min: u64,
    /// Max latency
    pub max: u64,
    /// Mean latency
    pub mean: u64,
    /// Created timestamp
    pub created_at: u64,
}

impl LatencyStats {
    /// Create new latency stats
    pub fn new() -> Self {
        Self {
            samples: vec![],
            min: u64::MAX,
            max: 0,
            mean: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Record latency
    pub fn record(&mut self, latency_ms: u64) {
        self.samples.push(latency_ms);
        self.min = self.min.min(latency_ms);
        self.max = self.max.max(latency_ms);
        self.mean = self.samples.iter().sum::<u64>() / self.samples.len() as u64;
    }
    
    /// Get sample count
    pub fn get_sample_count(&self) -> usize {
        self.samples.len()
    }
}

impl Default for LatencyStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Throughput stats
#[derive(Debug, Clone)]
pub struct ThroughputStats {
    /// Bytes per second
    pub bytes_per_second: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Created timestamp
    pub created_at: u64,
}

impl ThroughputStats {
    /// Create new throughput stats
    pub fn new() -> Self {
        Self {
            bytes_per_second: 0.0,
            requests_per_second: 0.0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Record throughput
    pub fn record(&mut self, bytes: u64, duration_seconds: u64) {
        if duration_seconds > 0 {
            self.bytes_per_second = bytes as f64 / duration_seconds as f64;
        }
    }
}

impl Default for ThroughputStats {
    fn default() -> Self {
        Self::new()
    }
}
