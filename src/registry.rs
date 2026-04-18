// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryPullRequest {
    pub image: String,
    pub tag: String,
}

impl RegistryPullRequest {
    pub fn new(image: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            tag: tag.into(),
        }
    }

    pub fn image_ref(&self) -> String {
        format!("{}:{}", self.image, self.tag)
    }

    pub fn registry_host(&self) -> Option<&str> {
        let first_segment = self.image.split('/').next()?;
        if first_segment.contains('.')
            || first_segment.contains(':')
            || first_segment == "localhost"
        {
            Some(first_segment)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistryPolicyError {
    #[error("registry pull denied for {image}: {reason}")]
    Denied { image: String, reason: String },

    #[error("registry policy failed: {0}")]
    Internal(String),
}

pub type SharedRegistryPolicy = Arc<dyn RegistryPolicy>;

#[async_trait]
pub trait RegistryPolicy: Send + Sync + 'static {
    async fn authorize_pull(
        &self,
        request: &RegistryPullRequest,
    ) -> Result<(), RegistryPolicyError>;
}
