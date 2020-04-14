// -*- mode:rust; coding:utf-8-unix; -*-

//! event_type.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2020/04/14

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{collections::BTreeMap, sync::Arc};
// ----------------------------------------------------------------------------
use hash_combine::hash_combine;
use log::info;
// ----------------------------------------------------------------------------
use super::error::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventType
#[derive(Debug, Clone)]
pub struct EventType {
    /// name
    name: String,
    /// hash
    hash: u32,
}
// ============================================================================
/// type EventTypeRef
pub type EventTypeRef = Arc<EventType>;
// ============================================================================
impl EventType {
    // ========================================================================
    /// new
    fn new(name: &str, hash: u32) -> Self {
        Self {
            name: name.to_string(),
            hash,
        }
    }
    // ========================================================================
    /// peek_name
    pub fn peek_name(&self) -> &str {
        self.name.as_ref()
    }
    // ========================================================================
    /// peek_hash
    pub fn peek_hash(&self) -> u32 {
        self.hash
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// EventTypeMap
#[derive(Debug, Default)]
pub(crate) struct EventTypeMap(BTreeMap<u32, EventTypeRef>);
// ============================================================================
impl EventTypeMap {
    // ========================================================================
    // ------------------------------------------------------------------------
    /// check_type
    fn check_type(&self, name: &str) -> (u32, Option<EventTypeRef>) {
        let hash = hash_combine(0u32, name.as_ref());
        match self.0.get(&hash) {
            Some(x) => (hash, Some(x.clone())),
            None => (hash, None),
        }
    }
    // ------------------------------------------------------------------------
    /// new_type
    pub(crate) fn new_type(
        &mut self,
        name: &str,
    ) -> Result<EventTypeRef, Error> {
        let l_name = name.to_lowercase();
        let (hash, ret) = self.check_type(l_name.as_str());
        match ret {
            Some(r) => {
                // already exists
                if r.peek_name() == l_name {
                    Ok(r)
                } else {
                    Err(Error::Eventor(format!(
                        "Eventor::new_type: \
                         Hash value are in conflict. \
                         Take a different name. \
                         already:\"{}\"/ new:\"{}\"",
                        r.peek_name(),
                        l_name
                    )))
                }
            }
            None => {
                let event_type =
                    Arc::new(EventType::new(l_name.as_str(), hash));
                match self.0.insert(hash, event_type.clone()) {
                    Some(_) => Err(Error::Eventor(format!(
                        "Eventor::new_type: \
                         Unknown insert error. \"{}\"",
                        name
                    ))),
                    None => {
                        info!(
                            "Eventor::new_type: \"{}\" = {:#x}",
                            l_name, hash
                        );
                        Ok(event_type)
                    }
                }
            }
        }
    }
    // ------------------------------------------------------------------------
    /// peek_type
    pub(crate) fn peek_type(&self, name: &str) -> Option<EventTypeRef> {
        let l_name = name.to_lowercase();
        let (_, ret) = self.check_type(l_name.as_str());
        ret
    }
}
