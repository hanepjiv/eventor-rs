// -*- mode:rust; coding:utf-8-unix; -*-

//! event_type.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/07
//  @date 2017/01/03

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use ::std::sync::Arc;
use ::std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use ::hash_combine::hash_combine;
// ----------------------------------------------------------------------------
use super::error::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct EventType
#[derive( Debug, Clone, )]
pub struct EventType {
    /// name
    name:       String,
    /// hash
    hash:       u32,
}
// ============================================================================
/// type EventTypeRef
pub type EventTypeRef   = Arc<EventType>;
// ============================================================================
impl EventType {
    // ========================================================================
    /// new
    pub fn new< 'a >(name: &'a str, hash: u32) -> EventTypeRef {
        Arc::new(EventType {
            name:       name.to_string(),
            hash:       hash,
        })
    }
    // ========================================================================
    /// peek_name
    pub fn peek_name< 'a >(&'a self) -> &'a str { self.name.as_ref() }
    // ========================================================================
    /// peek_hash
    pub fn peek_hash(&self) -> u32 { self.hash }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// EventTypeMap
#[derive( Debug, Default, )]
pub struct EventTypeMap(BTreeMap<u32, EventTypeRef>);
// ============================================================================
impl EventTypeMap {
    // ========================================================================
    // ------------------------------------------------------------------------
    /// check_type
    pub fn check_type(&self, name: &str) -> (u32, Option< EventTypeRef >) {
        let hash = hash_combine(0u32, name.as_ref());
        let &EventTypeMap(ref inner) = self;
        match inner.get(&hash) {
            Some(x)     => (hash, Some(x.clone())),
            None        => (hash, None),
        }
    }
    // ------------------------------------------------------------------------
    /// new_type
    pub fn new_type(&mut self, name: &str) -> Result< EventTypeRef, Error > {
        let (hash, ret) = self.check_type(name);
        match ret {
            Some(r)     => Ok(r),  // already exists
            None        => {
                let event_type = EventType::new(name, hash);
                let &mut EventTypeMap(ref mut inner) = self;
                match inner.insert(hash, event_type.clone()) {
                    Some(_)     => {
                        Err(Error::EventorError(
                            format!("Eventor::new_type: \
                                     Hash value are in conflict. \
                                     Take a different name. \"{}\"", name)))
                    },
                    None        => {
                        info!("Eventor::new_type: {:#x} \"{}\"", hash, name);
                        Ok(event_type)
                    },
                }
            }
        }
    }
    // ------------------------------------------------------------------------
    /// peek_type
    pub fn peek_type(&self, name: &str) -> Option< EventTypeRef > {
        let (_, ret) = self.check_type(name);
        ret
    }
}
