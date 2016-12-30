// -*- mode:rust; coding:utf-8-unix; -*-

//! event_listener.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2016/12/26

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use ::std::fmt::{ Debug, };
use ::std::collections::{ BTreeMap, };
use ::std::sync::{ RwLock, };
// ----------------------------------------------------------------------------
use ::libc::uintptr_t;
// ----------------------------------------------------------------------------
use super::event::{ Event, };
use super::eventor::{ EventorAelicit, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
aelicit_define!(aelicit_t_event_listener, TEventListener);
// ----------------------------------------------------------------------------
pub use self::aelicit_t_event_listener::AelicitError
    as EventListenerAelicitError;
pub use self::aelicit_t_event_listener::AelicitResult
    as EventListenerAelicitResult;
pub use self::aelicit_t_event_listener::Aelicit
    as EventListenerAelicit;
pub use self::aelicit_t_event_listener::EnableAelicitFromSelf
    as EventListenerEnableAelicitFromSelf;
pub use self::aelicit_t_event_listener::EnableAelicitFromSelfField
    as EventListenerEnableAelicitFromSelfField;
// ============================================================================
/// trait TEventListener
pub trait TEventListener: Debug + EventListenerEnableAelicitFromSelf {
    // ========================================================================
    /// peek_id
    fn peek_id(&self) -> uintptr_t;
    // ========================================================================
    /// on_event
    fn on_event(&mut self, event: &Event, eventor: &EventorAelicit) -> bool;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type EventListenerList
pub type EventListenerList = BTreeMap< uintptr_t, EventListenerAelicit >;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerMap
#[derive( Debug, Default, )]
pub struct EventListenerMap(BTreeMap< u32, EventListenerList >);
// ============================================================================
impl EventListenerMap {
    // ========================================================================
    /// insert
    pub fn insert(&mut self,
                  event_hash: u32, id: uintptr_t,
                  listener: EventListenerAelicit)
                  -> Option< EventListenerAelicit > {
        let &mut EventListenerMap(ref mut inner) = self;
        if inner.contains_key(&event_hash) {
            inner.get_mut(&event_hash).expect("EventListenerMap::insert").
                insert(id, listener)
        } else {
            match inner.insert(event_hash, EventListenerList::default()) {
                None    => inner.get_mut(&event_hash).
                    expect("EventListenerMap::insert").
                    insert(id, listener),
                Some(ref mut list) => list.insert(id, listener),
            }
        }
    }
    // ========================================================================
    /// remove
    pub fn remove(&mut self, event_hash: u32, id: uintptr_t)
                  -> Option< EventListenerAelicit > {
        let &mut EventListenerMap(ref mut inner) = self;
        if inner.contains_key(&event_hash) {
            inner.get_mut(&event_hash).expect("EventListenerMap::remove").
                remove(&id)
        } else {
            None
        }
    }
    // ========================================================================
    /// get_mut
    pub fn get_mut< Q: ?Sized >(&mut self, key: &Q)
                                -> Option< &mut EventListenerList >
        where Q:        Ord,
              u32:      ::std::borrow::Borrow< Q > {
        let &mut EventListenerMap(ref mut inner) = self;
        inner.get_mut(key)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerWaiting
#[derive( Debug, )]
pub struct EventListenerWaiting(RwLock< Vec< (u32, EventListenerAelicit) > >);
// ============================================================================
impl Default for EventListenerWaiting {
    // ========================================================================
    fn default() -> Self { EventListenerWaiting(RwLock::new(Vec::default())) }
}
// ============================================================================
impl EventListenerWaiting {
    // ========================================================================
    /// insert
    pub fn insert(&self, event_hash: u32, listener: EventListenerAelicit) {
        let &EventListenerWaiting(ref inner) = self;
        inner.write().expect("EventLitenerWaiting.insert").
            push((event_hash, listener))
    }
    // ========================================================================
    /// shrink_to_fit
    pub fn shrink_to_fit(&self) -> () {
        let &EventListenerWaiting(ref inner) = self;
        inner.write().expect("EventLitenerWaiting.shrink_to_fit").
            shrink_to_fit()
    }
    // ========================================================================
    /// apply
    pub fn apply< Q >(&self, map: &mut Q)
        where Q:        ::std::ops::DerefMut< Target = EventListenerMap > {
        let &EventListenerWaiting(ref inner) = self;
        let mut vec = inner.write().expect("EventLitenerWaiting.apply");
        for &(hash, ref listener) in vec.iter() {
            let id = listener.read().expect("EventListenerWaiting::apply").
                peek_id();
            let _ = map.insert(hash, id, listener.clone());
        }
        vec.clear();
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerRemoving
#[derive( Debug, )]
pub struct EventListenerRemoving(RwLock< Vec< (u32, uintptr_t) > >);
// ============================================================================
impl Default for EventListenerRemoving {
    // ========================================================================
    fn default() -> Self { EventListenerRemoving(RwLock::new(Vec::default())) }
}
// ============================================================================
impl EventListenerRemoving {
    // ========================================================================
    /// insert
    pub fn insert(&self, event_hash: u32, id: uintptr_t) -> () {
        let &EventListenerRemoving(ref inner) = self;
        inner.write().expect("EventLitenerRemoving.insert").
            push((event_hash, id))
    }
    /*
    // ========================================================================
    /// contains
    pub fn contains(&self, x: &(u32, uintptr_t)) -> bool {
        let &EventListenerRemoving(ref inner) = self;
        inner.read().expect("EventLitenerRemoving.contains").
            contains(x)
    }
     */
    // ========================================================================
    /// shrink_to_fit
    pub fn shrink_to_fit(&self) -> () {
        let &EventListenerRemoving(ref inner) = self;
        inner.write().expect("EventLitenerRemoving.shrink_to_fit").
            shrink_to_fit()
    }
    // ========================================================================
    /// apply
    pub fn apply< Q >(&self, map: &mut Q)
        where Q:        ::std::ops::DerefMut< Target = EventListenerMap > {
        let &EventListenerRemoving(ref inner) = self;
        let mut vec = inner.write().expect("EventLitenerRemoving.apply");
        for &(hash, id) in vec.iter() {
            let _ = map.remove(hash, id);
        }
        vec.clear();
    }
}
