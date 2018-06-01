// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/26
//  @date 2018/06/01

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(missing_copy_implementations, variant_size_differences)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// Elicit
    Elicit(::elicit::Error),
    /// EventorError
    Eventor(String),
    /// Downcast
    Downcast,
}
// ============================================================================
impl From<::elicit::Error> for Error {
    fn from(e: ::elicit::Error) -> Self {
        Error::Elicit(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ Error::Elicit(_)
            | ref e @ Error::Eventor(_)
            | ref e @ Error::Downcast => write!(f, "{:?}", e),
        }
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Elicit(ref e) => e.description(),
            Error::Eventor(ref m) => m.as_str(),
            Error::Downcast => "::eventor::error::Error::Downcast",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Elicit(ref e) => Some(e),
            Error::Eventor(_) => None,
            Error::Downcast => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
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
