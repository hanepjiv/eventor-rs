/* -*- mode:rust; coding:utf-8; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/03
// @date 2016/04/29

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
// IMPLIED, INCLUDING BUTNOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! eventor.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::sync::{ RwLock, };
/* ========================================================================== */
use self::super::event::{ Event, EventQueue, };
use super::event_type::{ EventTypeRef, EventTypeMap, };
use super::event_listener::{ EventListenerElicit,
                             EventListenerMap,
                             EventListenerWaiting, EventListenerRemoving, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
elicit_define!(elicit_t_eventor, TEventor);
/* -------------------------------------------------------------------------- */
pub use self::elicit_t_eventor::ElicitError     as EventorElicitError;
pub use self::elicit_t_eventor::ElicitResult    as EventorElicitResult;
pub use self::elicit_t_eventor::Elicit          as EventorElicit;
pub use self::elicit_t_eventor::EnableElicitFromSelf
    as EventorEnableElicitFromSelf;
pub use self::elicit_t_eventor::EnableElicitFromSelfField
    as EventorEnableElicitFromSelfField;
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// TEventor
pub trait TEventor: ::std::fmt::Debug + EventorEnableElicitFromSelf {
    /* ====================================================================== */
    /// new_type
    fn new_type(&self, &str) -> Option< EventTypeRef >;
    /* ---------------------------------------------------------------------- */
    /// peek_type
    fn peek_type(&self, &str) -> Option< EventTypeRef >;
    /* ////////////////////////////////////////////////////////////////////// */
    /* ---------------------------------------------------------------------- */
    /// insert_listener
    fn insert_listener(&self,
                       event_hash: u32, listener: &EventListenerElicit) -> ();
    /* ---------------------------------------------------------------------- */
    /// remove_listener
    fn remove_listener(&self,
                       event_hash: u32, id: ::libc::uintptr_t) -> ();
    /* ====================================================================== */
    /// push_event
    fn push_event(&self, Event) -> ();
    /* ---------------------------------------------------------------------- */
    /// dispatch
    fn dispatch(&self) -> bool;
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Eventor
#[derive( Debug, )]
pub struct Eventor {
    /* base  ================================================================ */
    _eefsf:             EventorEnableElicitFromSelfField,
    /* field  =============================================================== */
    /// event type map
    type_map:           RwLock< EventTypeMap >,
    /// event queue
    queue:              RwLock< EventQueue >,
    /// event listener map
    listener_map:       RwLock< EventListenerMap >,
    /// event listener waiting
    listener_waiting:   EventListenerWaiting,
    /// event listener removing
    listener_removing:  EventListenerRemoving,
}
/* ========================================================================== */
impl Eventor {
    /* ====================================================================== */
    /// new
    pub fn new() -> EventorElicit {
        EventorElicit::new(Eventor {
            _eefsf:             EventorEnableElicitFromSelfField::new(),
            type_map:           RwLock::new(EventTypeMap::new()),
            queue:              RwLock::new(EventQueue::new()),
            listener_map:       RwLock::new(EventListenerMap::new()),
            listener_waiting:   EventListenerWaiting::new(),
            listener_removing:  EventListenerRemoving::new(),
        })
    }
}
/* ========================================================================== */
impl EventorEnableElicitFromSelf for Eventor {
    enable_elicit_from_self_impl_inner!(TEventor, EventorElicit, _eefsf);
}
/* ========================================================================== */
impl TEventor for Eventor {
    /* ====================================================================== */
    /* ---------------------------------------------------------------------- */
    fn new_type(&self, name: &str) -> Option< EventTypeRef > {
        self.type_map.write().expect("Eventor::new_type").new_type(name)
    }
    /* ---------------------------------------------------------------------- */
    fn peek_type(&self, name: &str) -> Option< EventTypeRef > {
        self.type_map.read().expect("Eventor::peek_type").peek_type(name)
    }
    /* ====================================================================== */
    /* ---------------------------------------------------------------------- */
    fn insert_listener(&self,
                       event_hash: u32, listener: &EventListenerElicit) -> () {
        self.listener_waiting.insert(event_hash, listener.clone())
    }
    /* ---------------------------------------------------------------------- */
    fn remove_listener(&self,
                       event_hash: u32, id: ::libc::uintptr_t) -> () {
        self.listener_removing.insert(event_hash, id)
    }
    /* ====================================================================== */
    /* ---------------------------------------------------------------------- */
    fn push_event(&self, event: Event) -> () {
        self.queue.write().expect("Eventor::push_event").push(event)
    }
    /* ---------------------------------------------------------------------- */
    fn dispatch(&self) -> bool {
        self.listener_waiting.
            apply(&mut self.listener_map.write().expect("Eventor::dispatch"));

        self.listener_removing.
            apply(&mut self.listener_map.write().expect("Eventor::dispatch"));

        let event = self.queue.write().expect("Eventor::dispatch").pop();
        match event {
            None        => {
                self.queue.write().expect("Eventor::dispatch").shrink_to_fit();
                self.listener_waiting.shrink_to_fit();
                self.listener_removing.shrink_to_fit();
                false
            },
            Some(e)     => {
                match self.listener_map.write().expect("Eventor::dispatch").
                    get_mut(&(e.peek_type().peek_hash())) {
                        None        => debug!("Eventor::dispatch: no listener"),
                        Some(list)  => {
                            for (_, ref mut listener) in list.iter_mut() {
                                listener.write().expect("Eventor::dispatch").
                                    on_event(&e,
                                             &self.elicit_from_self()
                                             .expect("Eventor::dispatch"));
                            }
                        },
                    }
                true
            },
        }
    }

}
