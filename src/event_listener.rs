/* -*- mode:rust; coding:utf-8-unix; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/12
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

//! event_listener.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::fmt::{ Debug, };
use ::std::collections::{ BTreeMap, };
use ::std::sync::{ RwLock, };
/* -------------------------------------------------------------------------- */
use ::libc::uintptr_t;
/* -------------------------------------------------------------------------- */
use super::event::{ Event, };
use super::eventor::{ EventorElicit, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
elicit_define!(elicit_t_event_listener, TEventListener);
/* -------------------------------------------------------------------------- */
pub use self::elicit_t_event_listener::ElicitError
    as EventListenerElicitError;
pub use self::elicit_t_event_listener::ElicitResult
    as EventListenerElicitResult;
pub use self::elicit_t_event_listener::Elicit
    as EventListenerElicit;
pub use self::elicit_t_event_listener::EnableElicitFromSelf
    as EventListenerEnableElicitFromSelf;
pub use self::elicit_t_event_listener::EnableElicitFromSelfField
    as EventListenerEnableElicitFromSelfField;
/* ========================================================================== */
/// trait TEventListener
pub trait TEventListener: Debug + EventListenerEnableElicitFromSelf {
    /* ====================================================================== */
    /// on_event
    fn on_event(&mut self, event: &Event, eventor: &EventorElicit) -> bool;
    /* ====================================================================== */
    /// peek_id
    fn peek_id(&self) -> uintptr_t;
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// type EventListenerList
pub type EventListenerList = BTreeMap< uintptr_t, EventListenerElicit >;
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventListenerMap
#[derive( Debug, )]
pub struct EventListenerMap(BTreeMap< u32, EventListenerList >);
/* ========================================================================== */
impl EventListenerMap {
    /* ====================================================================== */
    /// new
    pub fn new() -> Self { EventListenerMap(BTreeMap::new()) }
    /* ====================================================================== */
    /// insert
    pub fn insert(&mut self,
                  event_hash: u32,
                  id: uintptr_t, listener: EventListenerElicit)
                  -> Option< EventListenerElicit > {
        let &mut EventListenerMap(ref mut inner) = self;
        if inner.contains_key(&event_hash) {
            inner.get_mut(&event_hash).expect("EventListenerMap::insert").
                insert(id, listener)
        } else {
            match inner.insert(event_hash, EventListenerList::new()) {
                None    => inner.get_mut(&event_hash).
                    expect("EventListenerMap::insert").
                    insert(id, listener),
                Some(_) => panic!("EventListenerMap::insert"),
            }
        }
    }
    /* ====================================================================== */
    /// remove
    pub fn remove(&mut self, event_hash: u32, id: uintptr_t)
                  -> Option< EventListenerElicit > {
        let &mut EventListenerMap(ref mut inner) = self;
        if inner.contains_key(&event_hash) {
            inner.get_mut(&event_hash).expect("EventListenerMap::remove").
                remove(&id)
        } else {
            None
        }
    }
    /* ====================================================================== */
    /// get_mut
    pub fn get_mut< Q: ?Sized >(&mut self, key: &Q)
                                -> Option< &mut EventListenerList >
        where Q: Ord,
              u32: ::std::borrow::Borrow< Q > {
        let &mut EventListenerMap(ref mut inner) = self;
        inner.get_mut(key)
    }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventListenerWaiting
#[derive( Debug, )]
pub struct EventListenerWaiting(RwLock< Vec< (u32, EventListenerElicit) > >);
/* ========================================================================== */
impl EventListenerWaiting {
    /* ====================================================================== */
    /// new
    pub fn new() -> Self { EventListenerWaiting(RwLock::new(Vec::new())) }
    /* ====================================================================== */
    /// insert
    pub fn insert(&self, event_hash: u32, listener: EventListenerElicit) {
        let &EventListenerWaiting(ref inner) = self;
        inner.write().expect("EventLitenerWaiting.insert").
            push((event_hash, listener))
    }
    /* ====================================================================== */
    /// shrink_to_fit
    pub fn shrink_to_fit(&self) -> () {
        let &EventListenerWaiting(ref inner) = self;
        inner.write().expect("EventLitenerWaiting.insert").
            shrink_to_fit()
    }
    /* ====================================================================== */
    /// apply
    pub fn apply< Q >(&self, map: &mut Q)
        where Q: ::std::ops::DerefMut< Target = EventListenerMap > {
        let &EventListenerWaiting(ref inner) = self;
        let mut vec = inner.write().expect("EventLitenerWaiting.insert");
        for &(hash, ref listener) in vec.iter() {
            let id = listener.read().expect("EventListenerWaiting::apply").
                peek_id();
            map.insert(hash, id, listener.clone());
        }
        vec.clear();
    }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventListenerRemoving
#[derive( Debug, )]
pub struct EventListenerRemoving(RwLock< Vec< (u32, uintptr_t) > >);
/* ========================================================================== */
impl EventListenerRemoving {
    /* ====================================================================== */
    /// new
    pub fn new() -> Self { EventListenerRemoving(RwLock::new(Vec::new())) }
/* ====================================================================== */
/// insert
pub fn insert(&self, event_hash: u32, id: uintptr_t) -> () {
let &EventListenerRemoving(ref inner) = self;
inner.write().expect("EventLitenerRemoving.insert").
    push((event_hash, id))
    }
    /* ====================================================================== */
    /// contains
    #[allow(dead_code)]
    pub fn contains(&self, x: &(u32, uintptr_t)) -> bool {
        let &EventListenerRemoving(ref inner) = self;
        inner.read().expect("EventLitenerRemoving.contains").
            contains(x)
    }
    /* ====================================================================== */
    /// shrink_to_fit
    pub fn shrink_to_fit(&self) -> () {
        let &EventListenerRemoving(ref inner) = self;
        inner.write().expect("EventLitenerRemoving.shrink_to_fit").
            shrink_to_fit()
    }
    /* ====================================================================== */
    /// apply
    pub fn apply< Q >(&self, map: &mut Q)
        where Q: ::std::ops::DerefMut< Target = EventListenerMap > {
        let &EventListenerRemoving(ref inner) = self;
        let mut vec = inner.write().expect("EventLitenerRemoving.apply");
        for &(hash, id) in vec.iter() {
            map.remove(hash, id);
        }
        vec.clear();
    }
}
