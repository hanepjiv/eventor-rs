// -*- mode:rust; coding:utf-8-unix; -*-

//! event.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2017/02/24

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use ::std::collections::VecDeque;
// ----------------------------------------------------------------------------
use super::Error;
use super::event_data::{ TEventData, EventDataAelicit, };
use super::event_type::EventTypeRef;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Event
#[derive( Debug, )]
pub struct Event {
    /// type
    type_:      EventTypeRef,
    /// data
    data:       EventDataAelicit,
}
// ============================================================================
impl Event {
    // ========================================================================
    /// new
    pub fn new(type_: EventTypeRef, data: EventDataAelicit) -> Self { Event {
        type_:  type_,
        data:   data,
    } }
    // ========================================================================
    /// peek_type
    pub fn peek_type<'a>(&'a self) -> &'a EventTypeRef { &self.type_ }
    // ========================================================================
    /// with_data
    pub fn with_data<T, R, F>(&self, f: F) -> ::elicit::Result<R>
        where T:        'static,
              F:        Fn(&T) -> ::elicit::Result<R> {
        self.data.with(|d: &TEventData| -> ::elicit::Result<R> {
            if let Some(ref t) = d.as_ref().downcast_ref::<T>() {
                f(t)
            } else {
                Err(Box::new(Error::DowncastError))
            }
        })
    }
    // ========================================================================
    /// with_mut_data
    pub fn with_mut_data<T, R, F>(&self, f: F) -> ::elicit::Result<R>
        where T:        'static,
              F:        Fn(&mut T) -> ::elicit::Result<R> {
        self.data.with_mut(|d: &mut TEventData| -> ::elicit::Result<R> {
            if let Some(ref mut t) = d.as_mut().downcast_mut::<T>() {
                f(t)
            } else {
                Err(Box::new(Error::DowncastError))
            }
        })
    }
    // ========================================================================
    /// try_with_data
    pub fn try_with_data<T, R, F>(&self, f: F) -> ::elicit::Result<R>
        where T:        'static,
              F:        Fn(&T) -> ::elicit::Result<R> {
        self.data.try_with(|d: &TEventData| -> ::elicit::Result<R> {
            if let Some(ref t) = d.as_ref().downcast_ref::<T>() {
                f(t)
            } else {
                Err(Box::new(Error::DowncastError))
            }
        })
    }
    // ========================================================================
    /// try_with_mut_data
    pub fn try_with_mut_data<T, R, F>(&self, f: F) -> ::elicit::Result<R>
        where T:        'static,
              F:        Fn(&mut T) -> ::elicit::Result<R> {
        self.data.try_with_mut(|d: &mut TEventData| -> ::elicit::Result<R> {
            if let Some(ref mut t) = d.as_mut().downcast_mut::<T>() {
                f(t)
            } else {
                Err(Box::new(Error::DowncastError))
            }
        })
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventQueue
#[derive( Debug, Default, )]
pub struct EventQueue(VecDeque<Event>);
// ============================================================================
impl EventQueue {
    // ========================================================================
    /// push
    pub fn push(&mut self, event: Event) -> () {
        self.0.push_back(event)
    }
    // ========================================================================
    /// pop
    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
    // ========================================================================
    /// shrink_to_fit
    pub fn shrink_to_fit(&mut self) -> () {
        self.0.shrink_to_fit()
    }
}
