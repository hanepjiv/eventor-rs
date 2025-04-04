// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/26
//  @date 2025/03/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
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
    fn from(e: elicit::Error) -> Self {
        Self::Elicit(e)
    }
}
// ============================================================================
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
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
pub type Result<T> = std::result::Result<T, Error>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::Error;
    // ========================================================================
    #[test]
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum `SyncError`
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
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
    Phantom(std::marker::PhantomData<dyn FnOnce() -> &'a Self>),
}
// ============================================================================
impl From<elicit::Error> for SyncError<'_> {
    fn from(e: elicit::Error) -> Self {
        SyncError::from(Error::from(e))
    }
}
// ----------------------------------------------------------------------------
impl From<Error> for SyncError<'_> {
    fn from(e: Error) -> Self {
        SyncError::Eventor(e)
    }
}
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
impl<'a> From<crate::EventDataBoxReadError<'a>> for SyncError<'a> {
    fn from(e: crate::EventDataBoxReadError<'a>) -> Self {
        Self::EventDataBoxRead(e)
    }
}
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
impl<'a> From<crate::EventDataBoxWriteError<'a>> for SyncError<'a> {
    fn from(e: crate::EventDataBoxWriteError<'a>) -> Self {
        Self::EventDataBoxWrite(e)
    }
}
// ============================================================================
impl std::fmt::Display for SyncError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for SyncError<'_> {
    // ========================================================================
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
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
#[allow(dead_code)]
pub type SyncResult<'a, T> = std::result::Result<T, SyncError<'a>>;
