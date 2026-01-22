//! Introspection utilities for Rapina applications.
//!
//! This module provides tools for inspecting route metadata,
//! enabling documentation generation and AI-native tooling.

mod endpoint;
mod route_info;

pub use endpoint::{RouteRegistry, list_routes};
pub use route_info::RouteInfo;
