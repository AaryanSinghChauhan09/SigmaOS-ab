//! Benchmark definitions for ApacheBench.

use crate::stats::BenchStats;
use crate::config::BenchConfig;

/// Benchmark state
#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkState {
    /// Idle benchmark
    Idle,
    /// Running benchmark
    Running,
    /// Paused benchmark
    Paused,
    /// Completed benchmark
    Completed,
    /// Failed benchmark
    Failed,
}

/// Benchmark config
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Bench config
    pub bench_config: BenchConfig,
    /// Target URL
    pub url: String,
    /// Number of requests
    pub requests: u64,
    /// Concurrency
    pub concurrency: u32,
    /// Keep-alive
    pub keep_alive: bool,
    /// Timeout in seconds
    pub timeout: u64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            bench_config: BenchConfig::default(),
            url: String::new(),
            requests: 100,
            concurrency: 10,
            keep_alive: true,
            timeout: 30,
        }
    }
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Benchmark success
    pub success: bool,
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Total time in seconds
    pub total_time: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time in ms
    pub avg_response_time: f64,
    /// Min response time in ms
    pub min_response_time: f64,
    /// Max response time in ms
    pub max_response_time: f64,
    /// 50th percentile response time
    pub p50_response_time: f64,
    /// 95th percentile response time
    pub p95_response_time: f64,
    /// 99th percentile response time
    pub p99_response_time: f64,
    /// Error message
    pub error: Option<String>,
    /// Result timestamp
    pub timestamp: u64,
}

/// Benchmark
#[derive(Debug, Clone)]
pub struct Benchmark {
    /// Benchmark ID
    pub id: String,
    /// Benchmark name
    pub name: String,
    /// Benchmark state
    pub state: BenchmarkState,
    /// Benchmark config
    pub config: BenchmarkConfig,
    /// Benchmark stats
    pub stats: BenchStats,
    /// Benchmark result
    pub result: Option<BenchmarkResult>,
    /// Benchmark created timestamp
    pub created_at: u64,
    /// Benchmark started timestamp
    pub started_at: Option<u64>,
    /// Benchmark completed timestamp
    pub completed_at: Option<u64>,
}

impl Benchmark {
    /// Create new benchmark
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            state: BenchmarkState::Idle,
            config: BenchmarkConfig::default(),
            stats: BenchStats::new(),
            result: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: None,
            completed_at: None,
        }
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Set config
    pub fn set_config(&mut self, config: BenchmarkConfig) {
        self.config = config;
    }
    
    /// Start benchmark
    pub fn start(&mut self) -> bool {
        if self.state == BenchmarkState::Idle {
            self.state = BenchmarkState::Running;
            self.started_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            return true;
        }
        false
    }
    
    /// Stop benchmark
    pub fn stop(&mut self) -> bool {
        if self.state == BenchmarkState::Running {
            self.state = BenchmarkState::Completed;
            self.completed_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            self.generate_result();
            return true;
        }
        false
    }
    
    /// Fail benchmark
    pub fn fail(&mut self, error: String) -> bool {
        if self.state == BenchmarkState::Running {
            self.state = BenchmarkState::Failed;
            self.completed_at = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
            self.result = Some(BenchmarkResult {
                success: false,
                total_requests: self.stats.total_requests,
                successful_requests: self.stats.successful_requests,
                failed_requests: self.stats.failed_requests,
                total_time: 0.0,
                requests_per_second: 0.0,
                avg_response_time: 0.0,
                min_response_time: 0.0,
                max_response_time: 0.0,
                p50_response_time: 0.0,
                p95_response_time: 0.0,
                p99_response_time: 0.0,
                error: Some(error),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
            return true;
        }
        false
    }
    
    /// Record request
    pub fn record_request(&mut self, response_time_ms: u64, status_code: u16, success: bool) {
        self.stats.record_request(response_time_ms, status_code, success);
    }
    
    /// Generate result
    fn generate_result(&mut self) {
        let total_time = self.started_at
            .zip(self.completed_at)
            .map(|(start, end)| (end - start) as f64)
            .unwrap_or(0.0);
        
        let rps = if total_time > 0.0 {
            self.stats.total_requests as f64 / total_time
        } else {
            0.0
        };
        
        self.result = Some(BenchmarkResult {
            success: true,
            total_requests: self.stats.total_requests,
            successful_requests: self.stats.successful_requests,
            failed_requests: self.stats.failed_requests,
            total_time,
            requests_per_second: rps,
            avg_response_time: self.stats.avg_response_time(),
            min_response_time: self.stats.min_response_time(),
            max_response_time: self.stats.max_response_time(),
            p50_response_time: self.stats.p50_response_time(),
            p95_response_time: self.stats.p95_response_time(),
            p99_response_time: self.stats.p99_response_time(),
            error: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    /// Check if benchmark is running
    pub fn is_running(&self) -> bool {
        self.state == BenchmarkState::Running
    }
    
    /// Check if benchmark is completed
    pub fn is_completed(&self) -> bool {
        self.state == BenchmarkState::Completed
    }
    
    /// Check if benchmark is failed
    pub fn is_failed(&self) -> bool {
        self.state == BenchmarkState::Failed
    }
    
    /// Get benchmark duration in seconds
    pub fn get_duration(&self) -> Option<u64> {
        self.started_at.zip(self.completed_at)
            .map(|(start, end)| end - start)
    }
}
