// -*- mode:rust; coding:utf-8-unix; -*-

//! mediator.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/21
//  @date 2024/04/22

// ////////////////////////////////////////////////////////////////////////////
// attributes  ================================================================
#![allow(box_pointers)]
// use  =======================================================================
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use crate::{
    event_listener::EventListenerMap,
    event_listener_aelicit_user::Aelicit as EventListenerAelicit,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct MediatorInner
#[derive(Debug, Default)]
pub(crate) struct MediatorInner {
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
        id: Uuid,
        listener: EventListenerAelicit,
    ) {
        let _ = self.retiree.entry(event_hash).or_default().remove(&id);
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
        drop(self.newface.entry(event_hash).or_default().remove(id));
        let _ = self.retiree.entry(event_hash).or_default().insert(*id);
    }
    // ========================================================================
    /// apply
    pub(crate) fn apply(
        &mut self,
        map: &mut impl std::ops::DerefMut<Target = EventListenerMap>,
    ) {
        for (hash, btree) in self.newface.iter_mut() {
            for (id, listener) in btree.iter() {
                map.insert(*hash, id, listener.clone());
            }
            btree.clear();
        }
        self.newface.clear();

        for (hash, bset) in self.retiree.iter_mut() {
            for id in bset.iter() {
                drop(map.remove(*hash, id));
            }
            bset.clear();
        }
        self.retiree.clear();
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
        id: Uuid,
        listener: EventListenerAelicit,
    ) {
        self.0
            .lock()
            .expect("eventor::inner::Mediator::insert")
            .insert(event_hash, id, listener);
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
        map: &mut impl std::ops::DerefMut<Target = EventListenerMap>,
    ) {
        self.0
            .lock()
            .expect("eventor::inner::Mediator::apply")
            .apply(map);
    }
}
