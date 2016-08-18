/* -*- mode:rust; coding:utf-8-unix; -*- */

//! event.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2016/08/18

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::any::{ Any, };
use ::std::collections::{ VecDeque, };
/* -------------------------------------------------------------------------- */
use super::event_data::{ TEventData, EventDataAelicitResult, EventDataAelicit, };
use super::event_type::{ EventTypeRef, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Event
#[derive( Debug, )]
pub struct Event {
    /// type
    type_:      EventTypeRef,
    /// data
    data:       EventDataAelicit,
}
/* ========================================================================== */
impl Event {
    /* ====================================================================== */
    /// new
    pub fn new(type_: EventTypeRef, data: EventDataAelicit) -> Self { Event {
        type_:  type_,
        data:   data,
    } }
    /* ====================================================================== */
    /// peek_type
    pub fn peek_type< 'a >(&'a self) -> &'a EventTypeRef { &self.type_ }
    /* ====================================================================== */
    /// with_data
    pub fn with_data< R, E, F >(&self, f: F)
                                -> EventDataAelicitResult< R, E, >
        where F: Fn(&Any) -> Result< R, E > {
        self.data.with(|data: &TEventData| -> Result< R, E > {
            f(data.as_ref())
        })
    }
    /* ====================================================================== */
    /// with_mut_data
        pub fn with_mut_data< R, E, F >(&self, f: F)
                                        -> EventDataAelicitResult< R, E, >
        where F: Fn(&mut Any) -> Result< R, E > {
            self.data.with_mut(|data: &mut TEventData| -> Result< R, E > {
                f(data.as_mut())
            })
        }
    /* ====================================================================== */
    /// try_with_data
        pub fn try_with_data< R, E, F >(&self, f: F)
                                        -> EventDataAelicitResult< R, E, >
        where F: Fn(&Any) -> Result< R, E > {
            self.data.try_with(|data: &TEventData| -> Result< R, E > {
                f(data.as_ref())
            })
        }
    /* ====================================================================== */
    /// try_with_mut_data
        pub fn try_with_mut_data< R, E, F >(&self, f: F)
                                            -> EventDataAelicitResult< R, E, >
        where F: Fn(&mut Any) -> Result< R, E > {
            self.data.try_with_mut(|data: &mut TEventData| -> Result< R, E > {
                f(data.as_mut())
            })
        }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventQueue
#[derive( Debug, Default, )]
pub struct EventQueue(VecDeque< Event >);
/* ========================================================================== */
impl EventQueue {
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
