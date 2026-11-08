//! Benchmark manager for ApacheBench.

use std::collections::HashMap;
use crate::bench::{Benchmark, BenchmarkState, BenchmarkConfig};
use crate::stats::BenchStats;
use crate::config::{BenchConfig, TargetConfig, SslConfig};
use crate::request::{HttpRequest, HttpResponse};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max benchmarks
    pub max_benchmarks: usize,
    /// Max concurrent benchmarks
    pub max_concurrent: usize,
    /// Default timeout
    pub default_timeout: u64,
    /// Enable SSL
    pub enable_ssl: bool,
    /// Enable keep-alive
    pub enable_keep_alive: bool,
    /// Enable statistics
    pub enable_stats: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_benchmarks: 100,
            max_concurrent: 10,
            default_timeout: 30,
            enable_ssl: true,
            enable_keep_alive: true,
            enable_stats: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total benchmarks
    pub total_benchmarks: usize,
    /// Running benchmarks
    pub running_benchmarks: usize,
    /// Completed benchmarks
    pub completed_benchmarks: usize,
    /// Failed benchmarks
    pub failed_benchmarks: usize,
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Bench manager
#[derive(Debug, Clone)]
pub struct BenchManager {
    /// Benchmarks
    benchmarks: HashMap<String, Benchmark>,
    /// HTTP requests
    requests: Vec<HttpRequest>,
    /// HTTP responses
    responses: Vec<HttpResponse>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for BenchManager {
    fn default() -> Self {
        Self {
            benchmarks: HashMap::new(),
            requests: vec![],
            responses: vec![],
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_benchmarks: 0,
                running_benchmarks: 0,
                completed_benchmarks: 0,
                failed_benchmarks: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl BenchManager {
    /// Create new bench manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            benchmarks: HashMap::new(),
            requests: vec![],
            responses: vec![],
            options,
            statistics: ManagerStatistics {
                total_benchmarks: 0,
                running_benchmarks: 0,
                completed_benchmarks: 0,
                failed_benchmarks: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Benchmark methods
    
    /// Create benchmark
    pub fn create_benchmark(&mut self, name: String) -> String {
        let benchmark = Benchmark::new(format!("bench-{}", self.benchmarks.len() + 1));
        let id = benchmark.id.clone();
        
        let mut benchmark = benchmark;
        benchmark.set_name(name);
        
        self.benchmarks.insert(id.clone(), benchmark);
        self.update_statistics();
        id
    }
    
    /// Get benchmark
    pub fn get_benchmark(&self, bench_id: &str) -> Option<&Benchmark> {
        self.benchmarks.get(bench_id)
    }
    
    /// Get benchmark mutably
    pub fn get_benchmark_mut(&mut self, bench_id: &str) -> Option<&mut Benchmark> {
        self.benchmarks.get_mut(bench_id)
    }
    
    /// Get all benchmarks
    pub fn list_benchmarks(&self) -> Vec<&Benchmark> {
        self.benchmarks.values().collect()
    }
    
    /// Delete benchmark
    pub fn delete_benchmark(&mut self, bench_id: &str) -> bool {
        if self.benchmarks.remove(bench_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start benchmark
    pub fn start_benchmark(&mut self, bench_id: &str) -> bool {
        if let Some(benchmark) = self.benchmarks.get_mut(bench_id) {
            if benchmark.start() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Stop benchmark
    pub fn stop_benchmark(&mut self, bench_id: &str) -> bool {
        if let Some(benchmark) = self.benchmarks.get_mut(bench_id) {
            if benchmark.stop() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Fail benchmark
    pub fn fail_benchmark(&mut self, bench_id: &str, error: String) -> bool {
        if let Some(benchmark) = self.benchmarks.get_mut(bench_id) {
            if benchmark.fail(error) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Set benchmark config
    pub fn set_benchmark_config(&mut self, bench_id: &str, config: BenchmarkConfig) -> bool {
        if let Some(benchmark) = self.benchmarks.get_mut(bench_id) {
            benchmark.set_config(config);
            return true;
        }
        false
    }
    
    /// Record request for benchmark
    pub fn record_request(&mut self, bench_id: &str, response_time_ms: u64, status_code: u16, success: bool) -> bool {
        if let Some(benchmark) = self.benchmarks.get_mut(bench_id) {
            benchmark.record_request(response_time_ms, status_code, success);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get benchmarks by state
    pub fn get_benchmarks_by_state(&self, state: &BenchmarkState) -> Vec<&Benchmark> {
        self.benchmarks.values()
            .filter(|b| b.state == *state)
            .collect()
    }
    
    /// Get running benchmarks
    pub fn get_running_benchmarks(&self) -> Vec<&Benchmark> {
        self.get_benchmarks_by_state(&BenchmarkState::Running)
    }
    
    /// Get completed benchmarks
    pub fn get_completed_benchmarks(&self) -> Vec<&Benchmark> {
        self.get_benchmarks_by_state(&BenchmarkState::Completed)
    }
    
    /// Get failed benchmarks
    pub fn get_failed_benchmarks(&self) -> Vec<&Benchmark> {
        self.get_benchmarks_by_state(&BenchmarkState::Failed)
    }
    
    /// Search benchmarks
    pub fn search_benchmarks(&self, query: &str) -> Vec<&Benchmark> {
        let query_lower = query.to_lowercase();
        self.benchmarks.values()
            .filter(|b| b.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Request/response methods
    
    /// Add request
    pub fn add_request(&mut self, request: HttpRequest) -> usize {
        self.requests.push(request);
        self.requests.len() - 1
    }
    
    /// Get request
    pub fn get_request(&self, index: usize) -> Option<&HttpRequest> {
        self.requests.get(index)
    }
    
    /// Add response
    pub fn add_response(&mut self, response: HttpResponse) -> usize {
        self.responses.push(response);
        self.responses.len() - 1
    }
    
    /// Get response
    pub fn get_response(&self, index: usize) -> Option<&HttpResponse> {
        self.responses.get(index)
    }
    
    /// Get all requests
    pub fn list_requests(&self) -> &[HttpRequest] {
        &self.requests
    }
    
    /// Get all responses
    pub fn list_responses(&self) -> &[HttpResponse] {
        &self.responses
    }
    
    /// Clear requests and responses
    pub fn clear_requests_responses(&mut self) {
        self.requests.clear();
        self.responses.clear();
    }
    
    // Config methods
    
    /// Create bench config
    pub fn create_bench_config(&self) -> BenchConfig {
        BenchConfig::default()
    }
    
    /// Create target config
    pub fn create_target_config(&self, url: String) -> TargetConfig {
        TargetConfig::new(url)
    }
    
    /// Create SSL config
    pub fn create_ssl_config(&self) -> SslConfig {
        SslConfig::new()
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_benchmarks = self.benchmarks.len();
        self.statistics.running_benchmarks = self.benchmarks.values()
            .filter(|b| b.state == BenchmarkState::Running)
            .count();
        self.statistics.completed_benchmarks = self.benchmarks.values()
            .filter(|b| b.state == BenchmarkState::Completed)
            .count();
        self.statistics.failed_benchmarks = self.benchmarks.values()
            .filter(|b| b.state == BenchmarkState::Failed)
            .count();
        
        self.statistics.total_requests = self.benchmarks.values()
            .map(|b| b.stats.total_requests)
            .sum();
        self.statistics.successful_requests = self.benchmarks.values()
            .map(|b| b.stats.successful_requests)
            .sum();
        self.statistics.failed_requests = self.benchmarks.values()
            .map(|b| b.stats.failed_requests)
            .sum();
        
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            benchmarks: self.benchmarks.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.benchmarks.clear();
        for benchmark in export.benchmarks {
            self.benchmarks.insert(benchmark.id.clone(), benchmark);
        }
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Benchmarks
    pub benchmarks: Vec<Benchmark>,
}
