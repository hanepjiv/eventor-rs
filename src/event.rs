/* -*- mode:rust; coding:utf-8-unix; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/07
// @date 2016/05/06

// The MIT License (MIT)
//
// Copyright (c) <2016> hanepjiv <hanepjiv@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! event.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::any::{ Any, };
use ::std::collections::{ VecDeque, };
/* -------------------------------------------------------------------------- */
use super::event_data::{ TEventData, EventDataElicitResult, EventDataElicit, };
use super::event_type::{ EventTypeRef, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Event
#[derive( Debug, )]
pub struct Event {
    /// type
    type_:      EventTypeRef,
    /// data
    data:       EventDataElicit,
}
/* ========================================================================== */
impl Event {
    /* ====================================================================== */
    /// new
    pub fn new(type_: EventTypeRef, data: EventDataElicit) -> Self { Event {
        type_:  type_,
        data:   data,
    } }
    /* ====================================================================== */
    /// peek_type
    pub fn peek_type< 'a >(&'a self) -> &'a EventTypeRef { &self.type_ }
    /* ====================================================================== */
    /// with_data
    pub fn with_data< R, E, F >(&self, f: F)
                                -> EventDataElicitResult< R, E, >
        where F: Fn(&Any) -> Result< R, E > {
        self.data.with(|data: &TEventData| -> Result< R, E > {
            f(data.as_ref())
        })
    }
    /* ====================================================================== */
    /// with_mut_data
        pub fn with_mut_data< R, E, F >(&self, f: F)
                                        -> EventDataElicitResult< R, E, >
        where F: Fn(&mut Any) -> Result< R, E > {
            self.data.with_mut(|data: &mut TEventData| -> Result< R, E > {
                f(data.as_mut())
            })
        }
    /* ====================================================================== */
    /// try_with_data
        pub fn try_with_data< R, E, F >(&self, f: F)
                                        -> EventDataElicitResult< R, E, >
        where F: Fn(&Any) -> Result< R, E > {
            self.data.try_with(|data: &TEventData| -> Result< R, E > {
                f(data.as_ref())
            })
        }
    /* ====================================================================== */
    /// try_with_mut_data
        pub fn try_with_mut_data< R, E, F >(&self, f: F)
                                            -> EventDataElicitResult< R, E, >
        where F: Fn(&mut Any) -> Result< R, E > {
            self.data.try_with_mut(|data: &mut TEventData| -> Result< R, E > {
                f(data.as_mut())
            })
        }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventQueue
#[derive( Debug, )]
pub struct EventQueue(VecDeque< Event >);
/* ========================================================================== */
impl EventQueue {
    /* ====================================================================== */
    /// new
    pub fn new() -> Self { EventQueue(VecDeque::new()) }
    /* ====================================================================== */
    /// push
    pub fn push(&mut self, event: Event) -> () {
        let &mut EventQueue(ref mut deq) = self;
        deq.push_back(event)
    }
    /* ====================================================================== */
    /// pop
    pub fn pop(&mut self) -> Option< Event > {
        let &mut EventQueue(ref mut deq) = self;
        deq.pop_front()
    }
    /* ====================================================================== */
    /// shrink_to_fit
    pub fn shrink_to_fit(&mut self) -> () {
        let &mut EventQueue(ref mut deq) = self;
        deq.shrink_to_fit()
    }
}
