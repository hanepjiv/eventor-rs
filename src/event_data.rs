// -*- mode:rust; coding:utf-8-unix; -*-

//! `event_data.rs`

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2025/04/07

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use alloc::sync::Arc;
use core::{any::Any, fmt::Debug};
// ----------------------------------------------------------------------------
use super::error::Error;

#[cfg(feature = "parking_lot")]
use super::inner::sync::RwLock;

#[cfg(not(any(feature = "parking_lot"),))]
use super::inner::sync::{RwLock, TryLockReadError, TryLockWriteError};
// ============================================================================
trait DataTerms: 'static + Debug + Send + Sync {}
impl<T> DataTerms for T where T: 'static + Debug + Send + Sync {}
// ============================================================================
type DataBox = Box<dyn Any + Send + Sync>;
// ============================================================================
#[cfg(not(any(feature = "parking_lot"),))]
/// `EventDataBoxReadError`
pub type EventDataBoxReadError<'a> = TryLockReadError<'a, DataBox>;
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot"),))]
/// `EventDataBoxWriteError`
pub type EventDataBoxWriteError<'a> = TryLockWriteError<'a, DataBox>;
// ============================================================================
/// `EventDataBox`
#[derive(Debug)]
pub struct EventDataBox(Arc<RwLock<DataBox>>);
// ----------------------------------------------------------------------------
#[expect(private_bounds, reason = "allow")]
impl EventDataBox {
    // ========================================================================
    /// new
    #[inline]
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
        let r = self.0.read();
        f(r.downcast_ref()
            .ok_or_else(|| Error::Downcast("EventDataBox::with".to_owned()))?)
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
        let r = self.0.read().map_err(EventDataBoxReadError::from)?;
        f(r.downcast_ref()
            .ok_or_else(|| Error::Downcast("EventDataBox::with".to_owned()))?)
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
        let mut w = self.0.write();
        f(w.downcast_mut().ok_or_else(|| {
            Error::Downcast("EventDataBox::with_mut".to_owned())
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
        let mut w = self.0.write().map_err(EventDataBoxWriteError::from)?;
        f(w.downcast_mut().ok_or_else(|| {
            Error::Downcast("EventDataBox::with_mut".to_owned())
        })?)
    }
}
