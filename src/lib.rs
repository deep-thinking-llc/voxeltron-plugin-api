// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
//! Stable public plugin and embedding traits for Voxeltron.
//!
//! This crate is intended to become the narrow compatibility surface for
//! external plugins and private/commercial wrappers. It contains traits and
//! shared value types only; concrete implementations stay in `voxeltron-core`
//! or downstream crates.

pub mod audit;
pub mod auth;
pub mod config;
pub mod middleware;
pub mod registry;
pub mod runtime;
