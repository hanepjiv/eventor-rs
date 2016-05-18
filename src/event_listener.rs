/* -*- mode:rust; coding:utf-8-unix; -*- */

//! event_listener.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2016/05/16

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
    /// peek_id
    fn peek_id(&self) -> uintptr_t;
    /* ====================================================================== */
    /// on_event
    fn on_event(&mut self, event: &Event, eventor: &EventorElicit) -> bool;
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// type EventListenerList
pub type EventListenerList = BTreeMap< uintptr_t, EventListenerElicit >;
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct EventListenerMap
#[derive( Debug, Default, )]
pub struct EventListenerMap(BTreeMap< u32, EventListenerList >);
/* ========================================================================== */
impl EventListenerMap {
    /* ====================================================================== */
    /// insert
    pub fn insert(&mut self,
                  event_hash: u32, id: uintptr_t, listener: EventListenerElicit)
                  -> Option< EventListenerElicit > {
        let &mut EventListenerMap(ref mut inner) = self;
        if inner.contains_key(&event_hash) {
            inner.get_mut(&event_hash).expect("EventListenerMap::insert").
                insert(id, listener)
        } else {
            match inner.insert(event_hash, EventListenerList::default()) {
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
        where Q:        Ord,
              u32:      ::std::borrow::Borrow< Q > {
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
impl Default for EventListenerWaiting {
    /* ====================================================================== */
    fn default() -> Self { EventListenerWaiting(RwLock::new(Vec::default())) }
}
/* ========================================================================== */
impl EventListenerWaiting {
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
        where Q:        ::std::ops::DerefMut< Target = EventListenerMap > {
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
impl Default for EventListenerRemoving {
    /* ====================================================================== */
    fn default() -> Self { EventListenerRemoving(RwLock::new(Vec::default())) }
}
/* ========================================================================== */
impl EventListenerRemoving {
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
        where Q:        ::std::ops::DerefMut< Target = EventListenerMap > {
        let &EventListenerRemoving(ref inner) = self;
        let mut vec = inner.write().expect("EventLitenerRemoving.apply");
        for &(hash, id) in vec.iter() {
            map.remove(hash, id);
        }
        vec.clear();
    }
}
