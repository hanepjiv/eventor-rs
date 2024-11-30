// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/12/01

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================

#![deny(dead_code, trivial_casts, private_bounds)]
// ////////////////////////////////////////////////////////////////////////////
// mod  =======================================================================
mod error;
mod event;
mod event_data;
mod event_listener;
mod event_type;
mod eventor;
mod inner;
// use  =======================================================================
pub use {
    error::{Error, Result, SyncError, SyncResult},
    event::Event,
    event_data::EventDataBox,
    event_listener::{
        aelicit_author as event_listener_aelicit_author,
        aelicit_user as event_listener_aelicit_user, EventListener,
        RetOnEvent,
    },
    event_type::EventType,
    eventor::Eventor,
};
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot"),))]
pub use event_data::{EventDataBoxReadError, EventDataBoxWriteError};
