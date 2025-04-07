// -*- mode:rust; coding:utf-8-unix; -*-

//! event.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2025/04/07

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use alloc::collections::VecDeque;
use core::result::Result as StdResult;
// ----------------------------------------------------------------------------
use super::{error::Error, event_data::EventDataBox, event_type::EventType};
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot"),))]
use super::event_data::{EventDataBoxReadError, EventDataBoxWriteError};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Event
#[derive(Debug)]
pub struct Event {
    /// type
    type_: EventType,
    /// event data box
    data: EventDataBox,
}
// ============================================================================
impl Event {
    // ========================================================================
    /// new
    #[must_use]
    #[inline]
    pub const fn new(type_: EventType, data: EventDataBox) -> Self {
        Self { type_, data }
    }
    // ========================================================================
    /// `peek_type`
    #[must_use]
    #[inline]
    pub const fn peek_type(&self) -> &EventType {
        &self.type_
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// with
    ///
    /// # Errors
    ///
    /// `E: From<Error>`
    #[inline]
    pub fn with<D, F, T, E>(&self, f: F) -> StdResult<T, E>
    where
        D: 'static,
        F: FnOnce(&D) -> StdResult<T, E>,
        E: From<Error>,
    {
        self.data.with(f)
    }
    // ------------------------------------------------------------------------
    #[cfg(not(any(feature = "parking_lot"),))]
    /// with
    ///
    /// # Errors
    ///
    /// `E: From<Error> + From<EventDataBoxReadError<'a>>`
    #[inline]
    pub fn with<'s, 'a, D, F, T, E>(&'s self, f: F) -> StdResult<T, E>
    where
        's: 'a,
        D: 'static,
        F: FnOnce(&D) -> StdResult<T, E>,
        E: From<Error> + From<EventDataBoxReadError<'a>>,
    {
        self.data.with(f)
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// `with_mut`
    ///
    /// # Errors
    ///
    /// `E: From<Error>`
    #[inline]
    pub fn with_mut<D, F, T, E>(&self, f: F) -> StdResult<T, E>
    where
        D: 'static,
        F: FnOnce(&mut D) -> StdResult<T, E>,
        E: From<Error>,
    {
        self.data.with_mut(f)
    }
    // ------------------------------------------------------------------------
    #[cfg(not(any(feature = "parking_lot"),))]
    /// `with_mut`
    ///
    /// # Errors
    ///
    /// `E: From<Error> + From<EventDataBoxWriteError<'a>>`
    #[inline]
    pub fn with_mut<'s, 'a, D, F, T, E>(&'s self, f: F) -> StdResult<T, E>
    where
        's: 'a,
        D: 'static,
        F: FnOnce(&mut D) -> StdResult<T, E>,
        E: From<Error> + From<EventDataBoxWriteError<'a>>,
    {
        self.data.with_mut(f)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct `EventQueue`
#[derive(Debug, Default)]
pub(crate) struct EventQueue {
    queue: VecDeque<Event>,
}
// ============================================================================
impl EventQueue {
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// push
    pub(crate) fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }
    // ========================================================================
    /// `push_front`
    pub(crate) fn push_front(&mut self, event: Event) {
        self.queue.push_front(event);
    }
    // ========================================================================
    /// pop
    pub(crate) fn pop(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
    // ========================================================================
    /// shrink
    #[expect(clippy::integer_division, reason = "checked")]
    pub(crate) fn shrink(&mut self) {
        self.queue.shrink_to((self.queue.capacity() / 2).max(64));
    }
    // ========================================================================
    /// capacity
    #[expect(dead_code, reason = "checked")]
    pub(crate) fn capacity(&self) -> usize {
        self.queue.capacity()
    }
}
