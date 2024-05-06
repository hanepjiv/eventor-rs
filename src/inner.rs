// -*- mode:rust; coding:utf-8-unix; -*-

//! inner.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/21
//  @date 2024/05/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
mod type_map;
pub(crate) use type_map::TypeMap;
// ----------------------------------------------------------------------------
mod listener_map;
pub(crate) use listener_map::ListenerMap;
// ----------------------------------------------------------------------------
mod mediator;
pub(crate) use mediator::Mediator;
// ----------------------------------------------------------------------------
#[cfg(feature = "parking_lot")]
mod sync_parking_lot;
#[cfg(feature = "parking_lot")]
pub(crate) mod sync {
    pub(crate) use super::sync_parking_lot::*;
}

#[cfg(not(any(feature = "parking_lot"),))]
mod sync_default;
#[cfg(not(any(feature = "parking_lot"),))]
pub(crate) mod sync {
    pub(crate) use super::sync_default::*;
}
