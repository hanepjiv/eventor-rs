// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/26
//  @date 2016/12/30

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone)]
pub enum Error {
    /// EventorError
    EventorError(String),
    /// DowncastError
    DowncastError,
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ Error::EventorError(_) | ref e @ Error::DowncastError => {
                write!(f, "{:?}", e)
            }
        }
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::EventorError(ref m) => m.as_str(),
            Error::DowncastError => "::eventor::error::Error::DowncastError",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::EventorError(_) => None,
            Error::DowncastError => None,
        }
    }
}
