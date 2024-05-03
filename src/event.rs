// -*- mode:rust; coding:utf-8-unix; -*-

//! event.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/05/03

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{collections::VecDeque, result::Result as StdResult};
// ----------------------------------------------------------------------------
use super::{error::Error, event_data::EventDataBox, event_type::EventType};
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
    pub fn new(type_: EventType, data: EventDataBox) -> Self {
        Self { type_, data }
    }
    // ========================================================================
    /// peek_type
    pub fn peek_type(&self) -> &EventType {
        &self.type_
    }
    // ========================================================================
    /// with
    pub fn with<D, T, E>(
        &self,
        f: impl FnOnce(&D) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
        D: 'static,
        E: From<Error> + From<elicit::Error>,
    {
        self.data.with(f)
    }
    // ========================================================================
    /// with_mut
    pub fn with_mut<D, T, E>(
        &self,
        f: impl FnOnce(&mut D) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
        D: 'static,
        E: From<Error> + From<elicit::Error>,
    {
        self.data.with_mut(f)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventQueue
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
        self.queue.push_back(event)
    }
    // ========================================================================
    /// push_front
    pub(crate) fn push_front(&mut self, event: Event) {
        self.queue.push_front(event)
    }
    // ========================================================================
    /// pop
    pub(crate) fn pop(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
    // ========================================================================
    /// shrink
    pub(crate) fn shrink(&mut self) {
        self.queue.shrink_to((self.queue.capacity() / 2).max(64))
    }
    // ========================================================================
    /// capacity
    #[allow(dead_code)]
    pub(crate) fn capacity(&self) -> usize {
        self.queue.capacity()
    }
}
