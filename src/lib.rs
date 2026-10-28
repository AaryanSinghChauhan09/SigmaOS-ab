// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ab - Security/Server component
//! Part of SigmaOS multi-repository architecture

#![no_std]

/// Component
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self { name: "SigmaOS-ab" }
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation() {
        let c = super::Component::new();
        assert_eq!(c.name, "SigmaOS-ab");
    }
}
