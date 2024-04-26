// -*- mode:rust; coding:utf-8-unix; -*-

//! mediator.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/21
//  @date 2024/04/26

// ////////////////////////////////////////////////////////////////////////////
// attributes  ================================================================
#![allow(box_pointers)]
// use  =======================================================================
use parking_lot::Mutex;
use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use crate::event_listener_aelicit_user::Aelicit as EventListenerAelicit;
// ----------------------------------------------------------------------------
use super::ListenerMap;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct MediatorInner
#[derive(Debug, Default)]
struct MediatorInner {
    newface: BTreeMap<u32, BTreeMap<Uuid, EventListenerAelicit>>,
    retiree: BTreeMap<u32, BTreeSet<Uuid>>,
}
// ============================================================================
impl MediatorInner {
    // ========================================================================
    pub(crate) fn insert(
        &mut self,
        hash: u32,
        listener: EventListenerAelicit,
    ) {
        #[cfg(feature = "elicit-parking_lot")]
        let id = *listener.read().peek_id();

        #[cfg(not(any(feature = "elicit-parking_lot"),))]
        let id = *listener.read().expect("Eventor::insert_listener").peek_id();

        if let Entry::Occupied(mut x) = self.retiree.entry(hash) {
            let _ = x.get_mut().remove(&id);
        }
        let _ = self
            .newface
            .entry(hash)
            .or_default()
            .entry(id)
            .or_insert(listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&mut self, hash: u32, id: &Uuid) {
        if let Entry::Occupied(mut x) = self.newface.entry(hash) {
            drop(x.get_mut().remove(id));
        }
        let _ = self.retiree.entry(hash).or_default().insert(*id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply<T>(&mut self, mut map: T)
    where
        T: std::ops::DerefMut<Target = ListenerMap>,
    {
        for (hash, tree) in self.newface.iter_mut() {
            for (id, listener) in tree.iter() {
                map.insert(*hash, id, listener.clone());
            }
            tree.clear();
        }
        for (hash, set) in self.retiree.iter_mut() {
            for id in set.iter() {
                map.remove(*hash, id);
            }
            set.clear();
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Mediator
#[derive(Debug, Default)]
pub(crate) struct Mediator(Mutex<MediatorInner>);
// ============================================================================
impl Mediator {
    // ========================================================================
    /// insert
    pub(crate) fn insert(&self, hash: u32, listener: EventListenerAelicit) {
        self.0.lock().insert(hash, listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&self, hash: u32, id: &Uuid) {
        self.0.lock().remove(hash, id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply<T>(&self, map: T)
    where
        T: std::ops::DerefMut<Target = ListenerMap>,
    {
        self.0.lock().apply(map);
    }
}
