//! Configuration for ApacheBench.

/// Bench config
#[derive(Debug, Clone)]
pub struct BenchConfig {
    /// Number of requests
    pub requests: u64,
    /// Concurrency level
    pub concurrency: u32,
    /// Time limit in seconds
    pub time_limit: u64,
    /// Keep-alive connections
    pub keep_alive: bool,
    /// Timeout in seconds
    pub timeout: u64,
    /// Window size
    pub window_size: u32,
    /// Server name
    pub server_name: String,
    /// Port
    pub port: u16,
    /// Use SSL
    pub use_ssl: bool,
    /// SSL verify
    pub ssl_verify: bool,
    /// Headers
    pub headers: Vec<(String, String)>,
    /// POST data
    pub post_data: Option<String>,
    /// POST content type
    pub post_content_type: String,
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            requests: 100,
            concurrency: 10,
            time_limit: 30,
            keep_alive: true,
            timeout: 30,
            window_size: 8192,
            server_name: String::new(),
            port: 80,
            use_ssl: false,
            ssl_verify: true,
            headers: vec![],
            post_data: None,
            post_content_type: "application/x-www-form-urlencoded".into(),
        }
    }
}

impl BenchConfig {
    /// Create new bench config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set requests
    pub fn set_requests(&mut self, requests: u64) {
        self.requests = requests;
    }
    
    /// Set concurrency
    pub fn set_concurrency(&mut self, concurrency: u32) {
        self.concurrency = concurrency;
    }
    
    /// Set time limit
    pub fn set_time_limit(&mut self, seconds: u64) {
        self.time_limit = seconds;
    }
    
    /// Enable keep-alive
    pub fn enable_keep_alive(&mut self) {
        self.keep_alive = true;
    }
    
    /// Disable keep-alive
    pub fn disable_keep_alive(&mut self) {
        self.keep_alive = false;
    }
    
    /// Set timeout
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
    
    /// Set server
    pub fn set_server(&mut self, server: String, port: u16) {
        self.server_name = server;
        self.port = port;
    }
    
    /// Enable SSL
    pub fn enable_ssl(&mut self) {
        self.use_ssl = true;
        self.port = 443;
    }
    
    /// Disable SSL
    pub fn disable_ssl(&mut self) {
        self.use_ssl = false;
        self.port = 80;
    }
    
    /// Set SSL verify
    pub fn set_ssl_verify(&mut self, verify: bool) {
        self.ssl_verify = verify;
    }
    
    /// Add header
    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value));
    }
    
    /// Set POST data
    pub fn set_post_data(&mut self, data: String) {
        self.post_data = Some(data);
    }
    
    /// Set POST content type
    pub fn set_post_content_type(&mut self, content_type: String) {
        self.post_content_type = content_type;
    }
}

/// Target config
#[derive(Debug, Clone)]
pub struct TargetConfig {
    /// Target URL
    pub url: String,
    /// Target host
    pub host: String,
    /// Target port
    pub port: u16,
    /// Target path
    pub path: String,
    /// Use SSL
    pub use_ssl: bool,
    /// SSL config
    pub ssl_config: Option<SslConfig>,
}

impl TargetConfig {
    /// Create new target config
    pub fn new(url: String) -> Self {
        // Parse URL (simplified)
        let (host, port, path, use_ssl) = if url.starts_with("https://") {
            let rest = &url[8..];
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            let host = parts[0].split(':').next().unwrap_or("").to_string();
            let port = parts[0].split(':').nth(1).and_then(|p| p.parse().ok()).unwrap_or(443);
            let path = if parts.len() > 1 { format!("/{}", parts[1]) } else { "/".into() };
            (host, port, path, true)
        } else {
            let rest = url.strip_prefix("http://").unwrap_or(&url);
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            let host = parts[0].split(':').next().unwrap_or("").to_string();
            let port = parts[0].split(':').nth(1).and_then(|p| p.parse().ok()).unwrap_or(80);
            let path = if parts.len() > 1 { format!("/{}", parts[1]) } else { "/".into() };
            (host, port, path, false)
        };
        
        Self {
            url,
            host,
            port,
            path,
            use_ssl,
            ssl_config: None,
        }
    }
    
    /// Set SSL config
    pub fn set_ssl_config(&mut self, ssl_config: SslConfig) {
        self.ssl_config = Some(ssl_config);
    }
    
    /// Get full URL
    pub fn get_url(&self) -> String {
        let scheme = if self.use_ssl { "https" } else { "http" };
        format!("{}://{}:{}{}", scheme, self.host, self.port, self.path)
    }
}

/// SSL config
#[derive(Debug, Clone)]
pub struct SslConfig {
    /// SSL certificate file
    pub cert_file: String,
    /// SSL key file
    pub key_file: String,
    /// SSL CA file
    pub ca_file: String,
    /// SSL ciphers
    pub ciphers: String,
    /// SSL verify
    pub verify: bool,
    /// SSL verify depth
    pub verify_depth: u32,
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            cert_file: String::new(),
            key_file: String::new(),
            ca_file: String::new(),
            ciphers: "DEFAULT".into(),
            verify: true,
            verify_depth: 9,
        }
    }
}

impl SslConfig {
    /// Create new SSL config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set certificate
    pub fn set_certificate(&mut self, cert_file: String, key_file: String) {
        self.cert_file = cert_file;
        self.key_file = key_file;
    }
    
    /// Set CA file
    pub fn set_ca_file(&mut self, ca_file: String) {
        self.ca_file = ca_file;
    }
    
    /// Set ciphers
    pub fn set_ciphers(&mut self, ciphers: String) {
        self.ciphers = ciphers;
    }
    
    /// Enable verify
    pub fn enable_verify(&mut self) {
        self.verify = true;
    }
    
    /// Disable verify
    pub fn disable_verify(&mut self) {
        self.verify = false;
    }
    
    /// Set verify depth
    pub fn set_verify_depth(&mut self, depth: u32) {
        self.verify_depth = depth;
    }
}
