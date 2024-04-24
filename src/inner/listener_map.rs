// -*- mode:rust; coding:utf-8-unix; -*-

//! listener_map.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/23
//  @date 2024/04/24

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeMap;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use crate::event_listener_aelicit_user::Aelicit as EventListenerAelicit;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
type UUIDAelicit = BTreeMap<Uuid, EventListenerAelicit>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ListenerMap
#[derive(Debug, Default)]
pub(crate) struct ListenerMap(BTreeMap<u32, UUIDAelicit>);
// ============================================================================
impl ListenerMap {
    // ========================================================================
    /// insert
    pub(crate) fn insert(
        &mut self,
        hash: u32,
        id: &Uuid,
        listener: EventListenerAelicit,
    ) {
        let _ = self
            .0
            .entry(hash)
            .or_default()
            .entry(*id)
            .or_insert(listener);
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&mut self, hash: u32, id: &Uuid) {
        let Some(list) = self.0.get_mut(&hash) else {
            return;
        };
        drop(list.remove(id));
    }
    // ========================================================================
    /// get
    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&UUIDAelicit>
    where
        Q: ?Sized + Ord,
        u32: std::borrow::Borrow<Q>,
    {
        self.0.get(key)
    }
}
