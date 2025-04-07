// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/19
//  @date 2025/03/01

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
        Self::Elicit(e)
    }
}
// ----------------------------------------------------------------------------
impl From<eventor::Error> for Error {
    fn from(e: eventor::Error) -> Self {
        Self::Eventor(e)
    }
}
// ============================================================================
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
// ============================================================================
impl core::error::Error for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match *self {
            Self::Elicit(ref e) => Some(e),
            Self::Eventor(ref e) => Some(e),
        }
    }
}
