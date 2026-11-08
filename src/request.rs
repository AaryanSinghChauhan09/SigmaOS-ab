//! HTTP request definitions for benchmarking.

/// HTTP method
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    /// GET method
    GET,
    /// POST method
    POST,
    /// HEAD method
    HEAD,
    /// PUT method
    PUT,
    /// DELETE method
    DELETE,
    /// OPTIONS method
    OPTIONS,
}

impl HttpMethod {
    /// Get method name
    pub fn name(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

/// HTTP header
#[derive(Debug, Clone)]
pub struct HttpHeader {
    /// Header name
    pub name: String,
    /// Header value
    pub value: String,
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// Status code
    pub status_code: u16,
    /// Status message
    pub status_message: String,
    /// Response headers
    pub headers: Vec<HttpHeader>,
    /// Response body
    pub body: Vec<u8>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

/// HTTP request
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// Request method
    pub method: HttpMethod,
    /// Request URL
    pub url: String,
    /// Request headers
    pub headers: Vec<HttpHeader>,
    /// Request body
    pub body: Option<Vec<u8>>,
    /// Request timeout in seconds
    pub timeout: u64,
    /// Request created timestamp
    pub created_at: u64,
}

impl HttpRequest {
    /// Create new HTTP request
    pub fn new(method: String, url: String) -> Self {
        let http_method = match method.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "HEAD" => HttpMethod::HEAD,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::GET,
        };
        
        Self {
            method: http_method,
            url,
            headers: vec![],
            body: None,
            timeout: 30,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add header
    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push(HttpHeader { name, value });
    }
    
    /// Set body
    pub fn set_body(&mut self, body: String) {
        self.body = Some(body.into_bytes());
    }
    
    /// Set timeout
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
    
    /// Get header
    pub fn get_header(&self, name: &str) -> Option<&HttpHeader> {
        self.headers.iter().find(|h| h.name == name)
    }
    
    /// Check if request has body
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    
    /// Get request age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}
