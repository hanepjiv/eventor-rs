// -*- mode:rust; coding:utf-8-unix; -*-

//! event.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2024/04/08

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{collections::VecDeque, result::Result as StdResult};
// ----------------------------------------------------------------------------
use super::{
    event_data::{EventDataTrait, EventDataWrapper},
    event_type::EventTypeRef,
    Error,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Event
#[derive(Debug)]
pub struct Event {
    /// type
    type_: EventTypeRef,
    /// data
    data: EventDataWrapper,
}
// ============================================================================
impl Event {
    // ========================================================================
    /// new
    pub fn new(type_: EventTypeRef, data: EventDataWrapper) -> Self {
        Self { type_, data }
    }
    // ========================================================================
    /// peek_type
    pub fn peek_type(&self) -> &EventTypeRef {
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
        self.data.with(|d: &dyn EventDataTrait| -> StdResult<T, E> {
            d.as_ref()
                .downcast_ref::<D>()
                .ok_or(Error::Downcast)
                .map(f)?
        })
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
        self.data
            .with_mut(|d: &mut dyn EventDataTrait| -> StdResult<T, E> {
                d.as_mut()
                    .downcast_mut::<D>()
                    .ok_or(Error::Downcast)
                    .map(f)?
            })
    }
    // ========================================================================
    /// try_with
    pub fn try_with<D, T, E>(
        &self,
        f: impl FnOnce(&D) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
        D: 'static,
        E: From<Error> + From<elicit::Error>,
    {
        self.data
            .try_with(|d: &dyn EventDataTrait| -> StdResult<T, E> {
                d.as_ref()
                    .downcast_ref::<D>()
                    .ok_or(Error::Downcast)
                    .map(f)?
            })
    }
    // ========================================================================
    /// try_with_mut
    pub fn try_with_mut<D, T, E>(
        &self,
        f: impl FnOnce(&mut D) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
        D: 'static,
        E: From<Error> + From<elicit::Error>,
    {
        self.data.try_with_mut(
            |d: &mut dyn EventDataTrait| -> StdResult<T, E> {
                d.as_mut()
                    .downcast_mut::<D>()
                    .ok_or(Error::Downcast)
                    .map(f)?
            },
        )
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventQueue
#[derive(Debug, Default)]
pub(crate) struct EventQueue(VecDeque<Event>);
// ============================================================================
impl EventQueue {
    // ========================================================================
    /// push
    pub(crate) fn push(&mut self, event: Event) {
        self.0.push_back(event)
    }
    // ========================================================================
    /// pop
    pub(crate) fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
    // ========================================================================
    /// shrink_to_fit
    pub(crate) fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
}
