// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrpcStatusCode {
    Unauthenticated,
    PermissionDenied,
    InvalidArgument,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpcRequestContext {
    pub path: String,
    pub auth_enabled: bool,
    pub authenticated_subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrpcResponseDisposition {
    Accepted,
    Rejected {
        code: GrpcStatusCode,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpcResponseContext {
    pub path: String,
    pub auth_enabled: bool,
    pub authenticated_subject: Option<String>,
    pub disposition: GrpcResponseDisposition,
}

impl GrpcResponseContext {
    pub fn accepted(
        path: impl Into<String>,
        auth_enabled: bool,
        authenticated_subject: Option<String>,
    ) -> Self {
        Self {
            path: path.into(),
            auth_enabled,
            authenticated_subject,
            disposition: GrpcResponseDisposition::Accepted,
        }
    }

    pub fn rejected(
        path: impl Into<String>,
        auth_enabled: bool,
        authenticated_subject: Option<String>,
        code: GrpcStatusCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            auth_enabled,
            authenticated_subject,
            disposition: GrpcResponseDisposition::Rejected {
                code,
                message: message.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MiddlewareError {
    #[error("{message}")]
    Rejected {
        code: GrpcStatusCode,
        message: String,
    },
}

impl MiddlewareError {
    pub fn rejected(code: GrpcStatusCode, message: impl Into<String>) -> Self {
        Self::Rejected {
            code,
            message: message.into(),
        }
    }
}

pub type SharedMiddleware = Arc<dyn Middleware>;

pub trait Middleware: Send + Sync + 'static {
    fn on_request(&self, _request: &GrpcRequestContext) -> Result<(), MiddlewareError> {
        Ok(())
    }

    fn on_response(&self, _response: &GrpcResponseContext) -> Result<(), MiddlewareError> {
        Ok(())
    }
}
