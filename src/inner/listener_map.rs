// -*- mode:rust; coding:utf-8-unix; -*-

//! listener_map.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/23
//  @date 2024/04/24

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use crate::event_listener_aelicit_user::Aelicit as EventListenerAelicit;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
type MapUUIDAelicit = Arc<RwLock<BTreeMap<Uuid, EventListenerAelicit>>>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ListenerMap
#[derive(Debug, Default)]
pub(crate) struct ListenerMap(BTreeMap<u32, MapUUIDAelicit>);
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
        let list = self.0.entry(hash).or_default();
        'outer: loop {
            if let Some(mut x) = list.try_write() {
                let _ = x.entry(*id).or_insert(listener);
                break 'outer;
            }
            std::thread::yield_now();
            std::thread::sleep(std::time::Duration::from_millis(200))
        }
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&self, hash: u32, id: &Uuid) {
        let Some(list) = self.0.get(&hash) else {
            return;
        };
        'outer: loop {
            if let Some(mut x) = list.try_write() {
                drop(x.remove(id));
                break 'outer;
            }
            std::thread::yield_now();
            std::thread::sleep(std::time::Duration::from_millis(200))
        }
    }
    // ========================================================================
    /// get
    pub(crate) fn get<Q>(&self, key: &Q) -> Option<MapUUIDAelicit>
    where
        Q: ?Sized + Ord,
        u32: std::borrow::Borrow<Q>,
    {
        self.0.get(key).cloned()
    }
}
