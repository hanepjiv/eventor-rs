// -*- mode:rust; coding:utf-8-unix; -*-

//! `event_type.rs`

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2025/03/01

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::sync::Arc;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[derive(Debug, Clone)]
/// `EventType`
pub struct EventType(Arc<(String, u32)>);
// ============================================================================
impl EventType {
    // ========================================================================
    /// new
    pub(crate) fn new<T>(name: T, hash: u32) -> Self
    where
        T: Into<String>,
    {
        Self(Arc::new((name.into(), hash)))
    }
    // ========================================================================
    /// `peek_name`
    #[must_use]
    pub fn peek_name(&self) -> &str {
        self.0.0.as_ref()
    }
    // ========================================================================
    /// `peek_hash`
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn peek_hash(&self) -> u32 {
        self.0.1
    }
}
