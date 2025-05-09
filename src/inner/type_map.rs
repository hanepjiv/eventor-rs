// -*- mode:rust; coding:utf-8-unix; -*-

//! `type_map.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/25
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use alloc::collections::BTreeMap;
// ----------------------------------------------------------------------------
use hash_combine::hash_combine;
use log::info;
// ----------------------------------------------------------------------------
use crate::{Error, EventType, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// `TypeMap`
#[derive(Debug, Default)]
pub(crate) struct TypeMap(BTreeMap<u32, EventType>);
// ============================================================================
impl TypeMap {
    // ========================================================================
    // ------------------------------------------------------------------------
    /// `check_type`
    fn check_type<T>(&self, name: T) -> (u32, Option<EventType>)
    where
        T: AsRef<[u8]>,
    {
        let hash = hash_combine(0_u32, name.as_ref());
        self.0
            .get(&hash)
            .map_or((hash, None), |x| (hash, Some(x.clone())))
    }
    // ------------------------------------------------------------------------
    /// `new_type`
    pub(crate) fn new_type<T>(&mut self, name: T) -> Result<EventType>
    where
        T: AsRef<str> + core::fmt::Display,
    {
        let l_name = name.as_ref().to_lowercase();
        let (hash, ret) = self.check_type(l_name.as_str());
        if let Some(r) = ret {
            // already exists
            if r.peek_name() == l_name {
                Ok(r)
            } else {
                // Hash value are in conflict. Take a different name.
                Err(Error::HashConflict {
                    already: r.peek_name().to_owned(),
                    new: l_name,
                })
            }
        } else {
            let event_type = EventType::new(l_name.as_str(), hash);
            if self.0.insert(hash, event_type.clone()).is_some() {
                Err(Error::Eventor(format!(
                    "Eventor::new_type: \
                     Unknown insert error. \"{name}\""
                )))
            } else {
                info!("Eventor::new_type: \"{l_name}\" = {hash:#x}");
                Ok(event_type)
            }
        }
    }
    // ------------------------------------------------------------------------
    /// `peek_type`
    pub(crate) fn peek_type<T>(&self, name: T) -> Option<EventType>
    where
        T: AsRef<str>,
    {
        let (_, ret) = self.check_type(name.as_ref().to_lowercase().as_str());
        ret
    }
}
