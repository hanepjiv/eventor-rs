// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/05/03
//  @date 2024/05/03

// ////////////////////////////////////////////////////////////////////////////
// mod  =======================================================================
mod error;
// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub(crate) use error::Error;
// ----------------------------------------------------------------------------
/// type Result
pub(crate) type Result<T> = std::result::Result<T, Error>;
