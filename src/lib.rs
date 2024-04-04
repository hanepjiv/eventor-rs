// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/04/04

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![forbid(
    clippy::all,
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2021_compatibility
)]
#![warn(unused, warnings)]
// mod  =======================================================================
mod error;
mod event;
mod event_data;
mod event_listener;
mod event_type;
mod eventor;
// use  =======================================================================
pub use self::{
    error::{Error, Result},
    event::Event,
    event_data::{
        EventData, EventDataAelicit, EventDataEAFS, EventDataEAFSField,
    },
    event_listener::{
        EventListener, EventListenerAelicit, EventListenerEAFS,
        EventListenerEAFSField, RetOnEvent,
    },
    event_type::{EventType, EventTypeRef},
    eventor::Eventor,
};
