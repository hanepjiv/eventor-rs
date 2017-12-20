// -*- mode:rust; coding:utf-8-unix; -*-

//! event_listener.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2017/05/29

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::fmt::Debug;
use std::collections::BTreeMap;
use std::sync::RwLock;
// ----------------------------------------------------------------------------
use libc::uintptr_t;
// ----------------------------------------------------------------------------
use super::event::Event;
use super::eventor::EventorAelicit;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
aelicit_define!(aelicit_t_event_listener, TEventListener);
// ----------------------------------------------------------------------------
pub use self::aelicit_t_event_listener::Aelicit as EventListenerAelicit;
pub use self::aelicit_t_event_listener::EnableAelicitFromSelf;
pub use self::EnableAelicitFromSelf as EventListenerEAFS;
pub use self::aelicit_t_event_listener::EnableAelicitFromSelfField;
pub use self::EnableAelicitFromSelfField as EventListenerEAFSField;
// ============================================================================
/// trait TEventListener
pub trait TEventListener: Debug + EventListenerEAFS {
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
pub type EventListenerList = BTreeMap<uintptr_t, EventListenerAelicit>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerMap
#[derive(Debug, Default)]
pub struct EventListenerMap(BTreeMap<u32, EventListenerList>);
// ============================================================================
impl EventListenerMap {
    // ========================================================================
    /// insert
    pub fn insert(
        &mut self,
        event_hash: u32,
        id: uintptr_t,
        listener: EventListenerAelicit,
    ) -> Option<EventListenerAelicit> {
        if self.0.contains_key(&event_hash) {
            self.0
                .get_mut(&event_hash)
                .expect("EventListenerMap::insert")
                .insert(id, listener)
        } else {
            match self.0.insert(event_hash, EventListenerList::default()) {
                None => self.0
                    .get_mut(&event_hash)
                    .expect("EventListenerMap::insert")
                    .insert(id, listener),
                Some(ref mut list) => list.insert(id, listener),
            }
        }
    }
    // ========================================================================
    /// remove
    pub fn remove(
        &mut self,
        event_hash: u32,
        id: uintptr_t,
    ) -> Option<EventListenerAelicit> {
        if self.0.contains_key(&event_hash) {
            self.0
                .get_mut(&event_hash)
                .expect("EventListenerMap::remove")
                .remove(&id)
        } else {
            None
        }
    }
    // ========================================================================
    /// get_mut
    pub fn get_mut<Q: ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<&mut EventListenerList>
    where
        Q: Ord,
        u32: ::std::borrow::Borrow<Q>,
    {
        self.0.get_mut(key)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerWaiting
#[derive(Debug)]
pub struct EventListenerWaiting(RwLock<Vec<(u32, EventListenerAelicit)>>);
// ============================================================================
impl Default for EventListenerWaiting {
    // ========================================================================
    fn default() -> Self {
        EventListenerWaiting(RwLock::new(Vec::default()))
    }
}
// ============================================================================
impl EventListenerWaiting {
    // ========================================================================
    /// insert
    pub fn insert(&self, event_hash: u32, listener: EventListenerAelicit) {
        self.0
            .write()
            .expect("EventLitenerWaiting.insert")
            .push((event_hash, listener))
    }
    // ========================================================================
    /// shrink_to_fit
    pub fn shrink_to_fit(&self) -> () {
        self.0
            .write()
            .expect("EventLitenerWaiting.shrink_to_fit")
            .shrink_to_fit()
    }
    // ========================================================================
    /// apply
    pub fn apply<Q>(&self, map: &mut Q)
    where
        Q: ::std::ops::DerefMut<Target = EventListenerMap>,
    {
        let mut vec = self.0.write().expect("EventLitenerWaiting.apply");
        for &(hash, ref listener) in vec.iter() {
            let id = listener
                .read()
                .expect("EventListenerWaiting::apply")
                .peek_id();
            let _ = map.insert(hash, id, listener.clone());
        }
        vec.clear();
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerRemoving
#[derive(Debug)]
pub struct EventListenerRemoving(RwLock<Vec<(u32, uintptr_t)>>);
// ============================================================================
impl Default for EventListenerRemoving {
    // ========================================================================
    fn default() -> Self {
        EventListenerRemoving(RwLock::new(Vec::default()))
    }
}
// ============================================================================
impl EventListenerRemoving {
    // ========================================================================
    /// insert
    pub fn insert(&self, event_hash: u32, id: uintptr_t) -> () {
        self.0
            .write()
            .expect("EventLitenerRemoving.insert")
            .push((event_hash, id))
    }
    /*
    // ========================================================================
    /// contains
    pub fn contains(&self, x: &(u32, uintptr_t)) -> bool {
        self.0.read().expect("EventLitenerRemoving.contains").
            contains(x)
    }
     */
    // ========================================================================
    /// shrink_to_fit
    pub fn shrink_to_fit(&self) -> () {
        self.0
            .write()
            .expect("EventLitenerRemoving.shrink_to_fit")
            .shrink_to_fit()
    }
    // ========================================================================
    /// apply
    pub fn apply<Q>(&self, map: &mut Q)
    where
        Q: ::std::ops::DerefMut<Target = EventListenerMap>,
    {
        let mut vec = self.0.write().expect("EventLitenerRemoving.apply");
        for &(hash, id) in vec.iter() {
            let _ = map.remove(hash, id);
        }
        vec.clear();
    }
}
