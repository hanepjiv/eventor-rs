// -*- mode:rust; coding:utf-8-unix; -*-

//! event.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2020/04/14

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::collections::VecDeque;
// ----------------------------------------------------------------------------
use super::{
    event_data::{EventDataAelicit, TEventData},
    event_type::EventTypeRef,
    {Error, Result},
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Event
#[derive(Debug)]
pub struct Event {
    /// type
    type_: EventTypeRef,
    /// data
    data: EventDataAelicit,
}
// ============================================================================
impl Event {
    // ========================================================================
    /// new
    pub fn new(type_: EventTypeRef, data: EventDataAelicit) -> Self {
        Event { type_, data }
    }
    // ========================================================================
    /// peek_type
    pub fn peek_type(&self) -> &EventTypeRef {
        &self.type_
    }
    // ========================================================================
    /// with_data
    pub fn with_data<T: 'static, R>(
        &self,
        f: impl FnOnce(&T) -> Result<R>,
    ) -> Result<R> {
        self.data.with(|d: &dyn TEventData| -> Result<R> {
            if let Some(ref t) = d.as_ref().downcast_ref::<T>() {
                f(t)
            } else {
                Err(Error::Downcast)
            }
        })
    }
    // ========================================================================
    /// with_mut_data
    pub fn with_mut_data<T: 'static, R>(
        &self,
        f: impl FnOnce(&mut T) -> Result<R>,
    ) -> Result<R> {
        self.data.with_mut(|d: &mut dyn TEventData| -> Result<R> {
            if let Some(ref mut t) = d.as_mut().downcast_mut::<T>() {
                f(t)
            } else {
                Err(Error::Downcast)
            }
        })
    }
    // ========================================================================
    /// try_with_data
    pub fn try_with_data<T: 'static, R>(
        &self,
        f: impl FnOnce(&T) -> Result<R>,
    ) -> Result<R> {
        self.data.try_with(|d: &dyn TEventData| -> Result<R> {
            if let Some(ref t) = d.as_ref().downcast_ref::<T>() {
                f(t)
            } else {
                Err(Error::Downcast)
            }
        })
    }
    // ========================================================================
    /// try_with_mut_data
    pub fn try_with_mut_data<T: 'static, R>(
        &self,
        f: impl FnOnce(&mut T) -> Result<R>,
    ) -> Result<R> {
        self.data
            .try_with_mut(|d: &mut dyn TEventData| -> Result<R> {
                if let Some(ref mut t) = d.as_mut().downcast_mut::<T>() {
                    f(t)
                } else {
                    Err(Error::Downcast)
                }
            })
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
    pub(crate) fn push(&mut self, event: Event) -> () {
        self.0.push_back(event)
    }
    // ========================================================================
    /// pop
    pub(crate) fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
    // ========================================================================
    /// shrink_to_fit
    pub(crate) fn shrink_to_fit(&mut self) -> () {
        self.0.shrink_to_fit()
    }
}
