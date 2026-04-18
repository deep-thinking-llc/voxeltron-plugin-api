// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
use anyhow::Result;
use async_trait::async_trait;
use jsonwebtoken::{Algorithm, DecodingKey};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub subject: String,
}

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn authenticate(&self, subject: &str, expires_in_secs: u64) -> Result<String>;

    async fn validate_token(&self, token: &str) -> Result<AuthenticatedUser>;

    fn cached_key(&self) -> Option<(DecodingKey, Algorithm)> {
        None
    }

    async fn fetch_initial_jwks(&self) -> Result<()> {
        Ok(())
    }
}
