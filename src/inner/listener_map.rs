// -*- mode:rust; coding:utf-8-unix; -*-

//! listener_map.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/23
//  @date 2024/04/23

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock, TryLockError},
};
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
    ) -> bool {
        let x = self.0.entry(hash).or_default();
        match x.try_write() {
            Ok(mut y) => {
                let _ = y.entry(*id).or_insert(listener);
                return true;
            }
            Err(e) => match e {
                TryLockError::WouldBlock => {
                    return false;
                }
                _ => panic!("listener insert"),
            },
        }
    }
    // ========================================================================
    /// remove
    pub(crate) fn remove(&self, hash: u32, id: &Uuid) -> bool {
        let Some(x) = self.0.get(&hash) else {
            return true;
        };
        match x.try_write() {
            Ok(mut y) => {
                drop(y.remove(id));
                return true;
            }
            Err(e) => match e {
                TryLockError::WouldBlock => {
                    return false;
                }
                _ => panic!("listener remove"),
            },
        }
    }
    // ========================================================================
    /// get
    pub(crate) fn get<Q>(&self, key: &Q) -> Option<MapUUIDAelicit>
    where
        Q: ?Sized + Ord,
        u32: std::borrow::Borrow<Q>,
    {
        self.0.get(key).map(MapUUIDAelicit::clone)
    }
}
