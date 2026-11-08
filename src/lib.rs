//! SigmaOS-ab — ApacheBench HTTP benchmarking component.
//!
//! Models HTTP load testing with concurrency, requests, keep-alive,
//! headers, SSL, and statistics for SigmaOS benchmarking tools.

pub mod request;
pub mod bench;
pub mod stats;
pub mod config;
pub mod manager;
pub mod options;

pub use request::{HttpRequest, HttpMethod, HttpHeader, HttpResponse};
pub use bench::{Benchmark, BenchmarkState, BenchmarkConfig, BenchmarkResult};
pub use stats::{BenchStats, LatencyStats, ThroughputStats};
pub use config::{BenchConfig, TargetConfig, SslConfig};
pub use manager::{BenchManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_http_method() {
        let methods = vec![
            HttpMethod::GET,
            HttpMethod::POST,
            HttpMethod::HEAD,
            HttpMethod::PUT,
            HttpMethod::DELETE,
            HttpMethod::OPTIONS,
        ];
        assert_eq!(methods.len(), 6);
    }
    
    #[test]
    fn test_benchmark_state() {
        let states = vec![
            BenchmarkState::Idle,
            BenchmarkState::Running,
            BenchmarkState::Paused,
            BenchmarkState::Completed,
            BenchmarkState::Failed,
        ];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_request_creation() {
        let req = HttpRequest::new("GET".into(), "http://example.com/".into());
        assert_eq!(req.method, HttpMethod::GET);
        assert_eq!(req.url, "http://example.com/");
        assert!(req.headers.is_empty());
    }
    
    #[test]
    fn test_request_with_headers() {
        let mut req = HttpRequest::new("POST".into(), "http://example.com/api".into());
        req.add_header("Content-Type".into(), "application/json".into());
        req.set_body("{\"key\":\"value\"}".into());
        
        assert_eq!(req.headers.len(), 1);
        assert!(req.body.is_some());
    }
    
    #[test]
    fn test_benchmark_creation() {
        let bench = Benchmark::new("bench-1".into());
        assert_eq!(bench.id, "bench-1");
        assert_eq!(bench.state, BenchmarkState::Idle);
    }
    
    #[test]
    fn test_benchmark_config() {
        let mut config = BenchConfig::default();
        config.concurrency = 100;
        config.requests = 10000;
        config.keep_alive = true;
        
        assert_eq!(config.concurrency, 100);
        assert_eq!(config.requests, 10000);
        assert!(config.keep_alive);
    }
    
    #[test]
    fn test_target_config() {
        let target = TargetConfig::new("http://example.com".into());
        assert_eq!(target.url, "http://example.com");
        assert!(!target.use_ssl);
    }
    
    #[test]
    fn test_ssl_config() {
        let ssl = SslConfig::new();
        assert!(ssl.verify);
        assert_eq!(ssl.ciphers, "DEFAULT");
    }
    
    #[test]
    fn test_bench_stats() {
        let mut stats = BenchStats::new();
        stats.record_request(100, 200, true);
        stats.record_request(150, 200, true);
        stats.record_request(50, 500, false);
        
        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.successful_requests, 2);
        assert_eq!(stats.failed_requests, 1);
    }
    
    #[test]
    fn test_latency_stats() {
        let mut stats = LatencyStats::new();
        stats.record(10);
        stats.record(20);
        stats.record(30);
        
        assert_eq!(stats.min, 10);
        assert_eq!(stats.max, 30);
        assert_eq!(stats.mean, 20);
    }
    
    #[test]
    fn test_throughput_stats() {
        let mut stats = ThroughputStats::new();
        stats.record(1000, 10); // 1000 bytes in 10 seconds = 100 B/s
        
        assert_eq!(stats.bytes_per_second, 100.0);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = BenchManager::new();
        assert!(manager.list_benchmarks().is_empty());
    }
    
    #[test]
    fn test_create_benchmark() {
        let mut manager = BenchManager::new();
        let bench_id = manager.create_benchmark("test-bench".into());
        
        assert!(manager.get_benchmark(&bench_id).is_some());
        assert_eq!(manager.list_benchmarks().len(), 1);
    }
    
    #[test]
    fn test_start_benchmark() {
        let mut manager = BenchManager::new();
        let bench_id = manager.create_benchmark("test".into());
        
        assert!(manager.start_benchmark(&bench_id));
        assert_eq!(manager.get_benchmark(&bench_id).unwrap().state, BenchmarkState::Running);
    }
    
    #[test]
    fn test_stop_benchmark() {
        let mut manager = BenchManager::new();
        let bench_id = manager.create_benchmark("test".into());
        manager.start_benchmark(&bench_id);
        
        assert!(manager.stop_benchmark(&bench_id));
        assert_eq!(manager.get_benchmark(&bench_id).unwrap().state, BenchmarkState::Completed);
    }
    
    #[test]
    fn test_record_request() {
        let mut manager = BenchManager::new();
        let bench_id = manager.create_benchmark("test".into());
        
        assert!(manager.record_request(&bench_id, 100, 200, true));
        assert!(manager.record_request(&bench_id, 150, 200, true));
        assert!(manager.record_request(&bench_id, 50, 500, false));
        
        let bench = manager.get_benchmark(&bench_id).unwrap();
        assert_eq!(bench.stats.total_requests, 3);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = BenchManager::new();
        manager.create_benchmark("bench1".into());
        
        let export = manager.export_data();
        
        let mut manager2 = BenchManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_benchmarks().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = BenchManager::new();
        let bench_id = manager.create_benchmark("test".into());
        manager.start_benchmark(&bench_id);
        manager.record_request(&bench_id, 100, 200, true);
        manager.stop_benchmark(&bench_id);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_benchmarks, 1);
        assert_eq!(stats.completed_benchmarks, 1);
        assert_eq!(stats.total_requests, 1);
    }
}
