// -*- mode:rust; coding:utf-8-unix; -*-

//! `event_data.rs`

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/11/30

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{any::Any, fmt::Debug, sync::Arc};
// ----------------------------------------------------------------------------
use super::error::Error;
#[allow(clippy::wildcard_imports)]
use super::inner::sync::*;
// ============================================================================
trait DataTerms: 'static + Debug + Send + Sync {}
impl<T> DataTerms for T where T: 'static + Debug + Send + Sync {}
// ============================================================================
type DataBox = Box<dyn Any + Send + Sync>;
// ============================================================================
#[cfg(not(any(feature = "parking_lot"),))]
#[allow(clippy::module_name_repetitions)]
/// `EventDataBoxReadError`
pub type EventDataBoxReadError<'a> = TryLockReadError<'a, DataBox>;
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot"),))]
#[allow(clippy::module_name_repetitions)]
/// `EventDataBoxWriteError`
pub type EventDataBoxWriteError<'a> = TryLockWriteError<'a, DataBox>;
// ============================================================================
#[allow(clippy::module_name_repetitions)]
/// `EventDataBox`
#[derive(Debug)]
pub struct EventDataBox(Arc<RwLock<DataBox>>);
// ----------------------------------------------------------------------------
#[allow(private_bounds)]
impl EventDataBox {
    // ========================================================================
    /// new
    pub fn new<D>(data: D) -> Self
    where
        D: DataTerms,
    {
        Self(Arc::new(RwLock::new(Box::new(data))))
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// with
    pub(crate) fn with<D, F, T, E>(&self, f: F) -> Result<T, E>
    where
        D: 'static,
        F: FnOnce(&D) -> Result<T, E>,
        E: From<Error>,
    {
        f(self.0.read().downcast_ref().ok_or_else(|| {
            Error::Downcast("EventDataBox::with".to_string())
        })?)
    }
    // ------------------------------------------------------------------------
    #[cfg(not(any(feature = "parking_lot"),))]
    /// with
    pub(crate) fn with<'s, 'a, D, F, T, E>(&'s self, f: F) -> Result<T, E>
    where
        's: 'a,
        D: 'static,
        F: FnOnce(&D) -> Result<T, E>,
        E: From<Error> + From<EventDataBoxReadError<'a>>,
    {
        f(self
            .0
            .read()
            .map_err(EventDataBoxReadError::from)?
            .downcast_ref()
            .ok_or_else(|| {
                Error::Downcast("EventDataBox::with".to_string())
            })?)
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// `with_mut`
    pub(crate) fn with_mut<D, F, T, E>(&self, f: F) -> Result<T, E>
    where
        D: 'static,
        F: FnOnce(&mut D) -> Result<T, E>,
        E: From<Error>,
    {
        f(self.0.write().downcast_mut().ok_or_else(|| {
            Error::Downcast("EventDataBox::with_mut".to_string())
        })?)
    }
    // ------------------------------------------------------------------------
    #[cfg(not(any(feature = "parking_lot"),))]
    /// `with_mut`
    pub(crate) fn with_mut<'s, 'a, D, F, T, E>(&'s self, f: F) -> Result<T, E>
    where
        's: 'a,
        D: 'static,
        F: FnOnce(&mut D) -> Result<T, E>,
        E: From<Error> + From<EventDataBoxWriteError<'a>>,
    {
        f(self
            .0
            .write()
            .map_err(EventDataBoxWriteError::from)?
            .downcast_mut()
            .ok_or_else(|| {
                Error::Downcast("EventDataBox::with_mut".to_string())
            })?)
    }
}
