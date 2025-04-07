// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/26
//  @date 2025/04/07

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use core::error::Error as CoreError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Elicit
    Elicit(elicit::Error),

    /// Eventor
    Eventor(String),

    /// Downcast
    Downcast(String),

    /// `HashConflict`
    HashConflict {
        /// already
        already: String,
        /// new
        new: String,
    },
}
// ============================================================================
impl From<elicit::Error> for Error {
    #[inline]
    fn from(e: elicit::Error) -> Self {
        Self::Elicit(e)
    }
}
// ============================================================================
impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl CoreError for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn CoreError + 'static)> {
        match *self {
            Self::Elicit(ref e) => Some(e),
            Self::Eventor(_)
            | Self::HashConflict { .. }
            | Self::Downcast(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = core::result::Result<T, Error>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::Error;
    // ========================================================================
    #[test]
    #[inline]
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    #[inline]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum `SyncError`
#[derive(Debug)]
#[non_exhaustive]
pub enum SyncError<'a> {
    /// Eventor
    Eventor(Error),

    #[cfg(not(any(feature = "parking_lot",)))]
    /// `EventDataBoxRead`
    EventDataBoxRead(crate::EventDataBoxReadError<'a>),

    #[cfg(not(any(feature = "parking_lot",)))]
    /// `EventDataBoxWrite`
    EventDataBoxWrite(crate::EventDataBoxWriteError<'a>),

    #[cfg(feature = "parking_lot")]
    /// Phantom
    Phantom(core::marker::PhantomData<dyn FnOnce() -> &'a Self>),
}
// ============================================================================
impl From<elicit::Error> for SyncError<'_> {
    #[inline]
    fn from(e: elicit::Error) -> Self {
        SyncError::from(Error::from(e))
    }
}
// ----------------------------------------------------------------------------
impl From<Error> for SyncError<'_> {
    #[inline]
    fn from(e: Error) -> Self {
        SyncError::Eventor(e)
    }
}
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
impl<'a> From<crate::EventDataBoxReadError<'a>> for SyncError<'a> {
    #[inline]
    fn from(e: crate::EventDataBoxReadError<'a>) -> Self {
        Self::EventDataBoxRead(e)
    }
}
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
impl<'a> From<crate::EventDataBoxWriteError<'a>> for SyncError<'a> {
    #[inline]
    fn from(e: crate::EventDataBoxWriteError<'a>) -> Self {
        Self::EventDataBoxWrite(e)
    }
}
// ============================================================================
impl core::fmt::Display for SyncError<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl CoreError for SyncError<'_> {
    #[inline]
    fn source(&self) -> Option<&(dyn CoreError + 'static)> {
        match *self {
            SyncError::Eventor(ref e) => Some(e),

            #[cfg(not(any(feature = "parking_lot",)))]
            SyncError::EventDataBoxRead(_) => None,

            #[cfg(not(any(feature = "parking_lot",)))]
            SyncError::EventDataBoxWrite(_) => None,

            #[cfg(feature = "parking_lot")]
            SyncError::Phantom(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type `SyncResult`
pub type SyncResult<'a, T> = core::result::Result<T, SyncError<'a>>;
