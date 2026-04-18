// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct AuditEvent {
    pub actor: String,
    pub action: String,
    pub target: String,
    pub ip_addr: String,
    pub result: String,
    pub detail: Value,
}

impl AuditEvent {
    pub fn new(
        actor: impl Into<String>,
        action: impl Into<String>,
        target: impl Into<String>,
        ip_addr: impl Into<String>,
        result: impl Into<String>,
        detail: Value,
    ) -> Self {
        Self {
            actor: actor.into(),
            action: action.into(),
            target: target.into(),
            ip_addr: ip_addr.into(),
            result: result.into(),
            detail,
        }
    }
}

pub type SharedAuditSink = Arc<dyn AuditSink>;

#[async_trait]
pub trait AuditSink: Send + Sync + 'static {
    async fn record_event(&self, event: AuditEvent) -> Result<()>;
}
