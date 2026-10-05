// -*- coding:utf-8-unix; -*-

//! mediator.rs.

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/21
//  @date 2026/10/04

// ////////////////////////////////////////////////////////////////////////////
// attributes  ================================================================
// use  =======================================================================
use crate::inner::sync::Mutex;
use alloc::collections::{BTreeMap, BTreeSet, btree_map::Entry};
// ----------------------------------------------------------------------------
use crate::event_listener_aelicit_user::Aelicit as EventListenerAelicit;
// ----------------------------------------------------------------------------
use super::ListenerMap;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct `MediatorInner`.
#[derive(Debug, Default)]
struct MediatorInner {
    /// newface.
    newface: BTreeMap<u32, BTreeMap<usize, EventListenerAelicit>>,
    /// retiree.
    retiree: BTreeMap<u32, BTreeSet<usize>>,
}
// ============================================================================
impl MediatorInner {
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// fn `get_id`.
    fn get_id(listener: &EventListenerAelicit) -> usize {
        listener.read().usizeptr()
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// fn `get_id`.
    #[expect(clippy::expect_used, reason = "checked")]
    fn get_id(listener: &EventListenerAelicit) -> usize {
        listener
            .read()
            .expect("Eventor::insert_listener")
            .usizeptr()
    }
    // ========================================================================
    /// fn insert.
    pub(crate) fn insert(
        &mut self,
        hash: u32,
        listener: EventListenerAelicit,
    ) {
        let id = Self::get_id(&listener);

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
    /// remove.
    pub(crate) fn remove(&mut self, hash: u32, id: usize) {
        if let Entry::Occupied(mut x) = self.newface.entry(hash) {
            drop(x.get_mut().remove(&id));
        }
        let _ = self.retiree.entry(hash).or_default().insert(id);
    }
    // ========================================================================
    /// apply.
    pub(crate) fn apply<T>(&mut self, mut map: T)
    where
        T: core::ops::DerefMut<Target = ListenerMap>,
    {
        for (hash, tree) in &mut self.newface {
            for (id, listener) in tree.iter() {
                map.insert(*hash, *id, listener.clone());
            }
            tree.clear();
        }
        for (hash, set) in &mut self.retiree {
            for id in set.iter() {
                map.remove(*hash, *id);
            }
            set.clear();
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Mediator.
#[derive(Debug, Default)]
pub(crate) struct Mediator(Mutex<MediatorInner>);
// ============================================================================
impl Mediator {
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// fn `insert`.
    pub(crate) fn insert(&self, hash: u32, listener: EventListenerAelicit) {
        self.0.lock().insert(hash, listener);
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// fn `insert`.
    #[expect(clippy::expect_used, reason = "checked")]
    pub(crate) fn insert(&self, hash: u32, listener: EventListenerAelicit) {
        self.0
            .lock()
            .expect("Mediator::insert")
            .insert(hash, listener);
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// remove.
    pub(crate) fn remove(&self, hash: u32, id: usize) {
        self.0.lock().remove(hash, id);
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    /// remove.
    #[expect(clippy::expect_used, reason = "checked")]
    pub(crate) fn remove(&self, hash: u32, id: usize) {
        self.0.lock().expect("Mediator::remove").remove(hash, id);
    }
    // ========================================================================
    #[cfg(feature = "parking_lot")]
    /// fn apply.
    pub(crate) fn apply<T>(&self, map: T)
    where
        T: core::ops::DerefMut<Target = ListenerMap>,
    {
        self.0.lock().apply(map);
    }

    #[cfg(not(any(feature = "parking_lot"),))]
    #[expect(clippy::expect_used, reason = "checked")]
    /// fn apply.
    pub(crate) fn apply<T>(&self, map: T)
    where
        T: core::ops::DerefMut<Target = ListenerMap>,
    {
        self.0.lock().expect("Mediator::apply").apply(map);
    }
}
