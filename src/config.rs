// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDocument {
    pub source: String,
    pub contents: Option<String>,
}

impl ConfigDocument {
    pub fn from_contents(source: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            contents: Some(contents.into()),
        }
    }

    pub fn missing(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            contents: None,
        }
    }
}

pub type SharedConfigLoader = Arc<dyn ConfigLoader>;

#[async_trait]
pub trait ConfigLoader: Send + Sync + 'static {
    async fn load_config(&self) -> Result<ConfigDocument>;
}
