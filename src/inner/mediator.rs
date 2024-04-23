// -*- mode:rust; coding:utf-8-unix; -*-

//! mediator.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/21
//  @date 2024/04/23

// ////////////////////////////////////////////////////////////////////////////
// attributes  ================================================================
#![allow(box_pointers)]
// use  =======================================================================
use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    sync::Mutex,
};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use crate::event_listener_aelicit_user::Aelicit as EventListenerAelicit;
// ----------------------------------------------------------------------------
use super::ListenerMap;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
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
        let id = listener.read().expect("eventor::insert").peek_id().clone();
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
            drop(x.get_mut().remove(&id));
        }
        let _ = self.retiree.entry(event_hash).or_default().insert(*id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply(
        &mut self,
        map: &mut impl std::ops::DerefMut<Target = ListenerMap>,
    ) {
        for (hash, btree) in self.newface.iter_mut() {
            for (id, listener) in btree.iter() {
                map.insert(*hash, id, listener.clone());
            }
            btree.clear();
        }

        for (hash, bset) in self.retiree.iter_mut() {
            for id in bset.iter() {
                drop(map.remove(*hash, id));
            }
            bset.clear();
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
        self.0
            .lock()
            .expect("eventor::inner::Mediator::insert")
            .insert(event_hash, listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&self, event_hash: u32, id: &Uuid) {
        self.0
            .lock()
            .expect("eventor::inner::Mediator::remove")
            .remove(event_hash, id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply(
        &self,
        map: &mut impl std::ops::DerefMut<Target = ListenerMap>,
    ) {
        self.0
            .lock()
            .expect("eventor::inner::Mediator::apply")
            .apply(map);
    }
}
