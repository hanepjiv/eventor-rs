// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/26
//  @date 2016/11/26

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive( Debug, Clone, )]
pub enum Error {
    /// EventorError
    EventorError(String),
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    fn description(&self) -> &str { match self {
        &Error::EventorError(ref m)            => m.as_str(),
    } }
}
