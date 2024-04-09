// -*- mode:rust; coding:utf-8-unix; -*-

//! event_data.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/04/09

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, RwLock},
};
// ----------------------------------------------------------------------------
use super::Error;
// ============================================================================
/// EventDataBox
#[allow(box_pointers)]
#[derive(Debug)]
pub struct EventDataBox {
    inner: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
}
// ============================================================================
impl EventDataBox {
    // ========================================================================
    /// new
    #[allow(box_pointers)]
    pub fn new<D>(data: D) -> Self
    where
        D: Any + Send + Sync + Debug,
    {
        Self {
            inner: Arc::new(RwLock::new(Box::new(data))),
        }
    }
    // ========================================================================
    #[allow(box_pointers)]
    /// with
    pub(crate) fn with<D, T, E0, E1>(
        &self,
        f: impl FnOnce(&D) -> Result<T, E0>,
    ) -> Result<T, E1>
    where
        D: 'static,
        E1: From<E0> + From<Error>,
    {
        if let Ok(x) = self.inner.read() {
            Ok(f(x.downcast_ref().ok_or(Error::Downcast(
                "EventDataBox::with".to_string(),
            ))?)?)
        } else {
            Err(Error::Eventor("EventDataBox::with".to_string()).into())
        }
    }
    // ========================================================================
    #[allow(box_pointers)]
    /// with_mut
    pub(crate) fn with_mut<D, T, E0, E1>(
        &self,
        f: impl FnOnce(&mut D) -> Result<T, E0>,
    ) -> Result<T, E1>
    where
        D: 'static,
        E1: From<E0> + From<Error>,
    {
        if let Ok(mut x) = self.inner.write() {
            Ok(f(x.downcast_mut().ok_or(Error::Downcast(
                "EventDataBox::with_mut".to_string(),
            ))?)?)
        } else {
            Err(Error::Eventor("EventDataBox::with_mut".to_string()).into())
        }
    }
}
