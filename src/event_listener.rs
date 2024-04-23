// -*- mode:rust; coding:utf-8-unix; -*-

//! event_listener.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2024/04/23

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::fmt::Debug;
// ----------------------------------------------------------------------------
use elicit::aelicit_define;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{event::Event, eventor::Eventor};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum RetOnEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RetOnEvent {
    /// Next
    Next,
    /// Complete
    Complete,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait EventListener
#[aelicit_define(event_listener_aelicit)]
pub trait EventListener: Debug + Sync + Send {
    // ========================================================================
    /// peek_id
    fn peek_id(&self) -> &Uuid;
    // ========================================================================
    /// on_event
    fn on_event(&self, event: &Event, eventor: &Eventor) -> RetOnEvent;
}
// ============================================================================
pub use event_listener_aelicit::author as aelicit_author;
pub use event_listener_aelicit::user as aelicit_user;
