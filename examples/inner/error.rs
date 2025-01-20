// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2025/01/20

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub(crate) enum Error {
    /// Elicit
    Elicit(elicit::Error),
    /// Eventor
    Eventor(eventor::Error),
}
// ============================================================================
impl From<elicit::Error> for Error {
    fn from(e: elicit::Error) -> Self {
        Error::Elicit(e)
    }
}
// ----------------------------------------------------------------------------
impl From<eventor::Error> for Error {
    fn from(e: eventor::Error) -> Self {
        Error::Eventor(e)
    }
}
// ============================================================================
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
// ============================================================================
impl std::error::Error for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Elicit(ref e) => Some(e),
            Error::Eventor(ref e) => Some(e),
        }
    }
}
