// -*- mode:rust; coding:utf-8-unix; -*-

//! event_listener.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2024/04/21

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{collections::BTreeMap, fmt::Debug, sync::RwLock};
// ----------------------------------------------------------------------------
use elicit::aelicit_define;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{event::Event, eventor::Eventor};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum RetOnEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RetOnEvent {
    /// Next
    Next,
    /// Complete
    Complete,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait EventListener
#[aelicit_define(event_listener_aelicit)]
pub trait EventListener: Debug + Sync + Send {
    // ========================================================================
    /// peek_id
    fn peek_id(&self) -> &Uuid;
    // ========================================================================
    /// on_event
    fn on_event(&mut self, event: &Event, eventor: &Eventor) -> RetOnEvent;
}
// ============================================================================
pub use event_listener_aelicit::author as aelicit_author;
pub use event_listener_aelicit::user as aelicit_user;
// ----------------------------------------------------------------------------
use aelicit_author::Aelicit as EventListenerAelicit;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type EventListenerList
pub(crate) type EventListenerList = BTreeMap<Uuid, EventListenerAelicit>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerMap
#[derive(Debug, Default)]
pub(crate) struct EventListenerMap(BTreeMap<u32, EventListenerList>);
// ============================================================================
impl EventListenerMap {
    // ========================================================================
    /// insert
    pub(crate) fn insert(
        &mut self,
        event_hash: u32,
        id: &Uuid,
        listener: EventListenerAelicit,
    ) {
        let _ = self
            .0
            .entry(event_hash)
            .or_default()
            .entry(*id)
            .or_insert(listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(
        &mut self,
        event_hash: u32,
        id: &Uuid,
    ) -> Option<EventListenerAelicit> {
        self.0.get_mut(&event_hash)?.remove(id)
    }
    // ========================================================================
    /// get_mut
    pub(crate) fn get_mut<Q>(
        &mut self,
        key: &Q,
    ) -> Option<&mut EventListenerList>
    where
        Q: ?Sized + Ord,
        u32: std::borrow::Borrow<Q>,
    {
        self.0.get_mut(key)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerWaiting
#[derive(Debug, Default)]
pub(crate) struct EventListenerWaiting(
    RwLock<Vec<(u32, EventListenerAelicit)>>,
);
// ============================================================================
impl EventListenerWaiting {
    // ========================================================================
    /// insert
    pub(crate) fn insert(
        &self,
        event_hash: u32,
        listener: EventListenerAelicit,
    ) {
        self.0
            .write()
            .expect("EventLitenerWaiting::insert")
            .push((event_hash, listener))
    }
    // ========================================================================
    /// shrink_to_fit
    pub(crate) fn shrink_to_fit(&self) {
        self.0
            .write()
            .expect("EventLitenerWaiting::shrink_to_fit")
            .shrink_to_fit()
    }
    // ========================================================================
    /// apply
    #[allow(box_pointers)]
    pub(crate) fn apply(
        &self,
        map: &mut impl std::ops::DerefMut<Target = EventListenerMap>,
    ) {
        let mut vec = self.0.write().expect("EventLitenerWaiting::apply");
        for &(hash, ref listener) in vec.iter() {
            map.insert(
                hash,
                listener
                    .read()
                    .expect("EventListenerWaiting::apply")
                    .peek_id(),
                listener.clone(),
            );
        }
        vec.clear();
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventListenerRemoving
#[derive(Debug, Default)]
pub(crate) struct EventListenerRemoving(RwLock<Vec<(u32, Uuid)>>);
// ============================================================================
impl EventListenerRemoving {
    // ========================================================================
    /// insert
    pub(crate) fn insert(&self, event_hash: u32, id: &Uuid) {
        self.0
            .write()
            .expect("EventLitenerRemoving::insert")
            .push((event_hash, *id))
    }
    // ========================================================================
    /// shrink_to_fit
    pub(crate) fn shrink_to_fit(&self) {
        self.0
            .write()
            .expect("EventLitenerRemoving::shrink_to_fit")
            .shrink_to_fit()
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply(
        &self,
        map: &mut impl std::ops::DerefMut<Target = EventListenerMap>,
    ) {
        let mut vec = self.0.write().expect("EventLitenerRemoving::apply");
        for &(hash, id) in vec.iter() {
            drop(map.remove(hash, &id));
        }
        vec.clear();
    }
}
