// -*- mode:rust; coding:utf-8-unix; -*-

//! `listener_map.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/23
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use alloc::collections::BTreeMap;
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
use crate::event_listener_aelicit_user::Aelicit as EventListenerAelicit;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
type MapUUIDAelicit = BTreeMap<usize, EventListenerAelicit>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct `ListenerMap`
#[derive(Debug, Default)]
pub(crate) struct ListenerMap(BTreeMap<u32, MapUUIDAelicit>);
// ============================================================================
impl ListenerMap {
    // ========================================================================
    /// insert
    pub(crate) fn insert(
        &mut self,
        hash: u32,
        id: usize,
        listener: EventListenerAelicit,
    ) {
        let _ = self
            .0
            .entry(hash)
            .or_default()
            .entry(id)
            .or_insert(listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&mut self, hash: u32, id: usize) {
        let Some(list) = self.0.get_mut(&hash) else {
            return;
        };
        drop(list.remove(&id));
    }
    // ========================================================================
    /// get
    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&MapUUIDAelicit>
    where
        Q: ?Sized + Ord,
        u32: core::borrow::Borrow<Q>,
    {
        self.0.get(key)
    }
}
