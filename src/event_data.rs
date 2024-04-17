// -*- mode:rust; coding:utf-8-unix; -*-

//! event_data.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/04/16

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
pub struct EventDataBox(Arc<RwLock<Box<dyn Any + Send + Sync>>>);
// ============================================================================
impl EventDataBox {
    // ========================================================================
    /// new
    #[allow(box_pointers)]
    pub fn new<D>(data: D) -> Self
    where
        D: Any + Send + Sync + Debug,
    {
        Self(Arc::new(RwLock::new(Box::new(data))))
    }
    // ========================================================================
    #[allow(box_pointers)]
    /// with
    pub(crate) fn with<D, T, E>(
        &self,
        f: impl FnOnce(&D) -> Result<T, E>,
    ) -> Result<T, E>
    where
        D: 'static,
        E: From<Error>,
    {
        f(self
            .0
            .read()
            .map_err(|_| Error::Eventor("EventDataBox::with".to_string()))?
            .downcast_ref()
            .ok_or_else(|| {
                Error::Downcast("EventDataBox::with".to_string())
            })?)
    }
    // ========================================================================
    #[allow(box_pointers)]
    /// with_mut
    pub(crate) fn with_mut<D, T, E>(
        &self,
        f: impl FnOnce(&mut D) -> Result<T, E>,
    ) -> Result<T, E>
    where
        D: 'static,
        E: From<Error>,
    {
        f(self
            .0
            .write()
            .map_err(|_| Error::Eventor("EventDataBox::with_mut".to_string()))?
            .downcast_mut()
            .ok_or_else(|| {
                Error::Downcast("EventDataBox::with_mut".to_string())
            })?)
    }
}
