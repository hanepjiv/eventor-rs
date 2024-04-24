// -*- mode:rust; coding:utf-8-unix; -*-

//! mediator.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/21
//  @date 2024/04/24

// ////////////////////////////////////////////////////////////////////////////
// attributes  ================================================================
#![allow(box_pointers)]
// use  =======================================================================
use parking_lot::{Mutex, RwLock};
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
    /// insert
    pub(crate) fn insert(
        &mut self,
        event_hash: u32,
        listener: EventListenerAelicit,
    ) {
        let id = *listener
            .read()
            .expect("Eventor::insert: listener")
            .peek_id();
        if let Entry::Occupied(mut x) = self.retiree.entry(event_hash) {
            let _ = x.get_mut().remove(&id);
        }
        let _ = self
            .newface
            .entry(event_hash)
            .or_default()
            .entry(id)
            .or_insert(listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&mut self, event_hash: u32, id: &Uuid) {
        if let Entry::Occupied(mut x) = self.newface.entry(event_hash) {
            drop(x.get_mut().remove(id));
        }
        let _ = self.retiree.entry(event_hash).or_default().insert(*id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply(&mut self, map: &RwLock<ListenerMap>) {
        for (hash, tree) in self.newface.iter_mut() {
            for (id, listener) in tree.iter() {
                'outer: loop {
                    if let Some(mut m) = map.try_write() {
                        m.insert(*hash, id, listener.clone());
                        break 'outer;
                    }
                    std::thread::yield_now();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            tree.clear();
        }
        for (hash, set) in self.retiree.iter_mut() {
            for id in set.iter() {
                'outer: loop {
                    if let Some(m) = map.try_read() {
                        m.remove(*hash, id);
                        break 'outer;
                    }
                    std::thread::yield_now();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
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
    pub(crate) fn insert(
        &self,
        event_hash: u32,
        listener: EventListenerAelicit,
    ) {
        self.0.lock().insert(event_hash, listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&self, event_hash: u32, id: &Uuid) {
        self.0.lock().remove(event_hash, id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply(&self, map: &RwLock<ListenerMap>) {
        self.0.lock().apply(map);
    }
}
