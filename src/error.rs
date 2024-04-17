// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/26
//  @date 2024/04/16

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
    /// HashConflict
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
        Error::Elicit(e)
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
            Error::Elicit(ref e) => Some(e),
            Error::Eventor(_) => None,
            Error::HashConflict { .. } => None,
            Error::Downcast(_) => None,
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
    use super::{Error, Result};
    // ========================================================================
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
        assert_send::<Result<()>>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
        assert_sync::<Result<()>>();
    }
}
